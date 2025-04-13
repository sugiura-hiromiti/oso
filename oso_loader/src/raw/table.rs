use super::protocol::TextInputProtocol;
use super::protocol::TextOutputProtocol;
use super::service::BootServices;
use super::service::RuntimeServices;
use super::types::Guid;
use super::types::Header;
use core::ffi::c_void;

#[repr(C)]
pub struct SystemTable {
	header:             Header,
	firmware_vendor:    u16,
	firmware_revision:  u32,
	stdin:              *mut c_void,
	stdin_handle:       *mut TextInputProtocol,
	stdout:             *mut c_void,
	stdout_handle:      *mut TextOutputProtocol,
	stderr:             *mut c_void,
	stderr_handle:      *mut TextOutputProtocol,
	runtime_services:   *mut RuntimeServices,
	boot_services:      *mut BootServices,
	config_table_count: usize,
	config_table:       *mut ConfigTable,
}

#[repr(C)]
pub struct ConfigTable {
	vendor_guid:  Guid,
	vendor_table: *mut c_void,
}
