use anyhow::Result as Rslt;
use anyhow::anyhow;
use colored::Colorize;
use std::ffi::OsStr;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug,)]
pub enum Architecture {
	Aarch64,
	Riscv64,
	X86_64,
}

impl Architecture {
	pub fn boot_file_name(&self,) -> String {
		match self {
			Architecture::Aarch64 => "bootaa64.efi",
			Architecture::Riscv64 => "bootriscv64.efi",
			Architecture::X86_64 => "bootx64.efi",
		}
		.to_string()
	}
}

impl TryFrom<&String,> for Architecture {
	type Error = anyhow::Error;

	fn try_from(value: &String,) -> Result<Self, Self::Error,> {
		let arch = if value.contains("aarch64",) {
			Self::Aarch64
		} else if value.contains("riscv64",) {
			Self::Riscv64
		} else if value.contains("x86_64",) {
			Self::X86_64
		} else {
			return Err(anyhow!("target {value} is not supported"),);
		};

		Ok(arch,)
	}
}

impl ToString for Architecture {
	fn to_string(&self,) -> String {
		match self {
			Architecture::Aarch64 => "aarch64",
			Architecture::Riscv64 => todo!(),
			Architecture::X86_64 => "x86_64",
		}
		.to_string()
	}
}

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

#[derive(PartialEq,)]
pub enum BuildMode {
	Release,
	Debug,
}

impl BuildMode {
	pub fn is_release(&self,) -> bool {
		self == &BuildMode::Release
	}
}

impl ToString for BuildMode {
	fn to_string(&self,) -> String {
		match self {
			BuildMode::Release => "release",
			BuildMode::Debug => "debug",
		}
		.to_string()
	}
}

pub struct Opts {
	pub build_mode: BuildMode,
	pub arch:       Architecture,
}

impl Opts {
	pub fn new() -> Self {
		let args = std::env::args();

		let mut build_mode = Some(BuildMode::Debug,);
		let mut arch = Some(Architecture::Aarch64,);
		args.for_each(|s| match s.as_str() {
			"-r" | "--release" => {
				build_mode = Some(BuildMode::Release,);
			},
			"-86" | "-x86_64" => {
				arch = Some(Architecture::X86_64,);
			},
			_ => (),
		},);

		Self { build_mode: build_mode.unwrap(), arch: arch.unwrap(), }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn build_mode_cmp() {
		assert!(BuildMode::Release.is_release());
		assert!(!BuildMode::Debug.is_release());
	}
}
