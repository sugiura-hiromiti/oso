//! parse device tree (FDT) which is provided by firmware to get pci devices controll
//! TODO:
//! - derive macroを用いて型定義からlazyパーサーを自動生成する
//! - マクロが生成するパーサーのための基盤を`oso_binary_parser`で提供する
#![allow(dead_code)]

use oso_error::Rslt;

pub trait DeviceTree: DeviceTreeHeader + DeviceTreeMemoryReservation + DeviceTreeStructure {
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

pub trait DeviceTreeHeader {
	fn check_magic(&self,) -> bool;
	fn total_size(&self,) -> usize;
	fn structure_block_offset(&self,) -> usize;
	fn strings_block_offset(&self,) -> usize;
	fn memory_reservation_block_offset(&self,) -> usize;
	fn version(&self,) -> usize;
	fn last_compatible_version(&self,) -> usize;
	fn system_boot_cpu_physical_id(&self,) -> usize;
	fn strings_block_size(&self,) -> usize;
	fn structure_block_size(&self,) -> usize;
}

pub trait DeviceTreeMemoryReservation: MemoryReserveEntry {
	fn mem_entries_count(&self,) -> usize;
	fn nth(&self, n: usize,) -> MemoryReserveEntryData;
}

pub trait MemoryReserveEntry: BinaryParser<false, usize,> {
	fn address(&self,) -> usize;
	fn mem_size(&self,) -> usize;
}

pub trait DeviceTreeStructure: DeviceTreeStrings {
	fn next_node(&self,) -> StructureToken;
	fn next_node_tree(&self,) -> StructureToken;
	fn get_node(&self, name: &str,);
}

pub trait DeviceTreeStrings {
	/// offset arg is offset from start of strings block
	fn get_name(&self, offset: usize,) -> &str;
	fn is_node_of(&self, offset: usize, name: &str,) -> bool {
		self.get_name(offset,) == name
	}
}

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
	fn try_interpret(_bytes: &[u8],) -> Rslt<Self::Output,> {
		todo!()
	}
}

pub struct DeviceTreeData {
	ptr:                              *const u8,
	cur_pos:                          usize,
	// header:                           FlattenedDeviceTreeHeader,
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

	fn set_pos(&mut self, _to: usize,) {
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
