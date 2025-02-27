//! provides file manipulation functionality

// mod via_fs_module;
pub mod via_simple_filesystem;

use uefi::boot;

pub fn img_fs()
-> Result<boot::ScopedProtocol<uefi::proto::media::fs::SimpleFileSystem,>, uefi::Error,> {
	boot::get_image_file_system(boot::image_handle(),)
}
