use core::ffi::c_void;

use super::Event;
use super::Status;

#[repr(transparent)]
pub struct OpenMode(pub u64,);

// TODO: use c_style_enum! macro
impl OpenMode {
	pub const CREATE: u64 = 0x8000000000000000;
	pub const READ: u64 = 0x1;
	pub const WRITE: u64 = 0x2;
}

#[repr(transparent)]
pub struct FileAttributes(pub u64,);

// TODO: use c_style_enum! macro
impl FileAttributes {
	pub const ARCHIVE: u64 = 0x0000000000000020;
	pub const DIRECTORY: u64 = 0x0000000000000010;
	pub const HIDDEN: u64 = 0x0000000000000002;
	pub const READ_ONLY: u64 = 0x0000000000000001;
	pub const RESERVED: u64 = 0x0000000000000008;
	pub const SYSTEM: u64 = 0x0000000000000004;
	pub const VALID_ATTR: u64 = 0x0000000000000037;
}

#[repr(C)]
pub struct FileIoToken {
	event:    Event,
	status:   Status,
	buf_size: usize,
	buf:      *mut c_void,
}
