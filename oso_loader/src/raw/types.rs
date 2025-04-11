//! uefi implementation

use core::ffi::c_void;

// #[repr(C)]
// pub struct EfiSystemTable {
// 	header:             Header,
// 	firmware_vendor:    u16,
// 	firmware_revision:  u32,
// 	stdin:              *mut c_void,
// 	stdin_handle:       *mut TextInputProtocol,
// 	stdout:             *mut c_void,
// 	stdout_handle:      *mut TextOutputProtocol,
// 	stderr:             *mut c_void,
// 	stderr_handle:      *mut TextOutputProtocol,
// 	runtime_services:   *mut RuntimeServices,
// 	boot_services:      *mut BootServices,
// 	config_table_count: usize,
// 	config_table:       *mut ConfigTable,
// }
//
// #[repr(C)]
// pub struct TextInputProtocol {
// 	reset:           unsafe extern "efiapi" fn(this: *mut Self, extended_verif: Boolean,) -> Status,
// 	read_key_stroke: unsafe extern "efiapi" fn(this: *mut Self, key: *const InputKey,) -> Status,
// 	wait_for_key:    Event,
// }
//
// #[repr(C)]
// pub struct TextOutputProtocol {
// 	reset:         TextOutputReset,
// 	output:        TextOutputString,
// 	test:          TextTestString,
// 	query_mode:    TextQueryMode,
// 	set_mode:      TextSetMode,
// 	set_attr:      TextSetAttr,
// 	clear:         TextClearScreen,
// 	set_cursor:    TextSetCursor,
// 	enable_cursor: TextEnableCursor,
// 	mode:          *mut TextOutputMode,
// }

/// abi compatible uefi boolean
/// 0 is false,
/// others are true
#[repr(C)]
pub struct Boolean(pub u8,);

impl Boolean {
	const FALSE: Self = Self(0,);
	const TRUE: Self = Self(1,);
}

#[oso_proc_macro::status_from_spec(2.11)]
#[repr(C)]
pub enum Status {}
