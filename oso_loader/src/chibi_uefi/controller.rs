use core::ptr;

use crate::Rslt;
use crate::raw::protocol::device_path::DevicePathProtocol;
use crate::raw::service::BootServices;
use crate::raw::types::Boolean;
use crate::raw::types::UnsafeHandle;

use super::Handle;

impl BootServices {
	pub fn connect_controller(
		&self,
		controller_handle: Handle,
		driver_image_handle: Option<Handle,>,
		remaining_device_path: Option<DevicePathProtocol,>,
		recursive: Boolean,
	) -> Rslt {
		let driver_image_handle = match driver_image_handle {
			Some(h,) => h.as_ptr(),
			None => ptr::null_mut(),
		};
		let remaining_device_path = match remaining_device_path {
			Some(dpp,) => &dpp as *const _,
			None => ptr::null(),
		};

		unsafe {
			(self.connect_controller)(
				controller_handle.as_ptr(),
				driver_image_handle,
				remaining_device_path,
				recursive,
			)
		}
		.ok_or()
	}
}
