use crate::Rslt;
use crate::elf::program_header::ProgramHeader;
use crate::elf::section_header::SectionHeader;
use crate::error::OsoLoaderError;
use crate::print;
use crate::println;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::Sum;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Shl;
use core::ops::Shr;
use core::ops::Sub;
use core::ops::SubAssign;
use program_header::ProgramHeaderType;

mod program_header;
mod section_header;

/// used to check magic number
const ELF_MAGIC_NUMBER: &[u8; ELF_MAGIC_NUMBER_SIZE] = b"\x7fELF";
const ELF_MAGIC_NUMBER_SIZE: usize = 4;
/// size of ident in elf header
const ELF_IDENT_SIZE: usize = 16;
/// file class byte index
const ELF_FILE_CLASS_INDEX: usize = 4;
/// indicates 32 bit file class object
const ELF_32_BIT_OBJECT: u8 = 1;
/// indicates 64 bit file class object
const ELF_64_BIT_OBJECT: u8 = 2;
/// index to flag of data encoding(endianness) in ident of elf header
const ELF_ENDIANNESS_INDEX: usize = 5;
/// index to elf version flag
const ELF_VERSION_INDEX: usize = 6;
/// index to target os abi flag
const ELF_OS_ABI_INDEX: usize = 7;
/// index to abi version flag
const ELF_ABI_VERSION_INDEX: usize = 8;

pub struct Elf {
	pub header:                             ElfHeader,
	pub program_headers:                    Vec<ProgramHeader,>,
	pub section_headers:                    Vec<SectionHeader,>,
	pub section_header_string_table:        StringTable,
	pub dynamic_string_table:               StringTable,
	pub dynamic_symbol_table:               SymbolTable,
	pub symbol_table:                       SymbolTable,
	pub string_table_for_symbol_table:      StringTable,
	pub dynamic_info:                       Option<Dynamic,>,
	pub dynamic_relocation_with_addend:     RelocationSection,
	pub dynamic_relocation:                 RelocationSection,
	pub procedure_linkage_table_relocation: RelocationSection,
	pub section_relocations:                Vec<(usize, RelocationSection,),>,
	pub shared_object_name:                 Option<String,>,
	pub interpreter:                        Option<String,>,
	pub libraries:                          Vec<String,>,
	pub runtime_search_path_deprecated:     Vec<String,>,
	pub runtime_search_path:                Vec<String,>,
	pub is_64:                              bool,
	pub is_shared_object:                   bool,
	pub entry_point_address:                u64,
	pub little_endian:                      bool,
	pub symbol_version_section:             Option<SymbolVersionSection,>,
	pub version_definition_section:         Option<VersionDefinitionSection,>,
	pub version_needed_section:             Option<VersionNeededSection,>,
}

impl Elf {
	pub fn parse(binary: &Vec<u8,>,) -> Rslt<Self,> {
		let header = ElfHeader::parse(binary,)?;

		let mut offset = header.program_header_offset as usize;
		let program_headers =
			ProgramHeader::parse(binary, &mut offset, header.program_header_count as usize,)?;

		let mut interpreter = None;

		for program_header in &program_headers {
			if program_header.ty == ProgramHeaderType::Interp && program_header.file_size != 0 {
				let count = program_header.file_size as usize - 1;
				let offset = program_header.offset as usize;
			}
		}

		todo!(
			"
			pub trait Pread<Ctx: Copy, E>

			TryFromCtx<'a, Ctx, Self, Error = E>
			"
		)
	}

	pub fn is_64(&self,) -> bool {
		self.header.is_64()
	}

	pub fn is_lib(&self,) -> bool {
		self.header.is_lib()
	}

	pub fn is_little_endian(&self,) -> bool {
		self.header.is_little_endian()
	}
}

pub struct ElfHeader {
	pub ident: ElfHeaderIdent,
	pub ty: ElfType,
	pub machine: u16,
	pub version: u32,
	pub entry: u64,
	pub program_header_offset: u64,
	pub section_header_offset: u64,
	pub flags: u32,
	pub elf_header_size: u16,
	pub program_header_entry_size: u16,
	pub program_header_count: u16,
	pub section_header_entry_size: u16,
	pub section_header_count: u16,
	pub section_header_index_of_section_name_string_table: u16,
}

impl ElfHeader {
	pub fn parse(binary: &Vec<u8,>,) -> Rslt<Self,> {
		let ident = &binary[..ELF_IDENT_SIZE];
		let ident = ElfHeaderIdent::new(ident,)?;
		let remain = &binary[ELF_IDENT_SIZE..];
		header_flag_fields(ident, remain,)
	}

	fn is_64(&self,) -> bool {
		self.ident.is_64()
	}

	fn is_lib(&self,) -> bool {
		self.ty.is_lib()
	}

	fn is_little_endian(&self,) -> bool {
		self.ident.is_little_endian()
	}
}

fn header_flag_fields(ident: ElfHeaderIdent, ident_remain: &[u8],) -> Rslt<ElfHeader,> {
	let offset = &mut 0;

	{
		let a: u16 = read_le_bytes(&mut 0, &[0x34, 0x12,],);
		assert_eq!(0x1234, a);

		let b: u32 = read_le_bytes(&mut 0, &[0x78, 0x56, 0x34, 0x12,],);
		assert_eq!(0x12345678, b);

		let c: u64 = read_le_bytes(&mut 0, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,],);
		assert_eq!(0x0123456789abcdef, c);

		println!("-------------------------------");
		println!("\n");
	}

	let ty: u16 = read_le_bytes(offset, ident_remain,);
	let machine: u16 = read_le_bytes(offset, ident_remain,);
	let version: u32 = read_le_bytes(offset, ident_remain,);
	let entry: u64 = read_le_bytes(offset, ident_remain,);
	let program_header_offset: u64 = read_le_bytes(offset, ident_remain,);
	let section_header_offset: u64 = read_le_bytes(offset, ident_remain,);
	let flags: u32 = read_le_bytes(offset, ident_remain,);
	let elf_header_size: u16 = read_le_bytes(offset, ident_remain,);
	let program_header_entry_size: u16 = read_le_bytes(offset, ident_remain,);
	let program_header_count: u16 = read_le_bytes(offset, ident_remain,);
	let section_header_entry_size: u16 = read_le_bytes(offset, ident_remain,);
	let section_header_count: u16 = read_le_bytes(offset, ident_remain,);
	let section_header_index_of_section_name_string_table: u16 =
		read_le_bytes(offset, ident_remain,);

	let ty = ElfType::try_from(ty,)?;

	Ok(ElfHeader {
		ident,
		ty,
		machine,
		version,
		entry,
		program_header_offset,
		section_header_offset,
		flags,
		elf_header_size,
		program_header_entry_size,
		program_header_count,
		section_header_entry_size,
		section_header_count,
		section_header_index_of_section_name_string_table,
	},)
}

//fn read_le_bytes<I: PrimitiveInteger + Sum<<I as Shl<usize,>>::Output,>,>(
fn read_le_bytes<I: PrimitiveInteger,>(offset: &mut usize, binary: &[u8],) -> I
where for<'a> &'a [u8]: AsInt<I,> {
	//let window = &binary[*offset..*offset + size];
	// let val =
	// 	window.iter().enumerate().map(|(i, b,)| Integer::<I,>::cast_int(*b,) << i * 8,).sum::<I>();

	let val = (&binary[*offset..]).as_int();
	*offset += size_of::<I,>();
	val
}

#[derive(PartialEq, Eq,)]
pub enum ElfType {
	None,
	Relocatable,
	Executable,
	SharedObject,
	Core,
	NumberOfDefined,
	OsSpecificRangeStart,
	OsSpecificRangeEnd,
	ProcessorSpecificRangeStart,
	ProcessorSpecificRangeEnd,
}

impl ElfType {
	fn is_lib(&self,) -> bool {
		*self == Self::SharedObject
	}
}

impl TryFrom<u16,> for ElfType {
	type Error = OsoLoaderError;

	fn try_from(value: u16,) -> Result<Self, Self::Error,> {
		let ty = match value {
			0 => Self::None,
			1 => Self::Relocatable,
			2 => Self::Executable,
			3 => Self::SharedObject,
			4 => Self::Core,
			5 => Self::NumberOfDefined,
			0xfe00 => Self::OsSpecificRangeStart,
			0xfeff => Self::OsSpecificRangeEnd,
			0xff00 => Self::ProcessorSpecificRangeStart,
			0xffff => Self::OsSpecificRangeEnd,
			_ => return Err(OsoLoaderError::EfiParse(format!("unknown type: {value}"),),),
		};
		Ok(ty,)
	}
}

pub struct ElfHeaderIdent {
	pub file_class:    FileClass,
	pub endianness:    Endian,
	pub elf_version:   ElfVersion,
	pub target_os_abi: TargetOsAbi,
	pub abi_version:   AbiVersion,
}

impl ElfHeaderIdent {
	fn new(ident: &[u8],) -> Rslt<Self,> {
		if ident.len() != ELF_IDENT_SIZE {
			return Err(OsoLoaderError::EfiParse(format!(
				"ident len is 16, but given ident len is {}",
				ident.len(),
			),),);
		}

		// check magic number
		// size of elf magic number is 4
		if &ident[0..4] != ELF_MAGIC_NUMBER {
			return Err(OsoLoaderError::EfiParse(format!("bad magic number: {:?}", &ident[0..4]),),);
		}

		let file_class = FileClass::try_from(ident[ELF_FILE_CLASS_INDEX],)?;
		let endianness = Endian::try_from(ident[ELF_ENDIANNESS_INDEX],)?;
		let elf_version = ElfVersion(ident[ELF_VERSION_INDEX],);
		let target_os_abi = TargetOsAbi::try_from(ident[ELF_OS_ABI_INDEX],)?;
		let abi_version = AbiVersion(ident[ELF_ABI_VERSION_INDEX],);

		Ok(Self { file_class, endianness, elf_version, target_os_abi, abi_version, },)
	}

	fn is_64(&self,) -> bool {
		self.file_class.is_64()
	}

	fn is_little_endian(&self,) -> bool {
		self.endianness.is_little_endian()
	}
}

#[derive(PartialEq, Eq,)]
pub enum FileClass {
	Bit32,
	Bit64,
}

impl FileClass {
	fn is_64(&self,) -> bool {
		*self == FileClass::Bit64
	}
}

impl TryFrom<u8,> for FileClass {
	type Error = OsoLoaderError;

	fn try_from(value: u8,) -> Result<Self, Self::Error,> {
		match value {
			ELF_32_BIT_OBJECT => Ok(Self::Bit32,),
			ELF_64_BIT_OBJECT => Ok(Self::Bit64,),
			_ => Err(OsoLoaderError::EfiParse(format!("invalid file class: {}", value),),),
		}
	}
}

pub struct ElfVersion(pub u8,);

impl ElfVersion {
	pub const ONE: Self = Self(1,);
}

#[non_exhaustive]
pub enum TargetOsAbi {
	SysV,
	Arm,
	Standalone,
}

impl TryFrom<u8,> for TargetOsAbi {
	type Error = OsoLoaderError;

	fn try_from(value: u8,) -> Result<Self, Self::Error,> {
		match value {
			0x0 => Ok(Self::SysV,),
			0x53 => Ok(Self::Arm,),
			0x61 => Ok(Self::Standalone,),
			_ => Err(OsoLoaderError::EfiParse(format!(
				"target os abi value is invalid or unsupported: {}",
				value
			),),),
		}
	}
}

pub struct AbiVersion(pub u8,);
impl AbiVersion {
	pub const ONE: Self = Self(0,);
}

pub struct StringTable {
	pub delimitor: StringContext,
	pub bytes:     Vec<u8,>,
	pub strings:   Vec<(usize, String,),>,
}

pub enum StringContext {
	Delimiter(u8,),
	DelimiterUntil(u8, usize,),
	Length(usize,),
}

pub struct SymbolTable {
	pub bytes: Vec<u8,>,
	pub count: usize,
	pub ctx:   Context,
	pub start: usize,
	pub end:   usize,
}

pub struct Context {
	pub container: Container,
	pub le:        Endian,
}

/// the size of a binary container
pub enum Container {
	Little,
	Big,
}

#[derive(PartialEq, Eq,)]
pub enum Endian {
	Little,
	Big,
}

impl Endian {
	fn is_little_endian(&self,) -> bool {
		*self == Self::Little
	}
}

impl TryFrom<u8,> for Endian {
	type Error = OsoLoaderError;

	fn try_from(value: u8,) -> Result<Self, Self::Error,> {
		match value {
			1 => Ok(Self::Little,),
			2 => Ok(Self::Big,),
			_ => {
				Err(OsoLoaderError::EfiParse(format!("invalid endianness flag value: {}", value),),)
			},
		}
	}
}

pub struct Dynamic {
	pub dyns: Vec<Dyn,>,
	pub info: DynamicInfo,
}

pub struct Dyn {
	pub tag: u64,
	pub val: u64,
}

pub struct DynamicInfo {
	/// An addend is an extra constant value used in a relocation to help compute the correct final
	/// address. It adjusts the value that gets written into the relocated memory.
	pub relocation_addend:                 usize,
	pub relocation_addend_size:            usize,
	pub relocation_addend_entry:           u64,
	pub relocation_addend_entry_count:     usize,
	pub relocation_addend_section_address: usize,
	pub relocation_size:                   usize,
	pub relocation_entry:                  u64,
	pub relocation_entry_count:            usize,
	pub gnu_hash:                          Option<u64,>,
	pub hash:                              Option<u64,>,
	pub string_table_address:              usize,
	pub string_table_size:                 usize,
	pub symbol_table:                      usize,
	pub symbol_table_entry:                usize,
	pub plt_got_address:                   Option<u64,>,
	pub plt_relocation_size:               usize,
	pub plt_relocation_type:               u64,
	pub jmp_relocation_address:            usize,
	pub virsion_definition_table_address:  u64,
	pub version_definition_count:          u64,
	pub version_need_table_address:        u64,
	pub version_need_count:                u64,
	pub version_symbol_table_address:      u64,
	pub init_fn_address:                   u64,
	pub finalization_fn_address:           u64,
	pub init_fn_array_address:             u64,
	pub init_fn_array_len:                 usize,
	pub finalization_fn_array_address:     u64,
	pub finalization_fn_array_len:         usize,
	pub required_shared_lib_count:         usize,
	pub flags:                             u64,
	pub extended_flags:                    u64,
	pub shared_object_name_offset:         usize,
	pub text_section_relocation:           bool,
}

pub struct RelocationSection {
	pub bytes:   Vec<u8,>,
	pub count:   usize,
	pub context: RelocationContext,
	pub start:   usize,
	pub end:     usize,
}

pub type RelocationContext = (bool, Context,);

pub struct SymbolVersionSection {
	pub bytes:   Vec<u8,>,
	pub context: Context,
}

pub struct VersionDefinitionSection {
	pub bytes:   Vec<u8,>,
	pub count:   usize,
	pub context: Context,
}

pub struct VersionNeededSection {
	pub bytes:   Vec<u8,>,
	pub count:   usize,
	pub context: Context,
}

trait Integer<T: PrimitiveInteger,>:
	Add
	+ AddAssign
	+ Sub
	+ SubAssign
	+ Mul
	+ MulAssign
	+ Div
	+ DivAssign
	+ Shl
	+ Shr
	+ Clone
	+ Sum
	+ Sized
{
	fn cast_int(self,) -> T;
}

trait PrimitiveInteger:
	Add
	+ AddAssign
	+ Sub
	+ SubAssign
	+ Mul
	+ MulAssign
	+ Div
	+ DivAssign
	+ Shl<usize, Output: Sum,>
	+ Shr
	+ Clone
	+ Sum
	+ Sized
{
}

impl PrimitiveInteger for u8 {}
impl PrimitiveInteger for u16 {}
impl PrimitiveInteger for u32 {}
impl PrimitiveInteger for u64 {}
impl PrimitiveInteger for u128 {}
impl PrimitiveInteger for usize {}

impl Integer<u8,> for u8 {
	fn cast_int(self,) -> u8 {
		self
	}
}

impl Integer<u16,> for u8 {
	fn cast_int(self,) -> u16 {
		self as u16
	}
}

impl Integer<u32,> for u8 {
	fn cast_int(self,) -> u32 {
		self as u32
	}
}

impl Integer<u64,> for u8 {
	fn cast_int(self,) -> u64 {
		self as u64
	}
}

impl Integer<u128,> for u8 {
	fn cast_int(self,) -> u128 {
		self as u128
	}
}

impl Integer<usize,> for u8 {
	fn cast_int(self,) -> usize {
		self as usize
	}
}

trait AsInt<T: PrimitiveInteger,> {
	fn as_int(&self,) -> T;
}

//  TODO: add trait constraint which describe relation betwen uXX primitive type and [u8; N]
impl AsInt<u8,> for &[u8] {
	fn as_int(&self,) -> u8 {
		let bytes = &self[..1];
		unsafe { *(&self[..1] as *const _ as *const u8) }
	}
}

impl AsInt<u16,> for &[u8] {
	fn as_int(&self,) -> u16 {
		unsafe { *(&self[..2] as *const _ as *const u16) }
	}
}

impl AsInt<u32,> for &[u8] {
	fn as_int(&self,) -> u32 {
		unsafe { *(&self[..4] as *const _ as *const u32) }
	}
}

impl AsInt<u64,> for &[u8] {
	fn as_int(&self,) -> u64 {
		unsafe { *(&self[..8] as *const _ as *const u64) }
	}
}

impl AsInt<u128,> for &[u8] {
	fn as_int(&self,) -> u128 {
		unsafe { *(&self[..16] as *const _ as *const u128) }
	}
}

impl AsInt<usize,> for &[u8] {
	fn as_int(&self,) -> usize {
		unsafe { *(&self[..8] as *const _ as *const usize) }
	}
}
