use super::Boolean;
use super::Char16;
use super::Event;
use super::Status;
use super::time::Time;
use crate::c_style_enum;
use crate::chibi_uefi::protocol::Protocol;
use core::ffi::c_void;

//#[repr(transparent)]
//pub struct OpenMode(pub u64,);

c_style_enum! {
	pub enum OpenMode : u64 => {
		CREATE = 0x8000000000000000,
		READ = 0x1,
		WRITE = 0x2,
	}
}

c_style_enum! {
	pub enum FileAttributes: u64 => {
		ARCHIVE = 0x0000000000000020,
		DIRECTORY = 0x0000000000000010,
		HIDDEN = 0x0000000000000002,
		READ_ONLY = 0x0000000000000001,
		RESERVED = 0x0000000000000008,
		SYSTEM = 0x0000000000000004,
		VALID_ATTR = 0x0000000000000037,
	}
}

#[repr(C)]
pub struct FileIoToken {
	event:    Event,
	status:   Status,
	buf_size: usize,
	buf:      *mut c_void,
}

pub trait FileInformation: Protocol {}
impl FileInformation for FileInfo {}
impl FileInformation for FileSystemInfo {}

#[repr(C)]
pub struct FileInfo {
	size:             u64,
	file_size:        u64,
	physical_size:    u64,
	created_at:       Time,
	last_accessed_at: Time,
	modified_at:      Time,
	attr:             FileAttributes,
	/// must be null terminated
	file_name:        [Char16; 0],
}

#[repr(C)]
pub struct FileSystemInfo {
	size:         u64,
	read_only:    Boolean,
	volume_size:  u64,
	free_space:   u64,
	block_size:   u32,
	volume_label: [Char16; 0],
}

#[repr(C)]
pub struct FileSystemVolumeLabel {
	volume_label: [Char16; 0],
}
