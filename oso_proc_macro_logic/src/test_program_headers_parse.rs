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

	let program_headers_info = readelf_l_out()?;

	let program_header_count = program_headers_count(&program_headers_info[0],)?;

	let program_headers_info = program_headers_fields(&program_headers_info, program_header_count,)
		.map(|s| {
			let fields_info: Vec<_,> = s.split(" ",).filter(|s| *s != "",).collect();

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

fn readelf_l_out() -> Rslt<Vec<String,>,> {
	let program_headers_info =
		Command::new("readelf",).args(["-l", "target/oso_kernel.elf",],).output()?.stdout;
	let program_headers_info = String::from_utf8(program_headers_info,)?;
	let program_headers_info: Vec<_,> =
		program_headers_info.split("Program Headers:",).map(|s| s.to_string(),).collect();

	Ok(program_headers_info,)
}

fn program_headers_count(info: &String,) -> Rslt<usize,> {
	let desc_lines_count = info.lines().count();
	let program_header_count: usize =
		info.lines().nth(desc_lines_count - 2,).unwrap().split(" ",).nth(2,).unwrap().parse()?;
	Ok(program_header_count,)
}

fn program_headers_fields(
	infos: &Vec<String,>,
	count: usize,
) -> impl Iterator<Item = std::string::String,> {
	infos[1].lines().skip(3,).array_chunks::<2>().map(|s| s.concat(),).take(count,)
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
		(0b101, align,)
	} else {
		return Err(anyhow!("fields_info length should be 8 or 9, get {}", fields_info.len()),);
	};

	Ok(rslt,)
}

#[cfg(test)]
mod tests {
	use std::env::current_dir;
	use std::env::set_current_dir;

	use super::*;

	fn go_root() -> Rslt<(),> {
		let cwd = current_dir()?;
		if cwd.file_name().unwrap() != "oso" {
			let oso_root = cwd.parent().unwrap();
			set_current_dir(oso_root,)?;
		}
		Ok((),)
	}

	#[test]
	fn test_slice_range() {
		let a = &"0x1"[2..];
		assert_eq!(a, "1");
	}

	#[test]
	// #[ignore = "not now"]
	fn test_readelf_l() -> Rslt<(),> {
		go_root()?;

		let phs = readelf_l()?;
		assert_eq!(phs.len(), 0, "{phs:#?}");
		Ok((),)
	}

	#[test]
	fn test_program_headers_info() -> Rslt<(),> {
		go_root()?;

		let program_headers_info = readelf_l_out()?;
		assert_eq!(program_headers_info.len(), 2);
		Ok((),)
	}

	#[test]
	fn test_program_headers_count() -> Rslt<(),> {
		go_root()?;

		let program_headers_info = readelf_l_out()?;
		let program_header_count = program_headers_count(&program_headers_info[0],)?;

		assert_eq!(program_header_count, 4);
		Ok((),)
	}

	#[test]
	fn test_program_headers_fields() -> Rslt<(),> {
		go_root()?;

		let program_headers_info = readelf_l_out()?;
		let program_header_count = program_headers_count(&program_headers_info[0],)?;
		let program_headers_info =
			program_headers_fields(&program_headers_info, program_header_count,);

		assert_eq!(program_header_count, program_headers_info.count());
		Ok((),)
	}
}
