use crate::Rslt;
use crate::error::OsoLoaderError;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

const ELF_MAGIC_NUMBER_SIZE: usize = 4;
/// used to check magic number
const ELF_MAGIC_NUMBER: &[u8; ELF_MAGIC_NUMBER_SIZE] = b"\x7felf";
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
	pub soname:                             Option<String,>,
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
		todo!(
			"
			pub trait Pread<Ctx: Copy, E>

			TryFromCtx<'a, Ctx, Self, Error = E>
			"
		)
	}
}

pub struct ElfHeader {
	pub ident: [u8; ELF_IDENT_SIZE],
	pub ty: u16,
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

		// check magic number
		// size of elf magic number is 4
		if &ident[0..4] != ELF_MAGIC_NUMBER {
			return Err(OsoLoaderError::EfiParse(format!("bad magic number: {:?}", &ident[0..4]),),);
		}

		let file_class = ident[ELF_FILE_CLASS_INDEX];
		let endianness = ident[ELF_ENDIANNESS_INDEX];

		match file_class {
			ELF_32_BIT_OBJECT => todo!(),
			ELF_64_BIT_OBJECT => todo!(),
			_ => Err(OsoLoaderError::EfiParse(format!("invalid elf class {:x}", file_class),),),
		}
	}
}

pub struct ElfHeaderIdent {
	file_class:    FileClass,
	endianness:    Endian,
	elf_version:   ElfVersion,
	target_os_abi: TargetOsAbi,
	abi_version:   AbiVersion,
}

pub enum FileClass {
	Bit32,
	Bit64,
}

pub struct ElfVersion(pub u8,);

impl ElfVersion {
	const ONE: Self = Self(0,);
}

#[non_exhaustive]
pub enum TargetOsAbi {
	SysV,
	Arm,
	Standalone,
}

pub struct AbiVersion(pub u8);
impl AbiVersion {
    const ONE:Self=Self(0);
}

pub struct ProgramHeader {
	pub ty:               u32,
	pub flags:            u32,
	pub offset:           u64,
	pub virtual_addres:   u64,
	pub physical_address: u64,
	pub file_size:        u64,
	pub memory_size:      u64,
	pub aligh:            u64,
}

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

pub struct StringTable {
	delimitor: StringContext,
	bytes:     Vec<u8,>,
	strings:   Vec<(usize, String,),>,
}

pub enum StringContext {
	Delimiter(u8,),
	DelimiterUntil(u8, usize,),
	Length(usize,),
}

pub struct SymbolTable {
	bytes: Vec<u8,>,
	count: usize,
	ctx:   Context,
	start: usize,
	end:   usize,
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

pub enum Endian {
	Little,
	Big,
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
	bytes:   Vec<u8,>,
	count:   usize,
	context: RelocationContext,
	start:   usize,
	end:     usize,
}

pub type RelocationContext = (bool, Context,);

pub struct SymbolVersionSection {
	bytes:   Vec<u8,>,
	context: Context,
}

pub struct VersionDefinitionSection {
	bytes:   Vec<u8,>,
	count:   usize,
	context: Context,
}

pub struct VersionNeededSection {
	bytes:   Vec<u8,>,
	count:   usize,
	context: Context,
}

pub fn parse_elf(content: Vec<u8,>,) {}
