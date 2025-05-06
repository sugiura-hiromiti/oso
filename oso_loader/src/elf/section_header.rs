use alloc::vec::Vec;

use crate::Rslt;
use crate::elf::read_le_bytes;

pub struct SectionHeader {
	pub name:          usize,
	pub ty:            u32,
	pub flags:         u64,
	pub address:       u64,
	pub offset:        u64,
	pub size:          u64,
	pub link:          u64,
	pub info:          u64,
	pub section_align: u64,
	pub entry_size:    u64,
}

impl SectionHeader {
	const SIZE_64: usize = 64;

	pub fn parse(binary: &[u8], offset: &mut usize, count: usize,) -> Rslt<Vec<Self,>,> {
		assert!(count <= binary.len() / Self::SIZE_64, "binary is too small");

		let mut section_headers = Vec::with_capacity(count,);

		for _ in 0..count {
			let name = read_le_bytes(offset, binary,);
			let ty = read_le_bytes(offset, binary,);
			let flags = read_le_bytes(offset, binary,);
			let address = read_le_bytes(offset, binary,);
			let segment_offset = read_le_bytes(offset, binary,);
			let size = read_le_bytes(offset, binary,);
			let link = read_le_bytes(offset, binary,);
			let info = read_le_bytes(offset, binary,);
			let section_align = read_le_bytes(offset, binary,);
			let entry_size = read_le_bytes(offset, binary,);

			let section_header = Self {
				name,
				ty,
				flags,
				address,
				offset: segment_offset,
				size,
				link,
				info,
				section_align,
				entry_size,
			};

			section_headers.push(section_header,);
		}

		Ok(section_headers,)
	}
}
