use super::raw::types::Status;
use crate::Rslt;
use crate::raw::types::UnsafeHandle;
use core::ffi::c_void;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

pub mod console;
pub mod guid;
pub mod memory;
pub mod protocol;
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
