//! oso_loader/kernel用のエラー型
//! TODO: アロケータを実装したら`oso_util`に移管
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use core::error::Error;
use core::fmt::Debug;
use core::fmt::Display;
use core::num::ParseIntError;

#[derive(Debug,)]
pub enum OsoLoaderError {
	Uefi(String,),
	EfiParse(String,),
}

impl Error for OsoLoaderError {}

impl Display for OsoLoaderError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_,>,) -> core::fmt::Result {
		let represent = match self {
			OsoLoaderError::Uefi(e,) => format!("{e:?}"),
			OsoLoaderError::EfiParse(e,) => format!("{e:?}"),
		};
		write!(f, "{represent}")
	}
}

impl From<goblin::error::Error,> for OsoLoaderError {
	fn from(value: goblin::error::Error,) -> Self {
		Self::EfiParse(format!("{value:?}"),)
	}
}

impl From<OsoLoaderError,> for core::fmt::Error {
	fn from(value: OsoLoaderError,) -> Self {
		core::fmt::Error
	}
}

impl From<ParseIntError,> for OsoLoaderError {
	fn from(value: ParseIntError,) -> Self {
		Self::Uefi(value.to_string(),)
	}
}
