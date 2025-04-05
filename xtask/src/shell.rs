use crate::qemu::Ovmf;
use anyhow::Result as Rslt;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug,)]
pub enum Architecture {
	X86_64,
	Aarch64,
}

impl TryFrom<&String,> for Architecture {
	type Error = anyhow::Error;

	fn try_from(value: &String,) -> Result<Self, Self::Error,> {
		let arch = if value.contains("x86_64",) {
			Self::X86_64
		} else if value.contains("aarch64",) {
			Self::Aarch64
		} else {
			return Err(anyhow!("target {value} is not supported"),);
		};

		Ok(arch,)
	}
}

impl ToString for Architecture {
	fn to_string(&self,) -> String {
		match self {
			Architecture::X86_64 => "x86_64",
			Architecture::Aarch64 => "aarch64",
		}
		.to_string()
	}
}

pub trait Run {
	fn run(&mut self,) -> Rslt<(),>;
}

impl Run for Command {
	fn run(&mut self,) -> Rslt<(),> {
		let out = self.stdout(Stdio::inherit(),).stderr(Stdio::inherit(),).status()?;
		out.exit_ok()?;
		Ok((),)
	}
}

#[derive(PartialEq,)]
enum BuildMode {
	Release,
	Debug,
}

impl BuildMode {
	pub fn is_release(&self,) -> bool {
		self == &BuildMode::Release
	}
}

pub struct Opts {
	pub build_mode: BuildMode,
	pub arch:       Architecture,
	ovmf:           Ovmf,
}

impl Opts {
	pub fn new() -> Self {
		todo!()
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
