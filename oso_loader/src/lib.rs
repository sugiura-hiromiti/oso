#![no_std]
#![feature(alloc_error_handler)]
#![feature(ptr_as_ref_unchecked)]
#![feature(iter_next_chunk)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![feature(assert_matches)]

extern crate alloc;

use alloc::vec::Vec;
use chibi_uefi::Handle;
use chibi_uefi::protocol::HandleSearchType;
use chibi_uefi::protocol::Protocol;
use chibi_uefi::table::boot_services;
use core::arch::asm;
use error::OsoLoaderError;
use raw::protocol::device_path::DevicePathProtocol;
use raw::table::SystemTable;
use raw::types::Status;
use raw::types::UnsafeHandle;

pub mod chibi_uefi;
pub mod error;
pub mod load;
pub mod raw;

pub type Rslt<T = Status,> = Result<T, OsoLoaderError,>;

#[panic_handler]
fn panic(panic: &core::panic::PanicInfo,) -> ! {
	println!("{panic:#?}");
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfe");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}

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
	unsafe { syst.as_ref().unwrap().stdout.as_mut().unwrap().clear().unwrap() };
	chibi_uefi::table::set_system_table_panicking(syst,);
	chibi_uefi::set_image_handle_panicking(image_handle,);

	// connect devices
	let bs = boot_services();

	// uefi only installs DevicePathProtocol on devices that are fully connected
	// `AllHandles` is the only way to find unconnected devices
	let handles = unsafe {
		bs.locate_handle_buffer(HandleSearchType::AllHandles,)
			.expect("failed to locate all handles ",)
	};
	handles.iter().for_each(|handle| {
		// ignore errors from connect_controller intendly
		unsafe { bs.connect_controller(*handle, None, None, raw::types::Boolean::TRUE,) };
	},);
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

fn into_null_terminated_utf16(s: impl AsRef<str,>,) -> Vec<u16,> {
	let mut utf16_repr: Vec<u16,> = s.as_ref().encode_utf16().collect();
	utf16_repr.push(0,);
	utf16_repr
}
