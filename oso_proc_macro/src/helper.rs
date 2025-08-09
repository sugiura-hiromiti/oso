//! this module is intended to handle error. separated from macro's main function makes codebase and
//! responsibility division clear.

use oso_proc_macro_logic::derive_from_pathbuf_for_crate::enum_impl;
use oso_proc_macro_logic::derive_from_pathbuf_for_crate::struct_impl;
use oso_proc_macro_logic::status_from_spec::StatusCode;
use oso_proc_macro_logic::status_from_spec::StatusCodeInfo;
use oso_proc_macro_logic::test_elf_header_parse::ReadElfH;
use oso_proc_macro_logic::test_elf_header_parse::readelf_h;
use oso_proc_macro_logic::test_program_headers_parse::ReadElfL;
use oso_proc_macro_logic::test_program_headers_parse::readelf_l;
use proc_macro::Diagnostic;
use proc_macro::Level;
use proc_macro2::Span;

/// Generates the implementation block for the UEFI Status struct.
///
/// This function takes parsed status code information from the UEFI specification
/// and generates a complete implementation block including associated constants
/// for all status codes and error handling methods.
///
/// # Parameters
///
/// * `spec_page` - Parsed status code information from the UEFI specification
///
/// # Returns
///
/// Returns a `proc_macro2::TokenStream` containing the complete implementation
/// block for the Status struct, including:
/// - Associated constants for success, warning, and error status codes
/// - `ok_or()` method for converting status to Result
/// - `ok_or_with()` method for custom error handling
///
/// # Generated Methods
///
/// - `ok_or()`: Converts the status to a Result, returning Ok for success/warning status codes and
///   Err for error status codes
/// - `ok_or_with()`: Similar to ok_or but allows custom transformation of success values
pub fn impl_status(spec_page: &StatusCode,) -> proc_macro2::TokenStream {
	// Generate token parts for success status codes (non-error)
	let (success_match, success_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.success.token_parts(false,).into_iter().unzip();

	// Generate token parts for warning status codes (non-error)
	let (warn_match, warn_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.warn.token_parts(false,).into_iter().unzip();

	// Generate token parts for error status codes (error)
	let (error_match, error_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.error.token_parts(true,).into_iter().unzip();

	quote::quote! {
		impl Status {
			// Associated constants for all status codes
			#(#success_assoc)*
			#(#warn_assoc)*
			#(#error_assoc)*

			/// Converts the status to a Result type.
			///
			/// Returns Ok(Self) for success and warning status codes,
			/// and Err(UefiError) for error status codes.
			pub fn ok_or(self) -> Rslt<Self, oso_error::loader::UefiError> {
				use alloc::string::ToString;
				match self {
					// Success status codes return Ok
					#(#success_match)*
					// Warning status codes return Ok
					#(#warn_match)*
					// Error status codes return Err
					#(#error_match)*
					// Unknown status codes return custom error
					Self(code) => Err(oso_error::oso_err!(oso_error::loader::UefiError::CustomStatus)),
				}
			}

			/// Converts the status to a Result with custom transformation.
			///
			/// Similar to ok_or(), but allows applying a transformation function
			/// to the success value before returning.
			pub fn ok_or_with<T>(self, with: impl FnOnce(Self) -> T) -> Rslt<T, oso_error::loader::UefiError> {
				let status = self.ok_or()?;
				Ok(with(status))
			}
		}
	}
}

/// Trait for converting status code information into token stream parts.
///
/// This trait provides a method to convert status code information into
/// the token stream components needed for generating match arms and
/// associated constants in the Status implementation.
trait TokenParts {
	/// Converts status code information into token stream parts.
	///
	/// # Parameters
	///
	/// * `is_err` - Whether these status codes represent error conditions
	///
	/// # Returns
	///
	/// Returns a vector of tuples where each tuple contains:
	/// - Match arm token stream for the ok_or() method
	/// - Associated constant token stream for the Status impl
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),>;
}

/// Implementation of TokenParts for vectors of StatusCodeInfo.
///
/// This implementation processes each status code in the vector and generates
/// the appropriate token streams for both match arms and associated constants.
impl TokenParts for Vec<StatusCodeInfo,> {
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),> {
		self.iter()
			.map(|sci| {
				// Create identifier from the status code mnemonic
				let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);

				// Create literal from the status code value
				let value =
					syn::Lit::Int(syn::LitInt::new(&format!("{}", sci.value), Span::call_site(),),);

				// Generate appropriate match arm based on error status
				let match_arms =
					if is_err { err_match(&mnemonic, &sci.desc,) } else { ok_match(&mnemonic,) };

				// Generate associated constant with documentation
				let assoc = assoc_const(&mnemonic, &value, &sci.desc,);

				(match_arms, assoc,)
			},)
			.collect()
	}
}

/// Generates a match arm for successful (non-error) status codes.
///
/// Creates a match arm that returns `Ok(Self::MNEMONIC)` for the given status code.
/// This is used for success and warning status codes in the `ok_or()` method.
///
/// # Parameters
///
/// * `mnemonic` - The identifier for the status code constant
///
/// # Returns
///
/// Returns a token stream representing a match arm that returns Ok
fn ok_match(mnemonic: &syn::Ident,) -> proc_macro2::TokenStream {
	quote::quote! {
		Self::#mnemonic => Ok(Self::#mnemonic,),
	}
}

/// Generates a match arm for error status codes.
///
/// Creates a match arm that returns an error with the status code description.
/// This is used for error status codes in the `ok_or()` method.
///
/// # Parameters
///
/// * `mnemonic` - The identifier for the status code constant
/// * `msg` - The description message for the error
///
/// # Returns
///
/// Returns a token stream representing a match arm that returns an error
fn err_match(mnemonic: &syn::Ident, msg: &String,) -> proc_macro2::TokenStream {
	let mnemonic_str = mnemonic.to_string();
	quote::quote! {
	Self::#mnemonic => {
		let mut mnemonic = concat!(#mnemonic_str, ": ", #msg);
		Err(oso_error::oso_err!(UefiError::ErrorStatus(mnemonic)))
	},
	}
}

/// Generates an associated constant for a status code.
///
/// Creates an associated constant with documentation derived from the status
/// code description. The constant has the same name as the mnemonic and
/// contains the numeric value of the status code.
///
/// # Parameters
///
/// * `mnemonic` - The identifier for the status code constant
/// * `value` - The numeric value of the status code
/// * `msg` - The description to use as documentation
///
/// # Returns
///
/// Returns a token stream representing an associated constant with documentation
fn assoc_const(mnemonic: &syn::Ident, value: &syn::Lit, msg: &String,) -> proc_macro2::TokenStream {
	quote::quote! {
		#[doc = #msg]
		pub const #mnemonic: Self = Self(#value);
	}
}

/// Generates token stream for expected ELF header information.
///
/// This function executes `readelf -h` on the target binary and parses the output
/// to create a token stream representing the expected ELF header structure. This
/// is used by the `test_elf_header_parse` macro to validate ELF header parsing.
///
/// # Returns
///
/// Returns a `proc_macro2::TokenStream` representing an `ElfHeader` struct
/// initialization with all fields populated from the actual binary.
///
/// # Generated Structure
///
/// The generated token stream creates an ElfHeader with:
/// - `ident`: ELF identification information (class, endianness, version, etc.)
/// - `ty`: ELF file type (executable, shared object, etc.)
/// - `machine`: Target machine architecture
/// - `version`: ELF version
/// - `entry`: Entry point address
/// - `program_header_offset`: Offset to program header table
/// - `section_header_offset`: Offset to section header table
/// - `flags`: Processor-specific flags
/// - `elf_header_size`: Size of ELF header
/// - `program_header_entry_size`: Size of program header entries
/// - `program_header_count`: Number of program header entries
/// - `section_header_entry_size`: Size of section header entries
/// - `section_header_count`: Number of section header entries
/// - `section_header_index_of_section_name_string_table`: Index of section name string table
///
/// # Panics
///
/// This function will panic if:
/// - The `readelf -h` command fails to execute
/// - The readelf output cannot be parsed
/// - Any required ELF header field is missing or malformed
///
/// # Dependencies
///
/// Requires the `readelf` command to be available in the system PATH.
pub fn elf_header_info() -> proc_macro2::TokenStream {
	// Execute readelf -h and parse the output
	let header = &match readelf_h() {
		Ok(r,) => r,
		Err(e,) => {
			Diagnostic::new(Level::Error, format!("failed to get `readelf -h` result: {e}"),)
				.emit();
			panic!("{}", module_path!())
		},
	};

	// Parse individual ELF header components
	let ident = elf_header_ident_build(header,);
	let ty = parse_ty(header,);
	let machine = parse_machine(header,);
	let version = parse_version(header,);
	let entry = parse_entry(header,);
	let program_header_offset = parse_program_header_offset(header,);
	let section_header_offset = parse_section_header_offset(header,);
	let flags = parse_flags(header,);
	let elf_header_size = parse_elf_header_size(header,);
	let program_header_entry_size = parse_program_header_entry_size(header,);
	let program_header_count = parse_program_header_count(header,);
	let section_header_entry_size = parse_section_header_entry_size(header,);
	let section_header_count = parse_section_header_count(header,);
	let section_header_index_of_section_name_string_table =
		parse_section_header_index_of_section_name_string_table(header,);

	// Generate the complete ElfHeader struct initialization
	quote::quote! {
		ElfHeader {
			ident: #ident,
			ty : #ty,
			machine : #machine,
			version : #version,
			entry : #entry,
			program_header_offset : #program_header_offset,
			section_header_offset : #section_header_offset,
			flags : #flags,
			elf_header_size : #elf_header_size,
			program_header_entry_size : #program_header_entry_size,
			program_header_count : #program_header_count,
			section_header_entry_size : #section_header_entry_size,
			section_header_count : #section_header_count,
			section_header_index_of_section_name_string_table : #section_header_index_of_section_name_string_table,
		}
	}
}

/// Builds the ELF header identification structure.
///
/// This function parses the ELF identification fields from the readelf output
/// and generates a token stream representing the `ElfHeaderIdent` structure.
/// The ELF identification contains metadata about the ELF file format.
///
/// # Parameters
///
/// * `header` - Parsed readelf -h output containing ELF header information
///
/// # Returns
///
/// Returns a token stream representing an `ElfHeaderIdent` struct initialization
/// with all identification fields populated.
///
/// # ELF Identification Fields
///
/// - `file_class`: Whether the file is 32-bit or 64-bit
/// - `endianness`: Byte order (little-endian or big-endian)
/// - `elf_version`: ELF format version
/// - `target_os_abi`: Target operating system ABI
/// - `abi_version`: ABI version number
fn elf_header_ident_build(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let file_class = parse_file_class(header,);
	let endianness = parse_endianness(header,);
	let elf_version = parse_elf_version(header,);
	let target_os_abi = parse_target_os_abi(header,);
	let abi_version = parse_abi_version(header,);

	quote::quote! {
		ElfHeaderIdent {
			file_class: #file_class,
			endianness: #endianness,
			elf_version: #elf_version,
			target_os_abi: #target_os_abi,
			abi_version: #abi_version,
		}
	}
}

/// Parses the ELF file class from readelf output.
///
/// Converts the file class string from readelf into the appropriate enum variant.
/// The file class indicates whether the ELF file is 32-bit or 64-bit.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing file class information
///
/// # Returns
///
/// Returns a token stream representing the FileClass enum variant
///
/// # Supported Values
///
/// - "ELF64" -> `FileClass::Bit64`
/// - "ELF32" -> `FileClass::Bit32`
///
/// # Panics
///
/// Panics if the file class is not recognized
fn parse_file_class(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let file_class = header.file_class.as_str();

	let file_class = match file_class {
		"ELF64" => quote::quote! {
			FileClass::Bit64
		},
		"ELF32" => quote::quote! {
			FileClass::Bit32
		},
		_ => {
			Diagnostic::new(Level::Error, format!("failed to parse file_class info: {file_class}"),)
				.emit();
			panic!()
		},
	};

	file_class
}

/// Parses the endianness from readelf output.
///
/// Converts the endianness string into the appropriate enum variant.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing endianness information
///
/// # Returns
///
/// Returns a token stream representing the Endian enum variant
///
/// # Supported Values
///
/// - "little" -> `Endian::Little`
/// - "big" -> `Endian::Big`
///
/// # Panics
///
/// Panics if the endianness is not recognized
fn parse_endianness(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let endianness = header.endianness.as_str();

	let endianness = match endianness {
		"little" => quote::quote! {
			Endian::Little
		},
		"big" => quote::quote! {
			Endian::Big
		},
		_ => {
			Diagnostic::new(Level::Error, format!("failed to parse endianness info: {endianness}"),)
				.emit();
			panic!()
		},
	};

	endianness
}

/// Parses the ELF version from readelf output.
///
/// Converts the ELF version string into the appropriate constant or creates
/// a new version variant for unrecognized versions.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing ELF version information
///
/// # Returns
///
/// Returns a token stream representing the ElfVersion constant or variant
///
/// # Behavior
///
/// - Version "1" maps to `ElfVersion::ONE`
/// - Other versions generate a warning and create `ElfVersion(n)` variant
fn parse_elf_version(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let elf_version = header.elf_version.as_str();

	let elf_version = match elf_version {
		"1" => quote::quote! {
			ElfVersion::ONE
		},
		ver => {
			Diagnostic::new(Level::Warning, format!("unrecognized elf version: {elf_version}"),)
				.emit();
			let ver: u8 = ver.parse().expect(&format!("elf version must be valid integer: {ver}"),);
			quote::quote! {
				ElfVersion(#ver)
			}
		},
	};

	elf_version
}

/// Parses the target OS ABI from readelf output.
///
/// Converts the target OS ABI string into the appropriate enum variant.
/// The target OS ABI indicates the operating system and ABI for which
/// the ELF file was created.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing target OS ABI information
///
/// # Returns
///
/// Returns a token stream representing the TargetOsAbi enum variant
///
/// # Supported Values
///
/// - Contains "UNIX - System V" -> `TargetOsAbi::SysV`
/// - Contains "Arm" -> `TargetOsAbi::Arm`
/// - Contains "Standalone" -> `TargetOsAbi::Standalone`
///
/// # Panics
///
/// Panics if the target OS ABI is not recognized
fn parse_target_os_abi(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let target_os_abi = header.target_os_abi.as_str();

	let target_os_abi = if target_os_abi.contains("UNIX - System V",) {
		quote::quote! {
		TargetOsAbi::SysV
			}
	} else if target_os_abi.contains("Arm",) {
		quote::quote! {
			TargetOsAbi::Arm
		}
	} else if target_os_abi.contains("Standalone",) {
		quote::quote! {
			TargetOsAbi::Standalone
		}
	} else {
		Diagnostic::new(Level::Error, format!("unrecognized target_os_abi : {target_os_abi}"),)
			.emit();
		unreachable!()
	};

	target_os_abi
}

/// Parses the ABI version from readelf output.
///
/// Converts the ABI version string into the appropriate constant or creates
/// a new version variant for unrecognized versions.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing ABI version information
///
/// # Returns
///
/// Returns a token stream representing the AbiVersion constant or variant
///
/// # Behavior
///
/// - Version "1" maps to `AbiVersion::ONE`
/// - Other versions generate a warning and create `AbiVersion(n)` variant
fn parse_abi_version(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let abi_version = header.abi_version.as_str();

	let abi_version = match abi_version {
		"1" => quote::quote! {
			AbiVersion::ONE
		},
		ver => {
			Diagnostic::new(Level::Warning, format!("unrecognized abi version: {abi_version}"),)
				.emit();
			let ver: u8 = ver.parse().expect(&format!("abi version must be valid integer: {ver}"),);
			quote::quote! {
				AbiVersion(#ver)
			}
		},
	};

	abi_version
}

/// Parses the ELF file type from readelf output.
///
/// Converts the ELF type string into the appropriate enum variant.
/// For the OSO kernel, this function specifically validates that the
/// file type is executable.
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing ELF type information
///
/// # Returns
///
/// Returns a token stream representing the ElfType enum variant
///
/// # Behavior
///
/// - Only "EXEC" (executable) type is supported for OSO kernel
/// - Other types will cause a compile-time error
///
/// # Panics
///
/// Panics if the ELF type is not "EXEC" (executable)
fn parse_ty(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let ty = header.ty.as_str();

	if ty != "EXEC" {
		Diagnostic::new(Level::Error, &format!("oso_kernel.elf type must be executable: {ty}"),)
			.emit();
		panic!();
	}

	quote::quote! {
		ElfType::Executable
	}
}

/// Parses the target machine architecture from readelf output.
///
/// Converts the machine string into the appropriate ELF machine constant.
/// The function normalizes the machine name by converting to uppercase
/// and replacing spaces with underscores, then prefixes with "EM_".
///
/// # Parameters
///
/// * `header` - Parsed readelf output containing machine information
///
/// # Returns
///
/// Returns a token stream representing the machine constant
///
/// # Examples
///
/// - "Advanced Micro Devices X86-64" -> `ElfHeader::EM_ADVANCED_MICRO_DEVICES_X86-64`
/// - "AArch64" -> `ElfHeader::EM_AARCH64`
fn parse_machine(header: &ReadElfH,) -> proc_macro2::TokenStream {
	// Normalize machine name: uppercase and replace spaces with underscores
	let machine: String = header
		.machine
		.as_str()
		.chars()
		.map(|c| match c {
			cap if 'a' <= cap && 'z' >= cap => (cap as u8 + b'A' - b'a') as char,
			space if space == ' ' => '_',
			_ => c,
		},)
		.collect();

	// Create the machine constant identifier
	let mut machine_const = "EM_".to_string();
	machine_const.push_str(&machine,);
	let machine = syn::Ident::new(&machine_const, Span::call_site(),);

	quote::quote! {
		ElfHeader::#machine
	}
}

/// Parses the ELF version field (different from ELF format version).
/// Expects a hexadecimal string prefixed with "0x".
fn parse_version(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let version = header.version.as_str();
	let version = &version[2..]; // Remove "0x" prefix
	let version = u32::from_str_radix(version, 16,)
		.expect(&format!("version must be valid hex number: {version}",),);

	quote::quote! {
		#version
	}
}

/// Parses the entry point address from readelf output.
/// Expects a hexadecimal string prefixed with "0x".
fn parse_entry(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let entry = header.entry.as_str();
	let entry = &entry[2..]; // Remove "0x" prefix
	let entry = u64::from_str_radix(entry, 16,)
		.expect(&format!("entry point address must be valid hex number: {entry}",),);

	quote::quote! {
		#entry
	}
}

/// Parses the program header table offset from readelf output.
/// Expects a decimal string.
fn parse_program_header_offset(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_offset = header.program_header_offset.as_str();
	let program_header_offset = u64::from_str_radix(program_header_offset, 10,).expect(&format!(
		"program_header_offset address must be valid hex number: {program_header_offset}",
	),);

	quote::quote! {
		#program_header_offset
	}
}

/// Parses the section header table offset from readelf output.
/// Expects a decimal string.
fn parse_section_header_offset(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_offset = header.section_header_offset.as_str();
	let section_header_offset = u64::from_str_radix(section_header_offset, 10,).expect(&format!(
		"section_header_offset address must be valid hex number: {section_header_offset}",
	),);

	quote::quote! {
		#section_header_offset
	}
}

/// Parses processor-specific flags from readelf output.
/// Expects a hexadecimal string prefixed with "0x".
fn parse_flags(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let flags = header.flags.as_str();
	let flags = &flags[2..]; // Remove "0x" prefix
	let flags = u32::from_str_radix(flags, 16,)
		.expect(&format!("flags must be valid hex number: {flags}",),);

	quote::quote! {
		#flags
	}
}

/// Parses the ELF header size from readelf output.
/// Expects a decimal string.
fn parse_elf_header_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let elf_header_size = header.elf_header_size.as_str();
	let elf_header_size = u16::from_str_radix(elf_header_size, 10,)
		.expect(&format!("elf_header_size must be valid hex number: {elf_header_size}",),);

	quote::quote! {
		#elf_header_size
	}
}

/// Parses the program header entry size from readelf output.
/// Expects a decimal string.
fn parse_program_header_entry_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_entry_size = header.program_header_entry_size.as_str();
	let program_header_entry_size = u16::from_str_radix(program_header_entry_size, 10,).expect(
		&format!("program_header_entry_size must be valid hex number: {program_header_entry_size}",),
	);

	quote::quote! {
		#program_header_entry_size
	}
}

/// Parses the number of program header entries from readelf output.
/// Expects a decimal string.
fn parse_program_header_count(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_count = header.program_header_count.as_str();
	let program_header_count = u16::from_str_radix(program_header_count, 10,)
		.expect(&format!("program_header_count must be valid hex number: {program_header_count}",),);

	quote::quote! {
		#program_header_count
	}
}

/// Parses the section header entry size from readelf output.
/// Expects a decimal string.
fn parse_section_header_entry_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_entry_size = header.section_header_entry_size.as_str();
	let section_header_entry_size = u16::from_str_radix(section_header_entry_size, 10,).expect(
		&format!("section_header_entry_size must be valid hex number: {section_header_entry_size}",),
	);

	quote::quote! {
		#section_header_entry_size
	}
}

/// Parses the number of section header entries from readelf output.
/// Expects a decimal string.
fn parse_section_header_count(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_count = header.section_header_count.as_str();
	let section_header_count = u16::from_str_radix(section_header_count, 10,)
		.expect(&format!("section_header_count must be valid hex number: {section_header_count}",),);

	quote::quote! {
		#section_header_count
	}
}

/// Parses the section header string table index from readelf output.
/// This index points to the section containing section names.
/// Expects a decimal string.
fn parse_section_header_index_of_section_name_string_table(
	header: &ReadElfH,
) -> proc_macro2::TokenStream {
	let section_header_index_of_section_name_string_table =
		header.section_header_index_of_section_name_string_table.as_str();
	let section_header_index_of_section_name_string_table =
		u16::from_str_radix(section_header_index_of_section_name_string_table, 10,).expect(
			&format!(
				"section_header_index_of_section_name_string_table must be valid hex number: \
				 {section_header_index_of_section_name_string_table}",
			),
		);

	quote::quote! {
		#section_header_index_of_section_name_string_table
	}
}

/// Generates token stream for expected program headers information.
///
/// This function executes `readelf -l` on the target binary and parses the output
/// to create a token stream representing the expected program headers structure.
/// This is used by the `test_program_headers_parse` macro to validate program
/// header parsing.
///
/// # Returns
///
/// Returns a `proc_macro2::TokenStream` representing a vector of `ProgramHeader`
/// structs, each initialized with data from the actual binary.
///
/// # Generated Structure
///
/// Each ProgramHeader contains:
/// - `ty`: Program header type (LOAD, DYNAMIC, INTERP, etc.)
/// - `flags`: Permission flags (read, write, execute)
/// - `offset`: Offset in file where segment begins
/// - `virtual_address`: Virtual address where segment should be loaded
/// - `physical_address`: Physical address (for systems where relevant)
/// - `file_size`: Size of segment in file
/// - `memory_size`: Size of segment in memory (may be larger than file_size)
/// - `align`: Alignment requirements for the segment
///
/// # Panics
///
/// This function will panic if:
/// - The `readelf -l` command fails to execute
/// - The readelf output cannot be parsed
/// - Any required program header field is missing or malformed
///
/// # Dependencies
///
/// Requires the `readelf` command to be available in the system PATH.
pub fn program_headers_info() -> proc_macro2::TokenStream {
	// Execute readelf -l and parse the output
	let program_headers = match readelf_l() {
		Ok(r,) => r,
		Err(e,) => {
			Diagnostic::new(Level::Error, format!("failed to get `readelf -l` result: {e}"),)
				.emit();
			panic!("{}", module_path!())
		},
	};

	// Generate ProgramHeader struct for each program header entry
	let program_headers = program_headers.iter().map(|rel| {
		let ty = parse_program_header_type(rel,);
		let flags = rel.flags;
		let offset = rel.offset;
		let virtual_address = rel.virtual_address;
		let physical_address = rel.physical_address;
		let file_size = rel.file_size;
		let memory_size = rel.memory_size;
		let align = rel.align;

		quote::quote! {
			ProgramHeader {
				ty: #ty,
				flags: #flags,
				offset: #offset,
				virtual_address: #virtual_address,
				physical_address: #physical_address,
				file_size: #file_size,
				memory_size: #memory_size,
				align: #align,
			}
		}
	},);

	// Generate vector containing all program headers
	quote::quote! {
		alloc::vec![
			#(#program_headers, )*
		]
	}
}

/// Parses program header type from readelf output.
///
/// Converts the program header type string into the appropriate enum variant.
/// The function converts underscore-separated uppercase strings into CamelCase
/// identifiers for the ProgramHeaderType enum.
///
/// # Parameters
///
/// * `program_header` - Parsed readelf -l output for a single program header
///
/// # Returns
///
/// Returns a token stream representing the ProgramHeaderType enum variant
///
/// # Conversion Logic
///
/// The function:
/// 1. Splits the type string on underscores
/// 2. Converts each word to CamelCase (first letter uppercase, rest lowercase)
/// 3. Concatenates the words to form the enum variant name
///
/// # Examples
///
/// - "PT_LOAD" -> `ProgramHeaderType::PtLoad`
/// - "PT_DYNAMIC" -> `ProgramHeaderType::PtDynamic`
/// - "PT_INTERP" -> `ProgramHeaderType::PtInterp`
fn parse_program_header_type(program_header: &ReadElfL,) -> proc_macro2::TokenStream {
	// Convert underscore_separated to CamelCase
	let camel_cased: String = program_header
		.ty
		.split("_",)
		.flat_map(|word| {
			word.char_indices()
				.map(|(i, c,)| if i == 0 { c } else { (c as u8 - b'A' + b'a') as char },)
		},)
		.collect();

	let ident = syn::Ident::new(&camel_cased, Span::call_site(),);

	quote::quote! {
		ProgramHeaderType::#ident
	}
}

pub fn from_pathbuf_helper(
	attr: proc_macro2::TokenStream,
	item: syn::Item,
) -> proc_macro2::TokenStream {
	let rslt = match item {
		syn::Item::Enum(item_enum,) => enum_impl(item_enum,),
		syn::Item::Struct(item_struct,) => struct_impl(item_struct,),
		_ => unreachable!(),
	};

	match rslt {
		Ok(r,) => r,
		Err(e,) => {
			Diagnostic::new(Level::Error, e,).emit();
			panic!()
		},
	}
}
