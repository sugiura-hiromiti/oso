use super::protocol::TextInputProtocol;
use super::protocol::TextOutputProtocol;
use super::service::BootServices;
use super::service::RuntimeServices;
use super::types::Guid;
use super::types::Header;
use core::ffi::c_void;

#[repr(C)]
pub struct SystemTable {
	pub header:             Header,
	pub firmware_vendor:    u16,
	pub firmware_revision:  u32,
	pub stdin:              *mut c_void,
	pub stdin_handle:       *mut TextInputProtocol,
	pub stdout:             *mut c_void,
	pub stdout_handle:      *mut TextOutputProtocol,
	pub stderr:             *mut c_void,
	pub stderr_handle:      *mut TextOutputProtocol,
	pub runtime_services:   *mut RuntimeServices,
	pub boot_services:      *mut BootServices,
	pub config_table_count: usize,
	pub config_table:       *mut ConfigTable,
}

#[repr(C)]
pub struct ConfigTable {
	vendor_guid:  Guid,
	vendor_table: *mut c_void,
}
