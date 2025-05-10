use crate::Rslt;
use crate::elf::program_header::ProgramHeader;
use crate::elf::section_header::SectionHeader;
use crate::error::OsoLoaderError;
use crate::print;
use crate::println;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
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
use core::usize;
use program_header::ProgramHeaderType;
use section_header::SHT_SYMTAB;
use section_header::get_string_table;

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

				interpreter = Some(StringContext::Length(count,).read_bytes(&binary[offset..],),);
			}
		}

		offset = header.section_header_offset as usize;
		let section_headers =
			SectionHeader::parse(binary, &mut offset, header.section_header_count as usize,)?;

		let string_table_index = header.section_header_index_of_section_name_string_table as usize;
		let section_header_string_table =
			get_string_table(&section_headers, string_table_index, &binary,)?;

		let mut symbol_table = SymbolTable::default();
		let mut string_table = StringTable::default();
		if let Some(section_header,) =
			section_headers.iter().rfind(|section_header| section_header.ty as u32 == SHT_SYMTAB,)
		{
			let size = section_header.entry_size;
			let count = if size == 0 { 0 } else { section_header.size / size };
			let context = Context::default();
			symbol_table = SymbolTable::parse(
				binary,
				section_header.offset as usize,
				count as usize,
				context,
			)?;
		}

		let mut is_position_independent_executable = false;
		let mut shared_object_name = None;
		let mut libraries = vec![];
		let mut runtime_search_path_deprecated = vec![];
		let mut runtime_search_path = vec![];
		let mut dynamic_symbol_table = SymbolTable::default();
		let mut dynamic_relocation_with_addend = RelocationSection::default();
		let mut dynamic_relocation = RelocationSection::default();
		let mut procedure_linkage_table_relocation = RelocationSection::default();
		let mut dynamic_string_table = StringTable::default();

		let dynamic = Dynamic::parse(binary, &program_headers,)?;
		if let Some(ref dynamic,) = dynamic {
			let dyn_info = &dynamic.info;
			is_position_independent_executable =
				dyn_info.extended_flags & Dynamic::DF_EXTEND_PIE != 0;
			dynamic_string_table = StringTable::parse(
				binary,
				dyn_info.string_table_address,
				dyn_info.string_table_size,
				0x0,
			)?;

			if dyn_info.shared_object_name_offset != 0 {
				shared_object_name = dynamic_string_table
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

impl Default for StringTable {
	fn default() -> Self {
		Self { delimitor: StringContext::default(), bytes: vec![], strings: vec![], }
	}
}

impl StringTable {
	/// # Params
	///
	/// - bytes
	///
	/// bytes expected to be entire elf file
	pub fn parse(binary: &Vec<u8,>, offset: usize, len: usize, delimiter: u8,) -> Rslt<Self,> {
		let (end, overflow,) = offset.overflowing_add(len,);
		if overflow || end > binary.len() {
			return Err(OsoLoaderError::EfiParse(format!(
				"string table size ({}) + offset ({}) is out of bounds for {} bytes. overflowed: \
				 {}",
				len,
				offset,
				binary.len(),
				overflow
			),),);
		}

		let mut rslt = Self::from_slice(&binary[offset..offset + len], delimiter,);
		let mut i = 0;
		while i < rslt.bytes.len() {
			let s = rslt.delimitor.read_bytes(&binary[i..],)?;
			let len = s.len();
			rslt.strings.push((i, s,),);
			i += len + 1;
		}

		Ok(rslt,)
	}

	fn from_slice(bytes: &[u8], delimiter: u8,) -> Self {
		Self {
			delimitor: StringContext::Delimiter(delimiter,),
			bytes:     bytes.to_vec(),
			strings:   vec![],
		}
	}
}

pub enum StringContext {
	Delimiter(u8,),
	DelimiterUntil(u8, usize,),
	Length(usize,),
}

impl StringContext {
	fn read_bytes(&self, bytes: &[u8],) -> Rslt<String,> {
		let bytes = match self {
			StringContext::Delimiter(delimiter,) => {
				let mut i = 0;
				while let a = &bytes[i..=i]
					&& a != &[*delimiter,]
				{
					i += 1;
					if i >= bytes.len() {
						return Err(OsoLoaderError::EfiParse(format!(
							"delimiter: {delimiter} not found"
						),),);
					}
				}

				&bytes[..i]
			},
			StringContext::DelimiterUntil(..,) => todo!(),
			StringContext::Length(l,) => &bytes[..*l],
		};

		// TODO: check encoding is actually utf8
		let rslt = String::from_utf8_lossy(bytes,);
		Ok(rslt.to_string(),)
	}
}

impl Default for StringContext {
	fn default() -> Self {
		// null delimiter
		Self::Delimiter(0,)
	}
}

#[derive(Default,)]
pub struct SymbolTable {
	pub bytes: Vec<u8,>,
	pub count: usize,
	pub ctx:   Context,
	pub start: usize,
	pub end:   usize,
}

impl SymbolTable {
	/// size of symbol structure in 64bit.
	const SIZE_OF_SYMBOL_64: usize = 4 + 1 + 1 + 2 + 8 + 8;

	fn parse(binary: &Vec<u8,>, offset: usize, count: usize, context: Context,) -> Rslt<Self,> {
		let size = count
			.checked_mul(match context.container {
				Container::Little => todo!(),
				Container::Big => Self::SIZE_OF_SYMBOL_64,
			},)
			.ok_or_else(|| {
				OsoLoaderError::EfiParse(format!(
					"too many elf symbols offset: {offset:#x}, count {count}"
				),)
			},)?;

		let bytes = binary[offset..offset + size].to_vec();

		Ok(SymbolTable { bytes, count, ctx: context, start: offset, end: offset + size, },)
	}
}

pub struct Context {
	pub container: Container,
	pub le:        Endian,
}

impl Default for Context {
	fn default() -> Self {
		Self { container: Container::default(), le: Endian::default(), }
	}
}

/// the size of a binary container
pub enum Container {
	Little,
	Big,
}

impl Default for Container {
	fn default() -> Self {
		// TODO: add conditional compilation current implementation only support 64bit pointer width
		Self::Big
	}
}

#[derive(PartialEq, Eq,)]
pub enum Endian {
	Little,
	Big,
}

impl Default for Endian {
	fn default() -> Self {
		// TODO: add conditional compilation current implementation only support 64bit pointer width
		Self::Big
	}
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

impl Dynamic {
	/// No lazy binding for this object.
	pub const DF_BIND_NOW: u64 = 0x0000_0008;
	/// Configuration alternative created.
	pub const DF_EXTEND_CONFALT: u64 = 0x0000_2000;
	/// Direct binding enabled.
	pub const DF_EXTEND_DIRECT: u64 = 0x0000_0100;
	/// Disp reloc applied at build time.
	pub const DF_EXTEND_DISPRELDNE: u64 = 0x0000_8000;
	/// Disp reloc applied at run-time.
	pub const DF_EXTEND_DISPRELPND: u64 = 0x0001_0000;
	/// Object is modified after built.
	pub const DF_EXTEND_EDITED: u64 = 0x0020_0000;
	/// Filtee terminates filters search.
	pub const DF_EXTEND_ENDFILTEE: u64 = 0x0000_4000;
	/// Set RTLD_GLOBAL for this object.
	pub const DF_EXTEND_GLOBAL: u64 = 0x0000_0002;
	/// Global auditing required.
	pub const DF_EXTEND_GLOBAUDIT: u64 = 0x0100_0000;
	/// Set RTLD_GROUP for this object.
	pub const DF_EXTEND_GROUP: u64 = 0x0000_0004;
	pub const DF_EXTEND_IGNMULDEF: u64 = 0x0004_0000;
	/// Set RTLD_INITFIRST for this object.
	pub const DF_EXTEND_INITFIRST: u64 = 0x0000_0020;
	/// Object is used to interpose.
	pub const DF_EXTEND_INTERPOSE: u64 = 0x0000_0400;
	/// Trigger filtee loading at runtime.
	pub const DF_EXTEND_LOADFLTR: u64 = 0x0000_0010;
	/// Ignore default lib search path.
	pub const DF_EXTEND_NODEFLIB: u64 = 0x0000_0800;
	/// Set RTLD_NODELETE for this object.
	pub const DF_EXTEND_NODELETE: u64 = 0x0000_0008;
	/// Object has no-direct binding.
	pub const DF_EXTEND_NODIRECT: u64 = 0x0002_0000;
	/// Object can't be dldump'ed.
	pub const DF_EXTEND_NODUMP: u64 = 0x0000_1000;
	pub const DF_EXTEND_NOHDR: u64 = 0x0010_0000;
	pub const DF_EXTEND_NOKSYMS: u64 = 0x0008_0000;
	/// Set RTLD_NOOPEN for this object.
	pub const DF_EXTEND_NOOPEN: u64 = 0x0000_0040;
	pub const DF_EXTEND_NORELOC: u64 = 0x0040_0000;
	/// === State flags ===
	/// selectable in the `d_un.d_val` element of the DT_FLAGS_1 entry in the dynamic section.
	///
	/// Set RTLD_NOW for this object.
	pub const DF_EXTEND_NOW: u64 = 0x0000_0001;
	/// $ORIGIN must be handled.
	pub const DF_EXTEND_ORIGIN: u64 = 0x0000_0080;
	/// Object is a Position Independent Executable (PIE).
	pub const DF_EXTEND_PIE: u64 = 0x0800_0000;
	/// Singleton dyn are used.
	pub const DF_EXTEND_SINGLETON: u64 = 0x0200_0000;
	/// Object has individual interposers.
	pub const DF_EXTEND_SYMINTPOSE: u64 = 0x0080_0000;
	pub const DF_EXTEND_TRANS: u64 = 0x0000_0200;
	// Values of `d_un.d_val` in the DT_FLAGS entry
	/// Object may use DF_ORIGIN.
	pub const DF_ORIGIN: u64 = 0x0000_0001;
	/// Module uses the static TLS model.
	pub const DF_STATIC_TLS: u64 = 0x0000_0010;
	/// Symbol resolutions starts here.
	pub const DF_SYMBOLIC: u64 = 0x0000_0002;
	/// Object contains text relocations.
	pub const DF_TEXTREL: u64 = 0x0000_0004;
	//DT_ADDRTAGIDX(tag)	(DT_ADDRRNGHI - (tag))	/* Reverse order! */
	pub const DT_ADDRNUM: u64 = 11;
	///
	pub const DT_ADDRRNGHI: u64 = 0x6fff_feff;
	/// DT_* entries which fall between DT_ADDRRNGHI & DT_ADDRRNGLO use the
	/// Dyn.d_un.d_ptr field of the Elf*_Dyn structure.
	///
	/// If any adjustment is made to the ELF object after it has been
	/// built these entries will need to be adjusted.
	pub const DT_ADDRRNGLO: u64 = 0x6fff_fe00;
	/// Object auditing
	pub const DT_AUDIT: u64 = 0x6fff_fefc;
	/// Process relocations of object
	pub const DT_BIND_NOW: u64 = 24;
	/// Configuration information
	pub const DT_CONFIG: u64 = 0x6fff_fefa;
	/// For debugging; unspecified
	pub const DT_DEBUG: u64 = 21;
	/// Dependency auditing
	pub const DT_DEPAUDIT: u64 = 0x6fff_fefb;
	/// Start of encoded range
	pub const DT_ENCODING: u64 = 32;
	/// Address of termination function
	pub const DT_FINI: u64 = 13;
	/// Array with addresses of fini fct
	pub const DT_FINI_ARRAY: u64 = 26;
	/// Size in bytes of DT_FINI_ARRAY
	pub const DT_FINI_ARRAYSZ: u64 = 28;
	/// Flags for the object being loaded
	pub const DT_FLAGS: u64 = 30;
	/// State flags, see DF_1_* below
	pub const DT_FLAGS_1: u64 = 0x6fff_fffb;
	/// Start of conflict section
	pub const DT_GNU_CONFLICT: u64 = 0x6fff_fef8;
	/// GNU-style hash table
	pub const DT_GNU_HASH: u64 = 0x6fff_fef5;
	/// Library list
	pub const DT_GNU_LIBLIST: u64 = 0x6fff_fef9;
	/// Address of symbol hash table
	pub const DT_HASH: u64 = 4;
	/// End of OS-specific
	pub const DT_HIOS: u64 = 0x6fff_f000;
	/// End of processor-specific
	pub const DT_HIPROC: u64 = 0x7fff_ffff;
	/// Address of init function
	pub const DT_INIT: u64 = 12;
	/// Array with addresses of init fct
	pub const DT_INIT_ARRAY: u64 = 25;
	/// Size in bytes of DT_INIT_ARRAY
	pub const DT_INIT_ARRAYSZ: u64 = 27;
	/// Address of PLT relocs
	pub const DT_JMPREL: u64 = 23;
	/// Start of OS-specific
	pub const DT_LOOS: u64 = 0x6000_000d;
	/// Start of processor-specific
	pub const DT_LOPROC: u64 = 0x7000_0000;
	/// Move table
	pub const DT_MOVETAB: u64 = 0x6fff_fefe;
	/// Name of needed library
	pub const DT_NEEDED: u64 = 1;
	/// Marks end of dynamic section
	pub const DT_NULL: u64 = 0;
	/// Number used
	pub const DT_NUM: u64 = 34;
	/// Processor defined value
	pub const DT_PLTGOT: u64 = 3;
	/// PLT padding
	pub const DT_PLTPAD: u64 = 0x6fff_fefd;
	/// Type of reloc in PLT
	pub const DT_PLTREL: u64 = 20;
	/// Size in bytes of PLT relocs
	pub const DT_PLTRELSZ: u64 = 2;
	/// Array with addresses of preinit fct
	pub const DT_PREINIT_ARRAY: u64 = 32;
	/// size in bytes of DT_PREINIT_ARRAY
	pub const DT_PREINIT_ARRAYSZ: u64 = 33;
	/// Address of Rel relocs
	pub const DT_REL: u64 = 17;
	/// Address of Rela relocs
	pub const DT_RELA: u64 = 7;
	pub const DT_RELACOUNT: u64 = 0x6fff_fff9;
	/// Size of one Rela reloc
	pub const DT_RELAENT: u64 = 9;
	/// Total size of Rela relocs
	pub const DT_RELASZ: u64 = 8;
	pub const DT_RELCOUNT: u64 = 0x6fff_fffa;
	/// Size of one Rel reloc
	pub const DT_RELENT: u64 = 19;
	/// Total size of Rel relocs
	pub const DT_RELSZ: u64 = 18;
	/// Library search path (deprecated)
	pub const DT_RPATH: u64 = 15;
	/// Library search path
	pub const DT_RUNPATH: u64 = 29;
	/// Name of shared object
	pub const DT_SONAME: u64 = 14;
	/// Size of string table
	pub const DT_STRSZ: u64 = 10;
	/// Address of string table
	pub const DT_STRTAB: u64 = 5;
	/// Start symbol search here
	pub const DT_SYMBOLIC: u64 = 16;
	/// Size of one symbol table entry
	pub const DT_SYMENT: u64 = 11;
	/// Syminfo table
	pub const DT_SYMINFO: u64 = 0x6fff_feff;
	/// Address of symbol table
	pub const DT_SYMTAB: u64 = 6;
	/// Reloc might modify .text
	pub const DT_TEXTREL: u64 = 22;
	///
	pub const DT_TLSDESC_GOT: u64 = 0x6fff_fef7;
	///
	pub const DT_TLSDESC_PLT: u64 = 0x6fff_fef6;
	/// Address of version definition table
	pub const DT_VERDEF: u64 = 0x6fff_fffc;
	/// Number of version definitions
	pub const DT_VERDEFNUM: u64 = 0x6fff_fffd;
	/// Address of table with needed versions
	pub const DT_VERNEED: u64 = 0x6fff_fffe;
	/// Number of needed versions
	pub const DT_VERNEEDNUM: u64 = 0x6fff_ffff;
	/// The versioning entry types. The next are defined as part of the GNU extension
	pub const DT_VERSYM: u64 = 0x6fff_fff0;

	fn parse(binary: &Vec<u8,>, program_headers: &Vec<ProgramHeader,>,) -> Rslt<Option<Self,>,> {
		for program_header in program_headers {
			if program_header.ty == ProgramHeaderType::Dynamic {
				let offset = program_header.offset as usize;
				let file_size = program_header.file_size as usize;
				let bytes = if file_size > 0 { &binary[offset..offset + file_size] } else { &[] };
				let size =
					Dyn::size_of(&Context { container: Container::Big, ..Default::default() },);
				let count = file_size / size;
				let mut dyns = Vec::with_capacity(count,);
				let offset = &mut 0;
				for _ in 0..count {
					let dynamic = Dyn::parse(bytes, offset,);
					let tag = dynamic.tag;
					dyns.push(dynamic,);
					if tag == Self::DT_NULL {
						break;
					}
				}

				let mut info = DynamicInfo::default();
				for dynamic in &dyns {
					info.update(&program_headers, dynamic,);
				}

				return Ok(Some(Dynamic { dyns, info, },),);
			}
		}

		Ok(None,)
	}
}

pub struct Dyn {
	pub tag: u64,
	pub val: u64,
}

impl Dyn {
	const SIZE_OF_DYN_32: usize = 16;
	const SIZE_OF_DYN_64: usize = 8;

	fn size_of(Context { container, .. }: &Context,) -> usize {
		match container {
			Container::Little => todo!(),
			Container::Big => Self::SIZE_OF_DYN_64,
		}
	}

	fn parse(bytes: &[u8], offset: &mut usize,) -> Self {
		let tag = read_le_bytes(offset, bytes,);
		let val = read_le_bytes(offset, bytes,);
		Self { tag, val, }
	}
}

#[derive(Default,)]
pub struct DynamicInfo {
	/// An addend is an extra constant value used in a relocation to help compute the correct final
	/// address. It adjusts the value that gets written into the relocated memory.
	pub relocation_addend:                 usize,
	pub relocation_addend_size:            usize,
	pub relocation_addend_entry:           u64,
	pub relocation_addend_entry_count:     usize,
	pub relocation_addend_section_address: usize,
	pub relocation:                        usize,
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

impl DynamicInfo {
	pub fn update(&mut self, phdrs: &[ProgramHeader], dynamic: &Dyn,) {
		match u64::from(dynamic.tag,) {
			Dynamic::DT_RELA => {
				self.relocation_addend = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,) as usize
			}, // .rela.dyn
			Dynamic::DT_RELASZ => self.relocation_addend_size = dynamic.val as usize,
			Dynamic::DT_RELAENT => self.relocation_addend_entry = dynamic.val,
			Dynamic::DT_RELACOUNT => self.relocation_addend_entry_count = dynamic.val as usize,
			Dynamic::DT_REL => {
				self.relocation = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,) as usize
			}, /* .rel.dyn */
			Dynamic::DT_RELSZ => self.relocation_size = dynamic.val as usize,
			Dynamic::DT_RELENT => self.relocation_entry = dynamic.val,
			Dynamic::DT_RELCOUNT => self.relocation_entry_count = dynamic.val as usize,
			Dynamic::DT_GNU_HASH => self.gnu_hash = vm_to_offset(phdrs, dynamic.val,),
			Dynamic::DT_HASH => self.hash = vm_to_offset(phdrs, dynamic.val,),
			Dynamic::DT_STRTAB => {
				self.string_table_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,) as usize
			},
			Dynamic::DT_STRSZ => self.string_table_size = dynamic.val as usize,
			Dynamic::DT_SYMTAB => {
				self.symbol_table = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,) as usize
			},
			Dynamic::DT_SYMENT => self.symbol_table_entry = dynamic.val as usize,
			Dynamic::DT_PLTGOT => self.plt_got_address = vm_to_offset(phdrs, dynamic.val,),
			Dynamic::DT_PLTRELSZ => self.plt_relocation_size = dynamic.val as usize,
			Dynamic::DT_PLTREL => self.plt_relocation_type = dynamic.val,
			Dynamic::DT_JMPREL => {
				self.jmp_relocation_address =
					vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,) as usize
			}, /* .rela.plt */
			Dynamic::DT_VERDEF => {
				self.version_definition_count = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_VERDEFNUM => {
				self.version_definition_count = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_VERNEED => {
				self.version_need_table_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_VERNEEDNUM => self.version_need_count = dynamic.val,
			Dynamic::DT_VERSYM => {
				self.version_symbol_table_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_INIT => {
				self.init_fn_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_FINI => {
				self.finalization_fn_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_INIT_ARRAY => {
				self.init_fn_array_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_INIT_ARRAYSZ => self.init_fn_array_len = dynamic.val as usize,
			Dynamic::DT_FINI_ARRAY => {
				self.finalization_fn_array_address = vm_to_offset(phdrs, dynamic.val,).unwrap_or(0,)
			},
			Dynamic::DT_FINI_ARRAYSZ => self.finalization_fn_array_len = dynamic.val as usize,
			Dynamic::DT_NEEDED => self.version_need_count += 1,
			Dynamic::DT_FLAGS => self.flags = dynamic.val,
			Dynamic::DT_FLAGS_1 => self.extended_flags = dynamic.val,
			Dynamic::DT_SONAME => self.shared_object_name_offset = dynamic.val as usize,
			Dynamic::DT_TEXTREL => self.text_section_relocation = true,
			_ => (),
		}
	}
}

fn vm_to_offset(program_headers: &[ProgramHeader], address: u64,) -> Option<u64,> {
	for program_header in program_headers {
		if program_header.ty == ProgramHeaderType::Load && address >= program_header.virtual_address
		{
			let offset = address - program_header.virtual_address;
			if offset < program_header.memory_size {
				return program_header.offset.checked_add(offset,);
			}
		}
	}
	None
}

#[derive(Default,)]
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
