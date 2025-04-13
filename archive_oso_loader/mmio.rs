use crate::error::OsoLoaderError;
use log::trace;
use uefi::boot::MemoryDescriptor;
use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;

pub fn mmio_descriptor() -> Result<MemoryDescriptor, OsoLoaderError,> {
	let mm = uefi::boot::memory_map(MemoryType::MMIO,)?;

	let mut md = MemoryDescriptor::default();
	for i in 0..mm.len() {
		let memory_descriptor = mm[i];
		if memory_descriptor.ty == MemoryType::MMIO {
			trace!("MMIO memory descriptor: {memory_descriptor:?}");
			md = memory_descriptor;
			break;
		}
	}

	Ok(md,)
}
