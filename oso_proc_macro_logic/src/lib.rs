#![feature(proc_macro_diagnostic)]
#![feature(str_as_str)]
#![feature(iter_array_chunks)]
#![feature(associated_type_defaults)]
#![feature(iterator_try_collect)]

use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::env::current_dir;

extern crate proc_macro;

pub mod fonts_data;
pub mod gen_wrapper_fn;
pub mod impl_init;
pub mod status_from_spec;
pub mod test_elf_header_parse;
pub mod test_program_headers_parse;

/// checks oso_kernel.elf exists
fn check_oso_kernel() -> Rslt<(),> {
	let target_path = current_dir()?.join("target/oso_kernel.elf",);
	if target_path.exists() { Ok((),) } else { Err(anyhow!("oso_kernel.elf not exist"),) }
}
