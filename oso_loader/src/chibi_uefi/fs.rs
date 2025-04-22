use core::ptr;

use crate::Rslt;
use crate::into_null_terminated_utf16;
use crate::raw::protocol::file::FileProtocolV1;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::OpenMode;

impl SimpleFileSystemProtocol {
	pub fn open_volume(&mut self,) -> Rslt<&mut FileProtocolV1,> {
		let root = ptr::null_mut();
		unsafe { (self.open_volume)(self, root,) }.ok_or_with(|_| {
			unsafe { (*root).as_mut() }
				.expect("tried to open volume. but returned file protocol is null",)
		},)
	}
}

impl FileProtocolV1 {
	/// opens a new file relative to the source directory's location
	pub fn open(
		&mut self,
		path: impl AsRef<str,>,
		mode: OpenMode,
		attrs: FileAttributes,
	) -> Rslt<&mut FileProtocolV1,> {
		let path = into_null_terminated_utf16(path,);
		let file = ptr::null_mut();

		unsafe { (self.open)(self, file, path, mode, attrs,) }
			.ok_or_with(|_| unsafe { (*file).as_mut() }.unwrap(),)
	}
}
