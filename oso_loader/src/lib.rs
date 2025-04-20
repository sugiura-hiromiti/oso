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

use core::arch::asm;

use chibi_uefi::Handle;
use chibi_uefi::console::console_mut;
use chibi_uefi::table::system_table;
use error::OsoLoaderError;
use raw::table::SystemTable;
use raw::types::Status;
use raw::types::UnsafeHandle;

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

pub fn init(image_handle: UnsafeHandle, syst: *const SystemTable,) -> Rslt<(),> {
	chibi_uefi::table::set_system_table_panicking(syst,);
	chibi_uefi::set_image_handle_panicking(image_handle,);
	chibi_uefi::console::init()
}

#[inline(always)]
pub fn wfi() -> ! {
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfi");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}
