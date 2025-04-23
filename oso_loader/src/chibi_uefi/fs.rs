use core::ptr;

use alloc::vec;
use alloc::vec::Vec;

use crate::Rslt;
use crate::error::OsoLoaderError;
use crate::into_null_terminated_utf16;
use crate::raw::protocol::file::FileProtocolV1;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::Char16;
use crate::raw::types::Status;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::FileInfo;
use crate::raw::types::file::FileInformation;
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

	/// reads file content to buf
	///
	/// # Return
	///
	/// returns bytes amount of read data
	pub unsafe fn read(&mut self, buf: &mut [Char16],) -> Rslt<usize,> {
		let mut len = buf.len();
		unsafe { (self.read)(self, &mut len, buf.as_mut_ptr().cast(),) }.ok_or_with(|_| len,)
	}

	pub fn get_info<F: FileInformation,>(&mut self,) -> Rslt<F,> {
		let mut len = self.info_size()?;
		let mut buf = vec![0; len];

		unsafe { (self.get_info)(self, &F::GUID, &mut len, buf.as_mut_ptr().cast(),) };
	}

	pub fn get_file_info(&mut self,) -> Rslt<FileInfo,> {
		todo!()
	}

	pub fn info_size<F: FileInformation,>(&mut self,) -> Rslt<usize,> {
		let mut len = 0;
		let status = unsafe { (self.get_info)(self, &F::GUID, &mut len, ptr::null_mut(),) };
		match status {
			Status::EFI_BUFFER_TOO_SMALL => Ok(len,),
			Status::EFI_SUCCESS => unreachable!(),
			_ => status.ok_or_with(|_| 0,),
		}
	}
}
