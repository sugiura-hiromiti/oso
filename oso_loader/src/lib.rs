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
pub mod load;
pub mod raw;

use core::arch::asm;

use alloc::vec::Vec;
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

/// # Panics
///
/// panics  when initialization failed
pub fn init(image_handle: UnsafeHandle, syst: *const SystemTable,) {
	chibi_uefi::table::set_system_table_panicking(syst,);
	chibi_uefi::set_image_handle_panicking(image_handle,);
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

fn into_null_terminated_utf16(s: impl AsRef<str,>,) -> *const u16 {
	let mut utf16_repr: Vec<u16,> = s.as_ref().encode_utf16().collect();
	utf16_repr.push(0,);
	utf16_repr.as_ptr()
}
