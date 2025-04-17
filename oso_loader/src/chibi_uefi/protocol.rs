use crate::Rslt;
use crate::guid;
use crate::raw::protocol::LocateSearchType;
use crate::raw::protocol::TextOutputProtocol;
use crate::raw::service::BootServices;
use crate::raw::types::Guid;
use alloc::vec::Vec;
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
	pub fn locate_handle_buffer(&self, ty: HandleSearchType,) -> Rslt<(usize, *mut *mut c_void,),> {
		let (ty, guid, key,) = match ty {
			HandleSearchType::AllHandles => {
				(LocateSearchType::AllHandles, ptr::null(), ptr::null(),)
			},
			HandleSearchType::ByRegisterNotify(protocol_search_key,) => (
				LocateSearchType::ByRegisterNotify,
				ptr::null(),
				protocol_search_key.0.as_ptr().cast_const(),
			),
			HandleSearchType::ByProtocol(guid,) => {
				(LocateSearchType::ByProtocol, ptr::from_ref(guid,), ptr::null(),)
			},
		};

		let mut num_handles: usize = 0;
		let mut buffer: *mut *mut c_void = ptr::null_mut();
		unsafe { (self.locate_handle_buffer)(ty, guid, key, &mut num_handles, &mut buffer,) }
			.ok_or_with(|| (num_handles, buffer,),)
	}

	pub fn protocols_for(&self,) -> Vec<(),> {
		todo!()
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
