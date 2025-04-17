use crate::Rslt;
use crate::error::OsoLoaderError;
use crate::raw::types::Guid;
use alloc::string::ToString;

#[macro_export]
macro_rules! guid {
	($s:literal) => {};
}

impl Guid {
	pub fn from_str(s: impl AsRef<str,>,) -> Rslt<Self,> {
		let s = s.as_ref();
		let len = s.len();

		for i in  {

		}

		todo!()
	}
}
