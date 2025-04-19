#![no_std]
#![feature(alloc_error_handler)]
#![feature(ptr_as_ref_unchecked)]
#![feature(iter_next_chunk)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![feature(assert_matches)]

extern crate alloc;

pub mod chibi_uefi;
pub mod error;
pub mod raw;

use chibi_uefi::console::console_mut;
use chibi_uefi::table::system_table;
use error::OsoLoaderError;
use raw::table::SystemTable;
use raw::types::Status;

pub type Rslt<T = Status,> = Result<T, OsoLoaderError,>;

#[macro_export]
/// ?演算子で処理できないエラーがあった場合に使う
macro_rules! on_error {
	($e:ident, $situation:expr) => {{
		log::error!("error happen {}", $situation);
		log::error!("error msg:");
		log::error!("{}", $e);
	}};
}

#[macro_export]
/// `AsRef<str>`を実装する型の変数をuefi::CStr16型へ変換する
/// 所有権の問題で関数ではなくマクロになっている
macro_rules! string_to_cstr16 {
	($str:expr, $rslt:ident) => {
		//let $rslt = alloc::string::ToString::to_string($string,);
		let $rslt = $str.as_ref();
		let $rslt: alloc::vec::Vec<u16,> = $rslt.chars().map(|c| c as u16,).collect();
		let $rslt = match uefi::CStr16::from_u16_with_nul(&$rslt[..],) {
			Ok(cstr16,) => cstr16,
			Err(e,) => {
				log::error!("{:?}", e);
				panic!(
					"failed to convert &[u16] to CStr16\ninvalid code may included or not null \
					 terminated",
				);
			},
		};
	};
}

#[macro_export]
macro_rules! print {
	($($args:tt)*) => {
		$crate::print(format_args!($($args)*),);
	};
}

#[macro_export]
macro_rules! println {
	() => {
		$crate::print!("\n");
	};
	($($args:tt)*)=>{
		$crate::print!("{}\n", format_args!($($args)*));
	}
}

pub fn print(args: core::fmt::Arguments,) {
	use core::fmt::Write;
	let c = console_mut();
	c.write_fmt(args,).unwrap();
}

pub fn init(syst: &SystemTable,) -> Rslt<(),> {
	unsafe { chibi_uefi::table::set_system_table(syst,) };
	let syst = system_table();
	chibi_uefi::memory::init(unsafe { syst.as_ref().boot_services.as_ref().unwrap() },);
	chibi_uefi::console::init()
}
