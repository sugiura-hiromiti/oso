//! provides file manipulation functionality

// mod via_fs_module;
pub mod via_simple_filesystem;

use uefi::boot;

use crate::error::OsoLoaderError;

pub fn img_fs()
-> Result<boot::ScopedProtocol<uefi::proto::media::fs::SimpleFileSystem,>, OsoLoaderError,> {
	let sfs = boot::get_image_file_system(boot::image_handle(),)?;
	Ok(sfs,)
}
