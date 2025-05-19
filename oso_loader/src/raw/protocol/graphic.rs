use crate::raw::types::Status;
use crate::raw::types::graphic::GraphicsOutputBltOperation;
use crate::raw::types::graphic::GraphicsOutputBltPixel;
use crate::raw::types::graphic::GraphicsOutputModeInfo;
use crate::raw::types::graphic::GraphicsOutputProtocolMode;

#[repr(C)]
pub struct GraphicsOutputProtocol {
	pub query_mode: unsafe extern "efiapi" fn(
		*const Self,
		mode_number: u32,
		size_of_info: *mut usize,
		info: *mut *const GraphicsOutputModeInfo,
	) -> Status,
	pub set_mode:   unsafe extern "efiapi" fn(*mut Self, mode_number: u32,) -> Status,
	pub blt: unsafe extern "efiapi" fn(
		*mut Self,
		blt_buffer: *mut GraphicsOutputBltPixel,
		blt_operation: GraphicsOutputBltOperation,
		source_x: usize,
		source_y: usize,
		dest_x: usize,
		dest_y: usize,
		width: usize,
		height: usize,
		delta: usize,
	) -> Status,
	pub mode:       *mut GraphicsOutputProtocolMode,
}

impl GraphicsOutputProtocol {
	pub fn query_mode(&self,) {
		todo!()
	}

	pub fn mode(&self,) -> &GraphicsOutputProtocolMode {
		unsafe { &*self.mode }
	}
}
