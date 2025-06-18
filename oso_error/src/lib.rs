#![no_std]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use core::error::Error;
use core::fmt::Debug;
use core::fmt::Display;
use core::num::ParseIntError;

pub type Rslt<T,> = Result<T, OsoError,>;

#[derive(Debug, Default,)]
pub struct OsoError<V = (),>
where V: Debug + Default
{
	pub from: &'static str,
	pub desc: Option<V,>,
}

#[macro_export]
macro_rules! oso_err {
	($causal:expr) => {
		$crate::OsoError { from: module_path!(), ..Default::default() }
	};
}

impl<V: Debug + Default,> OsoError<V,> {
	pub fn desc(&mut self, val: V,) -> &mut Self {
		self.desc = Some(val,);
		self
	}
}

// #[derive(Debug,)]
// pub enum OsoLoaderError {
// 	Uefi(String,),
// 	EfiParse(String,),
// }
//
// impl Error for OsoLoaderError {}
//
// impl Display for OsoLoaderError {
// 	fn fmt(&self, f: &mut core::fmt::Formatter<'_,>,) -> core::fmt::Result {
// 		let represent = match self {
// 			OsoLoaderError::Uefi(e,) => format!("{e:?}"),
// 			OsoLoaderError::EfiParse(e,) => format!("{e:?}"),
// 		};
// 		write!(f, "{represent}")
// 	}
// }
//
// impl From<OsoLoaderError,> for core::fmt::Error {
// 	fn from(_value: OsoLoaderError,) -> Self {
// 		core::fmt::Error
// 	}
// }
//
// impl From<ParseIntError,> for OsoLoaderError {
// 	fn from(value: ParseIntError,) -> Self {
// 		Self::Uefi(value.to_string(),)
// 	}
// }

// pub mod error {
// 	#[derive(Debug,)]
// 	pub enum KernelError {
// 		Graphics(GraphicError,),
// 	}
//
// 	#[derive(Debug,)]
// 	pub enum GraphicError {
// 		InvalidCoordinate,
// 	}
// 	impl From<KernelError,> for core::fmt::Error {
// 		fn from(_value: KernelError,) -> Self {
// 			core::fmt::Error
// 		}
// 	}
// }
