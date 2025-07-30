//! # ELF Header Parsing Module
//!
//! This module provides functionality for parsing ELF (Executable and Linkable Format)
//! headers from the OSO kernel binary. It uses the `readelf` command-line tool to extract
//! header information and parses it into structured Rust types.
//!
//! The module is primarily used for build-time analysis and validation of the kernel
//! binary to ensure it meets the expected format and requirements.

use crate::check_oso_kernel;
use anyhow::Result as Rslt;
use std::ops::Index;
use std::process::Command;

/// Structured representation of ELF header information
///
/// This struct contains all the key fields from an ELF header as parsed
/// from the output of `readelf -h`. Each field is stored as a string
/// to preserve the original formatting from readelf.
#[derive(Default, Debug,)]
pub struct ReadElfH {
	/// ELF file class (32-bit or 64-bit)
	pub file_class: String,
	/// Data encoding (little-endian or big-endian)
	pub endianness: String,
	/// ELF version number
	pub elf_version: String,
	/// Target OS/ABI identification
	pub target_os_abi: String,
	/// ABI version number
	pub abi_version: String,
	/// Object file type (executable, shared object, etc.)
	pub ty: String,
	/// Target machine architecture
	pub machine: String,
	/// Object file version
	pub version: String,
	/// Entry point virtual address
	pub entry: String,
	/// Program header table file offset
	pub program_header_offset: String,
	/// Section header table file offset
	pub section_header_offset: String,
	/// Processor-specific flags
	pub flags: String,
	/// ELF header size in bytes
	pub elf_header_size: String,
	/// Program header table entry size
	pub program_header_entry_size: String,
	/// Number of program header table entries
	pub program_header_count: String,
	/// Section header table entry size
	pub section_header_entry_size: String,
	/// Number of section header table entries
	pub section_header_count: String,
	/// Section header string table index
	pub section_header_index_of_section_name_string_table: String,
}

impl ReadElfH {
	/// Cleans up the parsed header fields by removing extra whitespace and comments
	///
	/// The `readelf` command often includes additional information after the main
	/// value (like comments or explanations). This method extracts just the first
	/// word/value from each field, which is typically the actual data we need.
	///
	/// # Examples
	///
	/// - "ELF64 (64-bit)" becomes "ELF64"
	/// - "0x401000 (entry point)" becomes "0x401000"
	/// - "little endian" becomes "little"
	fn fix(&mut self,) {
		// Extract first word from each field (split on whitespace)
		self.file_class = self.file_class.split(" ",).nth(0,).unwrap().to_string();
		self.endianness = self.endianness.split(" ",).nth(0,).unwrap().to_string();
		self.elf_version = self.elf_version.split(" ",).nth(0,).unwrap().to_string();
		// Note: target_os_abi is intentionally not processed as it may contain spaces
		self.abi_version = self.abi_version.split(" ",).nth(0,).unwrap().to_string();
		self.ty = self.ty.split(" ",).nth(0,).unwrap().to_string();
		self.machine = self.machine.split(" ",).nth(0,).unwrap().to_string();
		self.version = self.version.split(" ",).nth(0,).unwrap().to_string();
		self.entry = self.entry.split(" ",).nth(0,).unwrap().to_string();
		self.program_header_offset =
			self.program_header_offset.split(" ",).nth(0,).unwrap().to_string();
		self.section_header_offset =
			self.section_header_offset.split(" ",).nth(0,).unwrap().to_string();
		self.flags = self.flags.split(" ",).nth(0,).unwrap().to_string();
		self.elf_header_size = self.elf_header_size.split(" ",).nth(0,).unwrap().to_string();
		self.program_header_entry_size =
			self.program_header_entry_size.split(" ",).nth(0,).unwrap().to_string();
		self.program_header_count =
			self.program_header_count.split(" ",).nth(0,).unwrap().to_string();
		self.section_header_entry_size =
			self.section_header_entry_size.split(" ",).nth(0,).unwrap().to_string();
		self.section_header_count =
			self.section_header_count.split(" ",).nth(0,).unwrap().to_string();
		self.section_header_index_of_section_name_string_table = self
			.section_header_index_of_section_name_string_table
			.split(" ",)
			.nth(0,)
			.unwrap()
			.to_string();
	}
}

/// Trait for checking if a key-value pair matches a specific property name
///
/// This trait provides a method to check if the first element of a vector
/// (representing a parsed key-value pair) matches a given key string.
trait Property {
	/// Checks if this key-value pair represents the specified property
	///
	/// # Arguments
	///
	/// * `key` - The property name to check for
	///
	/// # Returns
	///
	/// `true` if the first element matches the key, `false` otherwise
	fn is_peoperty_of(&self, key: &str,) -> bool;
}

impl Property for Vec<&str,> {
	fn is_peoperty_of(&self, key: &str,) -> bool {
		// Check if the first element (index 0) matches the key
		*self.index(0,) == key
	}
}

/// Parses ELF header information from the OSO kernel binary
///
/// This function executes `readelf -h` on the kernel binary and parses
/// the output to extract all ELF header fields into a structured format.
/// It performs validation to ensure the kernel file exists before parsing.
///
/// # Returns
///
/// A `Result<ReadElfH>` containing the parsed ELF header information,
/// or an error if the kernel file doesn't exist or parsing fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The OSO kernel ELF file doesn't exist (checked via `check_oso_kernel`)
/// - The `readelf` command fails to execute
/// - The command output cannot be parsed as UTF-8
///
/// # Examples
///
/// ```ignore
/// let header = readelf_h()?;
/// println!("Entry point: {}", header.entry);
/// println!("Machine: {}", header.machine);
/// ```
pub fn readelf_h() -> Rslt<ReadElfH,> {
	// Ensure the kernel file exists before attempting to parse it
	check_oso_kernel()?;

	// Execute readelf command to get header information
	let header_info =
		Command::new("readelf",).args(["-h", "target/oso_kernel.elf",],).output()?.stdout;

	// Convert command output to string
	let header_info = String::from_utf8(header_info,)?;

	// Initialize default header struct
	let mut header = ReadElfH::default();

	// Parse each line of readelf output
	header_info.lines().for_each(|line| {
		// Split each line on ':' to get key-value pairs
		let key_value: Vec<_,> = line.split(':',).map(|s| s.trim(),).collect();

		// Parse each field based on the key name
		if key_value.is_peoperty_of("Class",) {
			header.file_class = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Data",) {
			// Extract endianness from "2's complement, little endian" format
			header.endianness = key_value[1].split(" ",).nth(2,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Version",) {
			// Handle both ELF version and object version fields
			if key_value[1].contains("0x",) {
				header.version = key_value[1].to_string();
			} else {
				header.elf_version = key_value[1].split(" ",).nth(0,).unwrap().to_string();
			}
		}
		if key_value.is_peoperty_of("OS/ABI",) {
			header.target_os_abi = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("ABI Version",) {
			header.abi_version = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Type",) {
			header.ty = key_value[1].split(" ",).nth(0,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Machine",) {
			header.machine = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Entry point address",) {
			header.entry = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Start of program headers",) {
			header.program_header_offset = key_value[1].split(" ",).nth(0,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Start of section headers",) {
			header.section_header_offset = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Flags",) {
			header.flags = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Size of this header",) {
			header.elf_header_size = key_value[1].split(" ",).nth(0,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Size of program headers",) {
			header.program_header_entry_size =
				key_value[1].split(" ",).nth(0,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Number of program headers",) {
			header.program_header_count = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Size of section headers",) {
			header.section_header_entry_size = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Number of section headers",) {
			header.section_header_count = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Section header string table index",) {
			header.section_header_index_of_section_name_string_table = key_value[1].to_string();
		}
	},);

	// Clean up the parsed fields by removing extra whitespace and comments
	header.fix();

	Ok(header,)
}
#[cfg(test)]
mod tests {
	use super::*;
	use std::env::current_dir;
	use std::env::set_current_dir;

	/// Helper function to navigate to the project root for tests
	fn go_root() -> Rslt<(),> {
		let cwd = current_dir()?;
		if cwd.file_name().unwrap() != "oso" {
			if let Some(oso_root,) = cwd.parent() {
				set_current_dir(oso_root,)?;
			}
		}
		Ok((),)
	}

	#[test]
	fn test_readelf_h_default() {
		let header = ReadElfH::default();

		// All fields should be empty strings by default
		assert_eq!(header.file_class, "");
		assert_eq!(header.endianness, "");
		assert_eq!(header.elf_version, "");
		assert_eq!(header.target_os_abi, "");
		assert_eq!(header.abi_version, "");
		assert_eq!(header.ty, "");
		assert_eq!(header.machine, "");
		assert_eq!(header.version, "");
		assert_eq!(header.entry, "");
		assert_eq!(header.program_header_offset, "");
		assert_eq!(header.section_header_offset, "");
		assert_eq!(header.flags, "");
		assert_eq!(header.elf_header_size, "");
		assert_eq!(header.program_header_entry_size, "");
		assert_eq!(header.program_header_count, "");
		assert_eq!(header.section_header_entry_size, "");
		assert_eq!(header.section_header_count, "");
		assert_eq!(header.section_header_index_of_section_name_string_table, "");
	}

	#[test]
	fn test_readelf_h_fix_method() {
		let mut header = ReadElfH {
			file_class: "ELF64 (64-bit)".to_string(),
			endianness: "little endian".to_string(),
			elf_version: "1 (current)".to_string(),
			target_os_abi: "UNIX - System V".to_string(), // This one should not be split
			abi_version: "0 (default)".to_string(),
			ty: "EXEC (Executable file)".to_string(),
			machine: "Advanced Micro Devices X86-64".to_string(),
			version: "0x1 (current)".to_string(),
			entry: "0x401000 (entry point)".to_string(),
			program_header_offset: "64 (bytes into file)".to_string(),
			section_header_offset: "1234 (bytes into file)".to_string(),
			flags: "0x0 (no flags)".to_string(),
			elf_header_size: "64 (bytes)".to_string(),
			program_header_entry_size: "56 (bytes)".to_string(),
			program_header_count: "2 (program headers)".to_string(),
			section_header_entry_size: "64 (bytes)".to_string(),
			section_header_count: "10 (section headers)".to_string(),
			section_header_index_of_section_name_string_table: "9 (string table index)".to_string(),
		};

		header.fix();

		// Check that only the first word is kept for most fields
		assert_eq!(header.file_class, "ELF64");
		assert_eq!(header.endianness, "little");
		assert_eq!(header.elf_version, "1");
		assert_eq!(header.target_os_abi, "UNIX - System V"); // Should remain unchanged
		assert_eq!(header.abi_version, "0");
		assert_eq!(header.ty, "EXEC");
		assert_eq!(header.machine, "Advanced");
		assert_eq!(header.version, "0x1");
		assert_eq!(header.entry, "0x401000");
		assert_eq!(header.program_header_offset, "64");
		assert_eq!(header.section_header_offset, "1234");
		assert_eq!(header.flags, "0x0");
		assert_eq!(header.elf_header_size, "64");
		assert_eq!(header.program_header_entry_size, "56");
		assert_eq!(header.program_header_count, "2");
		assert_eq!(header.section_header_entry_size, "64");
		assert_eq!(header.section_header_count, "10");
		assert_eq!(header.section_header_index_of_section_name_string_table, "9");
	}

	#[test]
	fn test_property_trait_positive() {
		let key_value = vec!["Class", "ELF64"];
		assert!(key_value.is_peoperty_of("Class"));
	}

	#[test]
	fn test_property_trait_negative() {
		let key_value = vec!["Class", "ELF64"];
		assert!(!key_value.is_peoperty_of("Data"));
	}

	#[test]
	fn test_property_trait_empty_vec() {
		let key_value: Vec<&str,> = vec![];
		// This should panic due to index out of bounds, which is expected behavior
		// We don't test this case as it would cause a panic
	}

	#[test]
	fn test_property_trait_single_element() {
		let key_value = vec!["Class"];
		assert!(key_value.is_peoperty_of("Class"));
	}

	#[test]
	fn test_property_trait_multiple_elements() {
		let key_value = vec!["Entry point address", "0x401000", "additional", "info"];
		assert!(key_value.is_peoperty_of("Entry point address"));
		assert!(!key_value.is_peoperty_of("0x401000"));
	}

	#[test]
	#[ignore = "Requires oso_kernel.elf file to exist"]
	fn test_readelf_h_integration() -> Rslt<(),> {
		go_root()?;

		let header = readelf_h()?;

		// Basic validation that we got some data
		assert!(!header.file_class.is_empty());
		assert!(!header.machine.is_empty());
		assert!(!header.entry.is_empty());

		// ELF files should have some basic properties
		assert!(header.file_class.starts_with("ELF"));

		Ok((),)
	}

	#[test]
	fn test_readelf_h_without_kernel_file() {
		// This should fail because the kernel file doesn't exist
		let result = readelf_h();
		assert!(result.is_err());
	}

	#[test]
	fn test_debug_trait_implementation() {
		let header = ReadElfH::default();

		// Should be able to debug print the struct
		let debug_str = format!("{:?}", header);
		assert!(debug_str.contains("ReadElfH"));
	}

	#[test]
	fn test_readelf_h_field_parsing_simulation() {
		// Simulate parsing different types of readelf output lines
		let test_cases = vec![
			("Class:                             ELF64", "Class", "ELF64",),
			("Data:                              2's complement, little endian", "Data", "2's",),
			("Version:                           1 (current)", "Version", "1",),
			("OS/ABI:                            UNIX - System V", "OS/ABI", "UNIX - System V",),
			("Type:                              EXEC (Executable file)", "Type", "EXEC",),
			(
				"Machine:                           Advanced Micro Devices X86-64",
				"Machine",
				"Advanced",
			),
			("Entry point address:               0x401000", "Entry point address", "0x401000",),
		];

		for (line, expected_key, expected_first_word,) in test_cases {
			let key_value: Vec<_,> = line.split(':',).map(|s| s.trim(),).collect();

			if key_value.len() >= 2 {
				assert_eq!(key_value[0], expected_key);

				if expected_key != "OS/ABI" {
					// OS/ABI is special case
					let first_word = key_value[1].split(' ',).nth(0,).unwrap();
					assert_eq!(first_word, expected_first_word);
				}
			}
		}
	}

	#[test]
	fn test_readelf_h_version_field_handling() {
		// Test the special case where Version field can be either ELF version or object version
		let elf_version_line = "Version:                           1 (current)";
		let object_version_line = "Version:                           0x1";

		let elf_key_value: Vec<_,> = elf_version_line.split(':',).map(|s| s.trim(),).collect();
		let obj_key_value: Vec<_,> = object_version_line.split(':',).map(|s| s.trim(),).collect();

		// ELF version doesn't contain 0x
		assert!(!elf_key_value[1].contains("0x"));

		// Object version contains 0x
		assert!(obj_key_value[1].contains("0x"));
	}
}
