#![feature(string_from_utf8_lossy_owned)]
#![feature(exit_status_error)]

use anyhow::Result as Rslt;
use builder::Builder;

pub mod builder;
pub mod qemu;
pub mod shell;
pub mod workspace;

fn main() -> Rslt<(),> {
	let builder = Builder::new()?;
	builder.build()?;
	builder.run()?;
	Ok((),)
}
