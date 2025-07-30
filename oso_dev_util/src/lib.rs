#![feature(exit_status_error)]

use anyhow::Result as Rslt;
use colored::Colorize;
use std::ffi::OsStr;
use std::process::Command;
use std::process::Stdio;

pub trait Run {
	fn run(&mut self,) -> Rslt<(),>;
}

impl Run for Command {
	fn run(&mut self,) -> Rslt<(),> {
		let cmd_dsply = format!(
			"{} {}",
			self.get_program().display(),
			self.get_args().collect::<Vec<&OsStr,>>().join(OsStr::new(" ")).display()
		);
		println!("\n{}", cmd_dsply.bold().blue());
		let out = self
			.stdout(Stdio::inherit(),)
			.stderr(Stdio::inherit(),)
			.stdin(Stdio::inherit(),)
			.status()?;
		out.exit_ok()?;
		Ok((),)
	}
}
