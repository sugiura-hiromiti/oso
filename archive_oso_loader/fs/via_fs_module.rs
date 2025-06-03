//! uefi::fsを使ってみたかったのでお試し実装

use alloc::string::String;
use uefi::fs;
use uefi::fs::FileSystemResult;

pub fn img_root_dir() -> uefi::Result<fs::FileSystem,> {
	let img_loaded_fs = super::img_fs()?;
	let root_dir = fs::FileSystem::from(img_loaded_fs,);
	Ok(root_dir,)
}

pub fn write_file(path: &str, content: &str,) -> FileSystemResult<(),> {
	let path = path.bytes().map(|code| code as u16,).collect::<alloc::vec::Vec<u16,>>();
	let path = path.as_slice();
	let path = uefi::CStr16::from_u16_with_nul(path,).expect("failed to convert to Cstr16",);
	let path = uefi::fs::Path::new(path,);
	let mut root_dir = img_root_dir().expect("failed to get root_dir",);
	root_dir.write(path, content,)
}

pub fn read_file(path: &str,) -> Result<String, uefi::fs::Error,> {
	let path = path.bytes().map(|code| code as u16,).collect::<alloc::vec::Vec<u16,>>();
	let path = path.as_slice();
	let path = uefi::CStr16::from_u16_with_nul(path,).expect("failed to convert to Cstr16",);
	let path = uefi::fs::Path::new(path,);
	let mut root_dir = img_root_dir().expect("failed to get root_dir",);

	root_dir.read_to_string(path,)
}
