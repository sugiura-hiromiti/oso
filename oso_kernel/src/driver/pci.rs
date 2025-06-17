//! parse device tree (FDT) which is provided by firmware to get pci devices controll

use crate::Rslt;

pub trait DeviceTree:
	DeviceTreeMemoryReservation + DeviceTreeStructure + DeviceTreeStrings
{
	fn memory_reservation_parser(&self,) -> &impl DeviceTreeMemoryReservation {
		self
	}
	fn structure_parser(&self,) -> &impl DeviceTreeStructure {
		self
	}

	fn strings_parser(&self,) -> &impl DeviceTreeStrings {
		self
	}
}

pub trait DeviceTreeMemoryReservation: MemoryReserveEntry {
	fn mem_entries_count(&self,) -> usize;
	fn nth(&self, n: usize,) -> MemoryReserveEntryData;
}

pub trait MemoryReserveEntry: BinaryParser<false, usize,> {
	fn address(&self,) -> usize;
	fn mem_size(&self,) -> usize;
}

pub trait DeviceTreeStructure {
	fn next_node(&self,) -> StructureToken;
}

pub trait DeviceTreeStrings {}

pub trait BinaryParser<const IS_LITTLE_ENDIAN: bool, T: BinaryParserTarget,>: Sized {
	// get type info
	fn is_little_endian() -> bool {
		IS_LITTLE_ENDIAN
	}

	fn is_big_endian() -> bool {
		!IS_LITTLE_ENDIAN
	}

	// inner state related
	fn raw(&self,) -> *const u8;
	fn cur_pos(&self,) -> usize;
	fn set_pos(&mut self, to: usize,);
	fn advance(&mut self, by: usize,) -> &mut Self {
		let cur_pos = self.cur_pos();
		self.set_pos(cur_pos + by,);
		self
	}

	fn bytes_of(&self, offset: usize, len: usize,) -> &[u8] {
		let raw = unsafe { self.raw().add(offset,) };
		unsafe { core::slice::from_raw_parts(raw, len,) }
	}

	fn read_range(&mut self,) -> &[u8] {
		let cur_pos = self.cur_pos();
		self.set_pos(cur_pos + T::DATA_SIZE,);
		self.bytes_of(cur_pos, T::DATA_SIZE,)
	}

	/// return parsed value and update inner stage for next next parse action
	fn parse(&mut self,) -> Rslt<T::Output,> {
		let bytes = self.read_range();
		T::try_interpret(bytes,)
	}

	/// basically same to `parse` fn but this only returns parsed result
	/// do not update inner state of parser
	fn peek(&self, offset: usize,) -> Rslt<T::Output,> {
		let bytes = self.bytes_of(offset, T::DATA_SIZE,);
		T::try_interpret(bytes,)
	}
}

pub trait BinaryParserTarget: Sized {
	type Output = Self;

	/// byte size of parsed data
	const DATA_SIZE: usize = size_of::<Self::Output,>();

	fn try_interpret(bytes: &[u8],) -> Rslt<Self::Output,>;
}

impl BinaryParserTarget for usize {
	fn try_interpret(bytes: &[u8],) -> Rslt<Self::Output,> {
		todo!()
	}
}

pub struct DeviceTreeData {
	ptr:                              *const u8,
	current_offset:                   usize,
	header:                           FlattenedDeviceTreeHeader,
	memory_reservation_entries_count: usize,
}

struct FlattenedDeviceTree {
	fdt_header:               FlattenedDeviceTreeHeader,
	memory_reservation_block: MemoryReservationBlock,
	structure_block:          StructureBlock,
	strings_block:            StringsBlock,
}

struct FlattenedDeviceTreeHeader {
	magic:                           u32,
	total_size:                      u32,
	struct_block_offset:             u32,
	strings_block_offset:            u32,
	memory_reservation_block_offset: u32,
	version:                         u32,
	last_compatible_version:         u32,
	system_boot_cpu_physical_id:     u32,
	strings_block_size:              u32,
	struct_block_size:               u32,
}

struct MemoryReservationBlock {}

pub struct MemoryReserveEntryData {
	entry_address: *const u8,
}

impl MemoryReserveEntry for MemoryReserveEntryData {
	fn address(&self,) -> usize {
		todo!()
	}

	fn mem_size(&self,) -> usize {
		todo!()
	}
}

impl BinaryParser<false, usize,> for MemoryReserveEntryData {
	fn raw(&self,) -> *const u8 {
		todo!()
	}

	fn cur_pos(&self,) -> usize {
		todo!()
	}

	fn set_pos(&mut self, to: usize,) {
		todo!()
	}
}

struct StructureBlock {}

pub enum StructureToken {
	BeginNode,
	EndNode,
	Property,
	Nop,
	End,
}

struct StringsBlock {}
