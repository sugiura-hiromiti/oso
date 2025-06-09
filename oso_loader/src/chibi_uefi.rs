use super::raw::types::Status;
use crate::print;
use crate::println;
use crate::raw::service::BootServices;
use crate::raw::service::RuntimeServices;
use crate::raw::types::UnsafeHandle;
use crate::raw::types::memory::MemoryMapBackingMemory;
use crate::raw::types::memory::MemoryType;
use crate::raw::types::memory::PAGE_SIZE;
use crate::raw::types::misc::ResetType;
use core::ffi::c_void;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

pub mod console;
pub mod controller;
pub mod fs;
pub mod guid;
pub mod memory;
pub mod protocol;
pub mod service;
pub mod table;

static IMAGE_HANDLE: AtomicPtr<c_void,> = AtomicPtr::new(core::ptr::null_mut(),);

#[derive(Clone,)]
#[repr(transparent)]
pub struct Handle(NonNull<c_void,>,);

impl Handle {
	/// # Safety
	///
	/// caller must be sure that pointer is valid
	#[must_use]
	pub const unsafe fn new(ptr: NonNull<c_void,>,) -> Self {
		Self(ptr,)
	}

	/// # Safety
	///
	/// caller must be sure that pointer is valid
	pub unsafe fn from_ptr(ptr: UnsafeHandle,) -> Option<Self,> {
		NonNull::new(ptr,).map(Self,)
	}

	/// get underlying raw pointer
	pub const fn as_ptr(&self,) -> UnsafeHandle {
		self.0.as_ptr()
	}

	/// # Note
	///
	/// returned value may null
	pub fn opt_to_ptr(handle: Option<Handle,>,) -> UnsafeHandle {
		handle.map(|h| h.as_ptr(),).unwrap_or(core::ptr::null_mut(),)
	}
}

impl Status {
	pub fn is_success(&self,) -> bool {
		self.clone() == Self::EFI_SUCCESS
	}
}

impl BootServices {
	pub fn exit_boot_services(&self,) {
		let mem_ty = MemoryType::BOOT_SERVICES_DATA;

		let mut buf = MemoryMapBackingMemory::new(mem_ty,).expect("failed to allocate memory",);
		let status = unsafe { self.try_exit_boot_services(buf.as_mut_slice(),) };

		if !status.is_success() {
			todo!("failed to exit boot service. reset the machine");
		}
	}

	unsafe fn try_exit_boot_services(&self, buf: &mut [u8],) -> Status {
		let mem_map = self.get_memory_map(buf,).expect("failed to get memmap",);
		let status =
			unsafe { (self.exit_boot_services)(image_handle().as_ptr(), mem_map.map_key,) };
		core::mem::forget(mem_map,);
		status
	}
}

impl RuntimeServices {
	pub fn reset(&self, _reset_type: ResetType, _status: Status, _data: Option<&[u8],>,) -> ! {
		todo!()
	}
}

pub fn image_handle() -> Handle {
	let p = IMAGE_HANDLE.load(Ordering::Acquire,);
	unsafe { Handle::from_ptr(p,).expect("set_image_handle has not been called",) }
}
unsafe fn set_image_handle(image_handle: Handle,) {
	IMAGE_HANDLE.store(image_handle.as_ptr(), Ordering::Release,);
}

pub(crate) fn set_image_handle_panicking(image_handle: UnsafeHandle,) {
	assert!(!image_handle.is_null());

	let image_handle = unsafe { Handle::from_ptr(image_handle,).unwrap() };
	unsafe { set_image_handle(image_handle,) };

	assert!(!IMAGE_HANDLE.load(Ordering::Acquire,).is_null());
}

pub fn required_pages(size: usize,) -> usize {
	size / PAGE_SIZE + 1
}
