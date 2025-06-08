use crate::check_oso_kernel;
use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::process::Command;

trait IntField: Sized {
	fn parse(hex: &str,) -> Rslt<Self,>;
}

impl IntField for u32 {
	fn parse(hex: &str,) -> Rslt<Self,> {
		let rslt = Self::from_str_radix(hex, 16,)?;
		Ok(rslt,)
	}
}
impl IntField for u64 {
	fn parse(hex: &str,) -> Rslt<Self,> {
		let rslt = Self::from_str_radix(hex, 16,)?;
		Ok(rslt,)
	}
}

#[derive(Default, Debug,)]
pub struct ReadElfL {
	pub ty:               String,
	pub offset:           u64,
	pub virtual_address:  u64,
	pub physical_address: u64,
	pub file_size:        u64,
	pub memory_size:      u64,
	pub flags:            u32,
	pub align:            u64,
}

pub fn readelf_l() -> Rslt<Vec<ReadElfL,>,> {
	check_oso_kernel()?;

	let program_headers_info =
		Command::new("readelf",).args(["-l", "target/oso_kernel.elf",],).output()?.stdout;
	let program_headers_info = String::from_utf8(program_headers_info,)?;

	let program_headers_info: Vec<_,> = program_headers_info.split("Program Headers:",).collect();

	let desc_lines_count = program_headers_info[0].lines().count();
	let program_header_count: usize = program_headers_info[0]
		.lines()
		.nth(desc_lines_count - 2,)
		.unwrap()
		.split(" ",)
		.nth(2,)
		.unwrap()
		.parse()?;

	// let program_headers = Vec::with_capacity(program_header_count,);

	let program_headers_info = program_headers_info[1]
		.lines()
		.skip(3,)
		.array_chunks::<2>()
		.map(|s| s.concat(),)
		.take(program_header_count,)
		.map(|s| {
			let fields_info: Vec<_,> = s.split(" ",).filter(|s| *s == "",).collect();

			let ty = fields_info[0].to_string();
			let offset = parse_str_hex_repr(fields_info[1],)?;
			let virtual_address = parse_str_hex_repr(fields_info[2],)?;
			let physical_address = parse_str_hex_repr(fields_info[3],)?;
			let file_size = parse_str_hex_repr(fields_info[4],)?;
			let memory_size = parse_str_hex_repr(fields_info[5],)?;
			let (flags, align,) = parse_flags_and_align(&fields_info,)?;

			Ok(ReadElfL {
				ty,
				offset,
				virtual_address,
				physical_address,
				file_size,
				memory_size,
				flags,
				align,
			},)
		},)
		.try_collect();

	program_headers_info
}

fn parse_str_hex_repr<I: IntField,>(hex: &str,) -> Rslt<I,> {
	let hex_repr = &hex[2..];
	I::parse(hex_repr,)
}

fn parse_flags_and_align(fields_info: &Vec<&str,>,) -> Rslt<(u32, u64,),> {
	let rslt = if fields_info.len() == 8 {
		let flags_str = fields_info[6];
		let mut flags = 0;
		if flags_str.contains("R",) {
			flags |= 0b100;
		}
		if flags_str.contains("W",) {
			flags |= 0b10;
		}
		if flags_str.contains("X",) {
			flags |= 0b1;
		};

		let align = parse_str_hex_repr(fields_info[7],)?;
		(flags, align,)
	} else if fields_info.len() == 9 {
		let align = parse_str_hex_repr(fields_info[8],)?;
		(0x101, align,)
	} else {
		return Err(anyhow!("fields_info length should be 8 or 9, get {}", fields_info.len()),);
	};

	Ok(rslt,)
}
