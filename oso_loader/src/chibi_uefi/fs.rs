use core::ptr;

use crate::Rslt;
use crate::raw::protocol::file::FileProtocolV1;
use crate::raw::protocol::file::SimpleFileSystemProtocol;

impl SimpleFileSystemProtocol {
	pub(crate) fn open_volume(&mut self,) -> Rslt<FileProtocolV1,> {
		let root=ptr::null_mut();
		(self.open_volume)(self,root)
		todo!()
	}
}
