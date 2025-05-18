#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::arch::asm;
use core::ffi::c_void;
use oso_bridge::graphic::FrameBufConf;
use oso_bridge::graphic::PixelFormatConf;
use oso_loader::Rslt;
use oso_loader::chibi_uefi::service::exit_boot_services;
use oso_loader::error::OsoLoaderError;
use oso_loader::exec_kernel;
use oso_loader::init;
use oso_loader::load::graphic_config;
use oso_loader::load::kernel;
use oso_loader::print;
use oso_loader::println;
use oso_loader::raw::table::SystemTable;
use oso_loader::raw::types::Status;
use oso_loader::raw::types::UnsafeHandle;
use oso_loader::wfi;

#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_image_entry_point(
	image_handle: UnsafeHandle,
	system_table: *const SystemTable,
) -> Status {
	init(image_handle, system_table,);

	let (kernel_entry, graphic_config,) = app().expect("error arise while executing application",);
	exit_boot_services();

	exec_kernel(kernel_entry, graphic_config,);

	Status::EFI_SUCCESS
}

fn app() -> Rslt<(u64, FrameBufConf,),> {
	let kernel_addr = kernel()?;
	println!("kernel_addr: {kernel_addr:#x}");

	let graphic_config = graphic_config()?;
	println!("graphic_config: {graphic_config:?}");

	Ok((kernel_addr, graphic_config,),)
}

// fn exit_boot_services() {
// 	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
// 	core::mem::forget(mem_map,);
// }
//
// fn exec_kernel(fbc: FrameBufConf, kernel_addr: u64,) {
// 	#[cfg(target_arch = "aarch64")]
// 	let entry_point: extern "C" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	#[cfg(target_arch = "riscv64")]
// 	let entry_point: extern "C" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	#[cfg(target_arch = "x86_64")]
// 	let entry_point: extern "sysv64" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	entry_point(fbc,);
// }
