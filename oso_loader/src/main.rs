#![no_std]
#![no_main]

extern crate alloc;

use oso_bridge::graphic::FrameBufConf;
use oso_loader::Rslt;
use oso_loader::chibi_uefi::service::exit_boot_services;
use oso_loader::exec_kernel;
use oso_loader::init;
use oso_loader::load::graphic_config;
use oso_loader::load::kernel;
use oso_loader::print;
use oso_loader::println;
use oso_loader::raw::table::SystemTable;
use oso_loader::raw::types::Status;
use oso_loader::raw::types::UnsafeHandle;

#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_image_entry_point(
	image_handle: UnsafeHandle,
	system_table: *const SystemTable,
) -> Status {
	init(image_handle, system_table,);

	let (kernel_entry, graphic_config,) = app().expect("error arise while executing application",);

	#[cfg(target_arch = "aarch64")]
	{
		assert_eq!(kernel_entry, 0x40010120, "path: {}", module_path!());
		println!("{kernel_entry:#x}");
	}

	exit_boot_services();

	exec_kernel(kernel_entry, graphic_config,);

	Status::EFI_SUCCESS
}

fn app() -> Rslt<(u64, FrameBufConf,),> {
	let kernel_addr = kernel()?;
	let graphic_config = graphic_config()?;

	Ok((kernel_addr, graphic_config,),)
}
