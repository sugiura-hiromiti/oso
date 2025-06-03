use super::types::Boolean;
use super::types::Status;
use super::types::text::InputKey;
use super::types::text::TextOutputMode;
use core::ffi::c_void;

#[repr(C)]
pub struct TextInputProtocol {
	reset:           unsafe extern "efiapi" fn(this: *mut Self, extended_verif: Boolean,) -> Status,
	read_key_stroke: unsafe extern "efiapi" fn(this: *mut Self, key: *const InputKey,) -> Status,
	wait_for_key:    *mut c_void,
}

#[repr(C)]
pub struct TextOutputProtocol {
	reset:         unsafe extern "efiapi" fn(this: *mut Self, extended_verif: Boolean,) -> Status,
	output:        unsafe extern "efiapi" fn(this: *mut Self, string: *const u16,) -> Status,
	test:          unsafe extern "efiapi" fn(this: *mut Self, string: *const u16,) -> Status,
	query_mode: unsafe extern "efiapi" fn(
		this: *mut Self,
		mode_number: usize,
		columns: *mut usize,
		rows: *mut usize,
	),
	set_mode:      unsafe extern "efiapi" fn(this: *mut Self, mode_number: usize,) -> Status,
	set_attr:      unsafe extern "efiapi" fn(this: *mut Self, attribute: usize,) -> Status,
	clear:         unsafe extern "efiapi" fn(this: *mut Self,) -> Status,
	set_cursor:    unsafe extern "efiapi" fn(this: *mut Self, column: usize, row: usize,) -> Status,
	enable_cursor: unsafe extern "efiapi" fn(this: *mut Self, visible: Boolean,) -> Status,
	mode:          *mut TextOutputMode,
}

#[repr(C)]
pub enum InterfaceType {
	NativeInterface,
}

#[repr(C)]
pub enum LocateSearchType {
	AllHandles,
	ByRegisterNotify,
	ByProtocol,
}

#[repr(C)]
pub struct DevicePathProtocol {
	path_type: u8,
	subtype:   u8,
	length:    [u8; 2],
}

#[repr(C)]
pub struct OpenProtocolInformationEntry {
	agent_handle:      *mut c_void,
	controller_handle: *mut c_void,
	attributes:        u32,
	open_count:        u32,
}
