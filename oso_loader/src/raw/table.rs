use alloc::vec::Vec;

use super::service::BootServices;
use super::service::RuntimeServices;
use super::types::Char16;
use super::types::Guid;
use super::types::Header;
use super::types::UnsafeHandle;
use crate::Rslt;
use crate::guid;
use crate::raw::protocol::text::TextInputProtocol;
use crate::raw::protocol::text::TextOutputProtocol;
use core::ffi::c_void;
use core::ptr::NonNull;

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
	pub config_tables:      *mut ConfigTable,
}

#[derive(Debug, Eq, PartialEq,)]
#[repr(C)]
pub struct ConfigTable {
	vendor_guid:  Guid,
	vendor_table: *mut c_void,
}

pub struct ConfigTableStream {
	current_index: usize,
	max_index:     usize,
	config_tables: Option<NonNull<ConfigTable,>,>,
}

impl ConfigTable {
	pub fn get_config_table_with(&self, guid: Guid,) -> Rslt<Option<&mut ConfigTable,>,> {
		todo!()
	}

	pub fn get_device_tree(&self,) -> Rslt<Option<&mut ConfigTable,>,> {
		self.get_config_table_with(DEVICE_TREE_TABLE_GUID,)
	}
}

pub const DEVICE_TREE_TABLE_GUID: Guid = guid!("b1b621d5-f19c-41a5-830b-d9152c69aae0");

impl SystemTable {
	pub fn get_config_tables(&self,) -> Rslt<ConfigTableStream,> {}
}
