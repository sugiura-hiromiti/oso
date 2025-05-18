use crate::Rslt;
use crate::chibi_uefi::required_pages;
use crate::chibi_uefi::table::boot_services;
use crate::elf::Elf;
use crate::elf::program_header::ProgramHeaderType;
use crate::print;
use crate::println;
use crate::raw::protocol::file::FileProtocolV1;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::protocol::graphic::GraphicsOutputProtocol;
use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::OpenMode;
use crate::raw::types::memory::AllocateType;
use alloc::vec::Vec;
use core::ptr::NonNull;
use oso_bridge::graphic::FrameBufConf;

pub fn kernel() -> Rslt<PhysicalAddress,> {
	let mut kernel_file = open_kernel_file()?;
	let contents = unsafe { kernel_file.as_mut() }.read_as_bytes()?;
	let elf = Elf::parse(&contents,)?;

	let (head, tail,) = elf_address_range(&elf,);
	println!("head: {head:#x} tail: {tail:#x}");
	let kernel_size = tail - head;

	let page_count = required_pages(kernel_size,);
	let alloc_head = boot_services().allocate_pages(
		AllocateType::ALLOCATE_ADDRESS,
		crate::raw::types::memory::MemoryType::LOADER_DATA,
		page_count,
		head as u64,
	)?;

	assert_eq!(alloc_head as usize, head);

	copy_load_segment(&elf, &contents,);

	Ok(elf.entry_point_address() as u64,)
}

fn open_kernel_file() -> Rslt<NonNull<FileProtocolV1,>,> {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	let bs = boot_services();
	let sfs_handle = unsafe { bs.handle_for_protocol::<SimpleFileSystemProtocol>() }?;

	let volume = unsafe {
		bs.open_protocol_exclusive::<SimpleFileSystemProtocol>(sfs_handle,)?.interface().as_mut()
	}
	.open_volume()?;

	let kernel_file = volume.open("oso_kernel.elf", open_mode, attrs,)?;
	let non_null_kernel_file = NonNull::new(kernel_file,).expect("reference can't be null",);
	Ok(non_null_kernel_file,)
}

/// # Return
///
/// returned tuple (usize, usize) represents (head_address, tail_address)
fn elf_address_range(elf: &Elf,) -> (usize, usize,) {
	let mut pair = (usize::MAX, 0,);
	for ph in &elf.program_headers {
		if ph.ty != ProgramHeaderType::Load {
			continue;
		}

		let segment_head = ph.virtual_address as usize;
		let segment_tail = (ph.virtual_address + ph.memory_size) as usize;

		pair.0 = pair.0.min(segment_head,);
		pair.1 = pair.1.max(segment_tail,);
	}

	pair
}

fn copy_load_segment(elf: &Elf, src: &Vec<u8,>,) {
	for ph in &elf.program_headers {
		if ph.ty != ProgramHeaderType::Load {
			continue;
		}

		// `size_on_mem` maybe larger than `size` due to `.bss` section
		let mem_size = ph.memory_size as usize;
		let dest =
			unsafe { core::slice::from_raw_parts_mut(ph.virtual_address as *mut u8, mem_size,) };

		let offset = ph.offset as usize;
		let file_size = ph.file_size as usize;

		// copy contents of segment described by current program header
		dest[..file_size].copy_from_slice(&src[offset..offset + file_size],);
		// fill remaining bytes by 0
		dest[file_size..].fill(0,);
	}
}

pub fn graphic_config() -> Rslt<FrameBufConf,> {
	let bs = boot_services();

	let mut gout = bs.open_protocol_with::<GraphicsOutputProtocol>()?.interface();
	let gout = unsafe { gout.as_mut() };

	let info = gout.mode();
	let (width, height,) = info.resolution();
	let stride = info.stride();
	let base = info.frame_buffer_base as *mut u8;
	let size = info.frame_buffer_size;
	let pixel_format = info.pixel_format();

	let fbc = FrameBufConf::new(pixel_format, base, size, width, height, stride,);

	Ok(fbc,)
}
