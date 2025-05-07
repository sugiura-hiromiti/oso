use super::StringTable;
use crate::Rslt;
use crate::elf::read_le_bytes;
use alloc::vec::Vec;

/// Undefined section.
pub const SHN_UNDEF: u32 = 0;
/// Start of reserved indices.
pub const SHN_LORESERVE: u32 = 0xff00;
/// Start of processor-specific.
pub const SHN_LOPROC: u32 = 0xff00;
/// Order section before all others (Solaris).
pub const SHN_BEFORE: u32 = 0xff00;
/// Order section after all others (Solaris).
pub const SHN_AFTER: u32 = 0xff01;
/// End of processor-specific.
pub const SHN_HIPROC: u32 = 0xff1f;
/// Start of OS-specific.
pub const SHN_LOOS: u32 = 0xff20;
/// End of OS-specific.
pub const SHN_HIOS: u32 = 0xff3f;
/// Associated symbol is absolute.
pub const SHN_ABS: u32 = 0xfff1;
/// Associated symbol is common.
pub const SHN_COMMON: u32 = 0xfff2;
/// Index is in extra table.
pub const SHN_XINDEX: u32 = 0xffff;
/// End of reserved indices.
pub const SHN_HIRESERVE: u32 = 0xffff;

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

pub fn get_string_table(section_headers: &[SectionHeader], mut idx: usize,) -> StringTable {
	if idx == SHN_XINDEX as usize {
		if section_headers.is_empty() {
			return StringTable::default();
		}
	}

	todo!()
}
