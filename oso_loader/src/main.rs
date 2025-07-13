#![no_std]
#![no_main]

extern crate alloc;

use oso_error::Rslt;
use oso_loader::chibi_uefi::service::exit_boot_services;
use oso_loader::exec_kernel;
use oso_loader::get_device_tree;
use oso_loader::init;
use oso_loader::load::kernel;
use oso_loader::raw::table::SystemTable;
use oso_loader::raw::types::Status;
use oso_loader::raw::types::UnsafeHandle;
use oso_no_std_shared::bridge::device_tree::DeviceTreeAddress;

#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_image_entry_point(
	image_handle: UnsafeHandle,
	system_table: *const SystemTable,
) -> Status {
	init(image_handle, system_table,);

	let (kernel_entry, device_tree_ptr,) = app().expect("error arise while executing application",);

	exit_boot_services();

	exec_kernel(kernel_entry, device_tree_ptr,);

	Status::EFI_SUCCESS
}

fn app() -> Rslt<(u64, DeviceTreeAddress,),> {
	let kernel_addr = kernel()?;
	let device_tree = get_device_tree()?;

	let device_tree_ptr = device_tree.as_ptr().cast_const().cast();

	Ok((kernel_addr, device_tree_ptr,),)
}
