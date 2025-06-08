use oso_proc_macro_logic::status_from_spec::StatusCode;
use oso_proc_macro_logic::status_from_spec::StatusCodeInfo;
use oso_proc_macro_logic::test_elf_header_parse::ReadElfH;
use oso_proc_macro_logic::test_elf_header_parse::readelf_h;
use proc_macro::Diagnostic;
use proc_macro::Level;
use proc_macro2::Span;

pub fn impl_status(spec_page: &StatusCode,) -> proc_macro2::TokenStream {
	let (success_match, success_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.success.token_parts(false,).into_iter().unzip();
	let (warn_match, warn_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.warn.token_parts(false,).into_iter().unzip();
	let (error_match, error_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.error.token_parts(true,).into_iter().unzip();

	quote::quote! {
		impl Status {
			#(#success_assoc)*
			#(#warn_assoc)*
			#(#error_assoc)*

			pub fn ok_or(self) -> Rslt<Self,> {
				use alloc::string::ToString;
				match self {
					#(#success_match)*
					#(#warn_match)*
					#(#error_match)*
					Self(code) => Err(OsoLoaderError::Uefi("vendor custom error code".to_string())),
				}
			}

			pub fn ok_or_with<T>(self, with: impl FnOnce(Self)->T) -> Rslt<T> {
				let status = self.ok_or()?;
				Ok(with(status))
			}
		}
	}
}

trait TokenParts {
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),>;
}

impl TokenParts for Vec<StatusCodeInfo,> {
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),> {
		self.iter()
			.map(|sci| {
				let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);
				let value =
					syn::Lit::Int(syn::LitInt::new(&format!("{}", sci.value), Span::call_site(),),);
				let match_arms =
					if is_err { err_match(&mnemonic, &sci.desc,) } else { ok_match(&mnemonic,) };
				let assoc = assoc_const(&mnemonic, &value, &sci.desc,);
				(match_arms, assoc,)
			},)
			.collect()
	}
}

fn ok_match(mnemonic: &syn::Ident,) -> proc_macro2::TokenStream {
	quote::quote! {
		Self::#mnemonic => Ok(Self::#mnemonic,),
	}
}

fn err_match(mnemonic: &syn::Ident, msg: &String,) -> proc_macro2::TokenStream {
	let mnemonic_str = mnemonic.to_string();
	quote::quote! {
	Self::#mnemonic => {
		let mut mnemonic = #mnemonic_str.to_string();
		mnemonic.push_str(": ");
		mnemonic.push_str(#msg);
		Err(OsoLoaderError::Uefi(mnemonic))
	},
	}
}

fn assoc_const(mnemonic: &syn::Ident, value: &syn::Lit, msg: &String,) -> proc_macro2::TokenStream {
	quote::quote! {
		#[doc = #msg]
		pub const #mnemonic: Self = Self(#value);
	}
}

pub fn elf_header_info() -> proc_macro2::TokenStream {
	let header = &match readelf_h() {
		Ok(r,) => r,
		Err(e,) => {
			Diagnostic::new(Level::Error, format!("failed to get `readelf -h` result: {e}"),)
				.emit();
			panic!("{}", module_path!())
		},
	};

	Diagnostic::new(Level::Note, format!("header:\n{header:#?}"),).emit();

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

fn parse_machine(header: &ReadElfH,) -> proc_macro2::TokenStream {
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

	let mut machine_const = "EM_".to_string();
	machine_const.push_str(&machine,);
	let machine = syn::Ident::new(&machine_const, Span::call_site(),);

	quote::quote! {
		ElfHeader::#machine
	}
}

fn parse_version(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let version = header.version.as_str();
	let version = &version[2..];
	let version = u32::from_str_radix(version, 16,)
		.expect(&format!("version must be valid hex number: {version}",),);

	quote::quote! {
		#version
	}
}

fn parse_entry(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let entry = header.entry.as_str();
	let entry = &entry[2..];
	let entry = u64::from_str_radix(entry, 16,)
		.expect(&format!("entry point address must be valid hex number: {entry}",),);

	quote::quote! {
		#entry
	}
}

fn parse_program_header_offset(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_offset = header.program_header_offset.as_str();
	let program_header_offset = u64::from_str_radix(program_header_offset, 10,).expect(&format!(
		"program_header_offset address must be valid hex number: {program_header_offset}",
	),);

	quote::quote! {
		#program_header_offset
	}
}

fn parse_section_header_offset(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_offset = header.section_header_offset.as_str();
	let section_header_offset = u64::from_str_radix(section_header_offset, 10,).expect(&format!(
		"section_header_offset address must be valid hex number: {section_header_offset}",
	),);

	quote::quote! {
		#section_header_offset
	}
}

fn parse_flags(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let flags = header.flags.as_str();
	let flags = &flags[2..];
	let flags = u32::from_str_radix(flags, 16,)
		.expect(&format!("flags must be valid hex number: {flags}",),);

	quote::quote! {
		#flags
	}
}

fn parse_elf_header_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let elf_header_size = header.elf_header_size.as_str();
	let elf_header_size = u16::from_str_radix(elf_header_size, 10,)
		.expect(&format!("elf_header_size must be valid hex number: {elf_header_size}",),);

	quote::quote! {
		#elf_header_size
	}
}

fn parse_program_header_entry_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_entry_size = header.program_header_entry_size.as_str();
	let program_header_entry_size = u16::from_str_radix(program_header_entry_size, 10,).expect(
		&format!("program_header_entry_size must be valid hex number: {program_header_entry_size}",),
	);

	quote::quote! {
		#program_header_entry_size
	}
}

fn parse_program_header_count(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let program_header_count = header.program_header_count.as_str();
	let program_header_count = u16::from_str_radix(program_header_count, 10,)
		.expect(&format!("program_header_count must be valid hex number: {program_header_count}",),);

	quote::quote! {
		#program_header_count
	}
}

fn parse_section_header_entry_size(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_entry_size = header.section_header_entry_size.as_str();
	let section_header_entry_size = u16::from_str_radix(section_header_entry_size, 10,).expect(
		&format!("section_header_entry_size must be valid hex number: {section_header_entry_size}",),
	);

	quote::quote! {
		#section_header_entry_size
	}
}

fn parse_section_header_count(header: &ReadElfH,) -> proc_macro2::TokenStream {
	let section_header_count = header.section_header_count.as_str();
	let section_header_count = u16::from_str_radix(section_header_count, 10,)
		.expect(&format!("section_header_count must be valid hex number: {section_header_count}",),);

	quote::quote! {
		#section_header_count
	}
}

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

pub fn program_headers_info() {}
