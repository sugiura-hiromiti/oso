use core::arch::asm;

use log::trace;
use uefi::boot::MemoryAttribute;
use uefi::boot::MemoryDescriptor;
use uefi::boot::MemoryType;

use crate::error::OsoLoaderError;

pub fn mmio_address() -> Result<(), OsoLoaderError,> {
	let mm = uefi::boot::memory_map(MemoryType::MMIO,)?;
	trace!("{mm:?}");
	let syst = uefi::table::system_table_raw().unwrap();

	let memory_map_size = &mut 0usize;
	let memory_descriptor = &mut [0; 1024 * 16];
	let map_key = &mut 0usize;
	let descriptor_size = &mut 0usize;
	let descriptor_version = &mut 0u32;

	let status = unsafe {
		(syst.as_ref().boot_services.as_ref().unwrap().get_memory_map)(
			memory_map_size as *mut usize,
			memory_descriptor as *mut [i32] as *mut MemoryDescriptor,
			map_key as *mut usize,
			descriptor_size as *mut usize,
			descriptor_version as *mut u32,
		)
	};
	trace!("{status}");
	unsafe {
		let mem_dsc = memory_descriptor as *mut [i32] as *mut MemoryDescriptor;
		trace!("{:?}", *mem_dsc);
	}
	loop {
		unsafe { asm!("hlt") }
	}
	Ok((),)
}
