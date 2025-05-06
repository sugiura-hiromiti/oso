use crate::Rslt;
use crate::elf::read_le_bytes;
use alloc::vec::Vec;

pub struct ProgramHeader {
	pub ty:               u32,
	pub flags:            u32,
	pub offset:           u64,
	pub virtual_address:  u64,
	pub physical_address: u64,
	pub file_size:        u64,
	pub memory_size:      u64,
	pub align:            u64,
}

impl ProgramHeader {
	const SIZE_64: usize = 56;

	pub fn parse(binary: &[u8], offset: &mut usize, count: usize,) -> Rslt<Vec<Self,>,> {
		assert!(count <= binary.len() / Self::SIZE_64, "binary is too small");

		let mut program_headers = Vec::with_capacity(count,);

		for _ in 0..count {
			let ty = read_le_bytes(offset, binary,);
			let flags = read_le_bytes(offset, binary,);
			let segment_offset = read_le_bytes(offset, binary,);
			let virtual_address = read_le_bytes(offset, binary,);
			let physical_address = read_le_bytes(offset, binary,);
			let file_size = read_le_bytes(offset, binary,);
			let memory_size = read_le_bytes(offset, binary,);
			let align = read_le_bytes(offset, binary,);

			let program_header = Self {
				ty,
				flags,
				offset: segment_offset,
				virtual_address,
				physical_address,
				file_size,
				memory_size,
				align,
			};

			program_headers.push(program_header,);
		}

		Ok(program_headers,)
	}
}
