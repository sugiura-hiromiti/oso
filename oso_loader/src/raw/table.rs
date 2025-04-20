use super::protocol::TextInputProtocol;
use super::protocol::TextOutputProtocol;
use super::service::BootServices;
use super::service::RuntimeServices;
use super::types::Char16;
use super::types::Guid;
use super::types::Header;
use super::types::UnsafeHandle;
use core::ffi::c_void;

#[repr(C)]
pub struct SystemTable {
	pub header: Header,

	pub firmware_vendor:   *const Char16,
	pub firmware_revision: u32,

	pub stdin_handle: UnsafeHandle,
	pub stdin:        *mut TextInputProtocol,

	pub stdout_handle: UnsafeHandle,
	pub stdout:        *mut TextOutputProtocol,

	pub stderr_handle: UnsafeHandle,
	pub stderr:        *mut TextOutputProtocol,

	pub runtime_services: *mut RuntimeServices,
	pub boot_services:    *mut BootServices,

	pub config_table_count: usize,
	pub config_table:       *mut ConfigTable,
}

#[derive(Debug, Eq, PartialEq,)]
#[repr(C)]
pub struct ConfigTable {
	vendor_guid:  Guid,
	vendor_table: *mut c_void,
}
