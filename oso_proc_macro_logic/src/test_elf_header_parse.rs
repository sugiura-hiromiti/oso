use crate::check_oso_kernel;
use anyhow::Result as Rslt;
use std::ops::Index;
use std::process::Command;

#[derive(Default, Debug,)]
pub struct ReadElfH {
	pub file_class: String,
	pub endianness: String,
	pub elf_version: String,
	pub target_os_abi: String,
	pub abi_version: String,
	pub ty: String,
	pub machine: String,
	pub version: String,
	pub entry: String,
	pub program_header_offset: String,
	pub section_header_offset: String,
	pub flags: String,
	pub elf_header_size: String,
	pub program_header_entry_size: String,
	pub program_header_count: String,
	pub section_header_entry_size: String,
	pub section_header_count: String,
	pub section_header_index_of_section_name_string_table: String,
}

impl ReadElfH {
	fn fix(&mut self,) {
		self.file_class = self.file_class.split(" ",).nth(0,).unwrap().to_string();
		self.endianness = self.endianness.split(" ",).nth(0,).unwrap().to_string();
		self.elf_version = self.elf_version.split(" ",).nth(0,).unwrap().to_string();
		// self.target_os_abi = self.target_os_abi.split(" ",).nth(0,).unwrap().to_string();
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

trait Property {
	fn is_peoperty_of(&self, key: &str,) -> bool;
}

impl Property for Vec<&str,> {
	fn is_peoperty_of(&self, key: &str,) -> bool {
		*self.index(0,) == key
	}
}

pub fn readelf_h() -> Rslt<ReadElfH,> {
	check_oso_kernel()?;
	let header_info =
		Command::new("readelf",).args(["-h", "target/oso_kernel.elf",],).output()?.stdout;

	let header_info = String::from_utf8(header_info,)?;

	let mut header = ReadElfH::default();
	header_info.lines().for_each(|line| {
		let key_value: Vec<_,> = line.split(':',).map(|s| s.trim(),).collect();

		if key_value.is_peoperty_of("Class",) {
			header.file_class = key_value[1].to_string();
		}
		if key_value.is_peoperty_of("Data",) {
			header.endianness = key_value[1].split(" ",).nth(2,).unwrap().to_string();
		}
		if key_value.is_peoperty_of("Version",) {
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

	header.fix();

	Ok(header,)
}
