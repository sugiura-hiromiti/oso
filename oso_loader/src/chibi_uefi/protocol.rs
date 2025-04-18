use super::Handle;
use super::image_handle;
use super::table::boot_services;
use crate::Rslt;
use crate::guid;
use crate::raw::protocol::TextOutputProtocol;
use crate::raw::service::BootServices;
use crate::raw::types::Guid;
use crate::raw::types::UnsafeHandle;
use core::assert_matches::assert_matches;
use core::ffi::c_void;
use core::ptr;
use core::ptr::NonNull;

pub trait Protocol {
	const GUID: Guid;
}

impl Protocol for TextOutputProtocol {
	const GUID: Guid = guid!("387477c1-69c7-11d2-8e3900a0c969723b");
}

impl BootServices {
	pub unsafe fn locate_handle_buffer(&self, ty: HandleSearchType,) -> Rslt<&mut [Handle],> {
		let (ty, guid, key,) = match ty {
			HandleSearchType::AllHandles => (0, ptr::null(), ptr::null(),),
			HandleSearchType::ByRegisterNotify(protocol_search_key,) => {
				(1, ptr::null(), protocol_search_key.0.as_ptr().cast_const(),)
			},
			HandleSearchType::ByProtocol(guid,) => (2, ptr::from_ref(guid,), ptr::null(),),
		};

		let mut num_handles: usize = 0;
		let mut buffer: *mut UnsafeHandle = ptr::null_mut();

		let handlers =
			unsafe { (self.locate_handle_buffer)(ty, guid, key, &mut num_handles, &mut buffer,) };
		panic!();
		//.ok_or_with(|| (num_handles, buffer,),)?;
		let handlers = (num_handles, buffer,);

		let handler_range =
			unsafe { core::slice::from_raw_parts_mut::<Handle,>(handlers.1.cast(), handlers.0,) };
		Ok(handler_range,)
	}

	/// return guid to search protocol
	pub fn protocol_for<P: Protocol,>(&self,) -> HandleSearchType {
		HandleSearchType::ByProtocol(&P::GUID,)
	}

	pub unsafe fn handle_for_protocol<P: Protocol,>(&self,) -> Rslt<&mut [Handle],> {
		let search_ty = self.protocol_for::<P>();
		unsafe { self.locate_handle_buffer(search_ty,) }
	}

	/// # Parms
	///
	/// - handle
	/// 開きたいプロトコルのインターフェースハンドラ
	///
	/// - agent
	/// プロトコルを開くためのエージェントのハンドル
	/// agentがUEFI Driver
	/// Modelに従っている場合この引数はEFI_DRIVER_BINDING_PROTOCOLのハンドラということになる
	/// EFI_DRIVER_BINDING_PROTOCOLのインスタンスはUEFIドライバによって生成される
	/// UEFIアプリケーションの場合、これはイメージハンドラにあたる
	///
	/// - controller
	/// agentがUEFI Driver Modelに従っている場合この引数はagentのハンドラとなる
	/// そうでない場合はこの引数はnullでも良い
	///
	/// - attr
	/// プロトコルをどの様に開くかを指定する
	/// 詳細は引数の型定義参照
	pub unsafe fn open_protocol<P: Protocol,>(
		&self,
		necessity: OpenProtoNecessity,
		attr: OpenProtoAttr,
	) -> Rslt<ProtocolInterface<P,>,> {
		let mut interface = ptr::null_mut();
		unsafe {
			(self.open_protocol)(
				necessity.handle.as_ptr(),
				&P::GUID,
				&mut interface,
				necessity.agent.as_ptr(),
				Handle::opt_to_ptr(necessity.controller.clone(),),
				attr.0,
			)
			.ok_or_with(|| ProtocolInterface {
				interface: if interface.is_null() {
					None
				} else {
					Some(NonNull::new_unchecked(interface.cast(),),)
				},
				handles:   necessity,
			},)
		}
	}

	pub fn open_protocol_exclusive<P: Protocol,>(
		&self,
		handle: Handle,
	) -> Rslt<ProtocolInterface<P,>,> {
		let necessity = OpenProtoNecessity::for_app(handle,);
		unsafe { self.open_protocol(necessity, OpenProtoAttr::EXCULSIVE,) }
	}
}

#[derive(Debug,)]
pub enum HandleSearchType<'g,> {
	/// return all handles present on the system
	AllHandles,
	/// return all handles that implement a protocol when an intereface for that protocol is
	/// (re)installed
	ByRegisterNotify(ProtocolSearchKey,),
	/// returns all handles supporting a certain protocol, specified by its guid
	ByProtocol(&'g Guid,),
}

#[derive(Clone, Debug,)]
#[repr(transparent)]
pub struct ProtocolSearchKey(pub(crate) NonNull<c_void,>,);

#[repr(transparent)]
pub struct OpenProtoAttr(u32,);

impl OpenProtoAttr {
	/// busドライバに使用される
	/// このフラグが立っている場合、再帰的にchild controllerに接続しようとする
	pub const BY_CHILD_CONTROLLER: Self = Self(0x8,);
	/// ドライバがプロトコルインターフェースのアクセスを得る為に使用される
	/// このフラグが立っている場合、プロトコルインターフェースが削除、
	/// 再インストールされる際にドライバが停止する
	/// 一度プロトコルインターフェースがドライバを用いて、そしてこのフラグをオンにして開かれた場合、
	/// 他のドライバはこのフラグを立てて同じプロトコルインターフェースを開くことが許可されない
	pub const BY_DRIVER: Self = Self(0x10,);
	/// boot_services.handle_protocolで使用される
	pub const BY_HANDLE_PROTOCOL: Self = Self(0x1,);
	/// UEFIアプリケーションがプロトコルインターフェースの排他的アクセスを得る際に使用される
	/// BY_DRIVERフラグでプロトコルインターフェースを開いているドライバがある場合、
	/// ドライバを停止する試みがなされる
	pub const EXCULSIVE: Self = Self(0x20,);
	pub const GET_PROTOCOL: Self = Self(0x2,);
	pub const TEST_PROTOCOL: Self = Self(0x4,);
}

/// protocol interface representation which is designed as safe(automatically closed on drop)
pub struct ProtocolInterface<P: Protocol,> {
	interface: Option<NonNull<P,>,>,
	handles:   OpenProtoNecessity,
}

impl<P: Protocol,> ProtocolInterface<P,> {
	pub fn interface(&self,) -> NonNull<P,> {
		self.interface.unwrap()
	}
}

impl<P: Protocol,> Drop for ProtocolInterface<P,> {
	fn drop(&mut self,) {
		let bt = boot_services();
		let rslt = unsafe {
			(bt.close_protocol)(
				self.handles.handle_ptr(),
				&P::GUID,
				self.handles.agent_ptr(),
				self.handles.controller_ptr(),
			)
		}
		.ok_or();

		assert_matches!(rslt, Err(_));
	}
}

pub struct OpenProtoNecessity {
	handle:     Handle,
	agent:      Handle,
	controller: Option<Handle,>,
}

impl OpenProtoNecessity {
	pub fn for_app(handle: Handle,) -> Self {
		let agent = image_handle();
		Self { handle, agent, controller: None, }
	}

	pub fn handle_ptr(&self,) -> UnsafeHandle {
		self.handle.as_ptr()
	}

	pub fn agent_ptr(&self,) -> UnsafeHandle {
		self.agent.as_ptr()
	}

	/// may null
	pub fn controller_ptr(&self,) -> UnsafeHandle {
		Handle::opt_to_ptr(self.controller.clone(),)
	}
}
