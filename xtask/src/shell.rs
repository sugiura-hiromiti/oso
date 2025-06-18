//! # Shell Module
//!
//! This module handles command-line argument parsing and architecture definitions.
//!
//! It provides:
//! - Architecture enum for supported target architectures
//! - BuildMode enum for debug/release build modes
//! - Feature enum for build features
//! - Opts struct for command-line options

use anyhow::anyhow;
use std::ffi::OsStr;

/// Supported target architectures
#[derive(Debug, PartialEq, Eq,)]
pub enum Architecture {
	/// ARM 64-bit architecture
	Aarch64,
	/// RISC-V 64-bit architecture
	Riscv64,
	/// x86 64-bit architecture
	X86_64,
}

impl Architecture {
	/// Gets the boot file name for the architecture
	///
	/// # Returns
	///
	/// The boot file name (e.g., "bootaa64.efi" for aarch64)
	pub fn boot_file_name(&self,) -> String {
		match self {
			Architecture::Aarch64 => "bootaa64.efi",
			Architecture::Riscv64 => "bootriscv64.efi",
			Architecture::X86_64 => "bootx64.efi",
		}
		.to_string()
	}
}

/// Converts a string to an Architecture enum
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

/// Converts an Architecture enum to a string
impl ToString for Architecture {
	fn to_string(&self,) -> String {
		match self {
			Architecture::Aarch64 => "aarch64",
			Architecture::Riscv64 => "riscv64",
			Architecture::X86_64 => "x86_64",
		}
		.to_string()
	}
}

/// Build mode (debug or release)
#[derive(PartialEq, Debug,)]
pub enum BuildMode {
	/// Release build mode (optimized)
	Release,
	/// Debug build mode (with debug symbols)
	Debug,
}

impl BuildMode {
	/// Checks if the build mode is release
	///
	/// # Returns
	///
	/// true if the build mode is release, false otherwise
	pub fn is_release(&self,) -> bool {
		self == &BuildMode::Release
	}
}

/// Converts a BuildMode enum to a string
impl ToString for BuildMode {
	fn to_string(&self,) -> String {
		match self {
			BuildMode::Release => "release",
			BuildMode::Debug => "debug",
		}
		.to_string()
	}
}

/// Build features
#[derive(Debug, PartialEq, Eq,)]
pub enum Feature {
	/// Loader feature
	Loader(String,),
	/// Kernel feature
	Kernel(String,),
	/// Workspace feature
	Workspace(String,),
}

impl Feature {
	/// Creates a Feature enum from a string
	///
	/// # Parameters
	///
	/// * `s` - The feature string
	///
	/// # Returns
	///
	/// A vector of Feature enums
	fn from_str(s: &str,) -> Vec<Self,> {
		match s {
			f if f == "rgb" || f == "bgr" || f == "bitmask" || f == "bltonly" => {
				vec![Self::Loader(f.to_string(),), Self::Kernel(f.to_string(),)]
			},
			_ => vec![],
		}
	}
}

/// Converts a Feature enum to an OsStr
impl AsRef<OsStr,> for Feature {
	fn as_ref(&self,) -> &OsStr {
		match self {
			Feature::Loader(s,) => OsStr::new(s,),
			Feature::Kernel(s,) => OsStr::new(s,),
			Feature::Workspace(s,) => OsStr::new(s,),
		}
	}
}

/// Command-line options
#[derive(Debug,)]
pub struct Opts {
	/// Build mode (debug or release)
	pub build_mode: BuildMode,
	/// Target architecture
	pub arch:       Architecture,
	/// Build features
	pub features:   Vec<Feature,>,
	/// Debug mode flag
	pub debug:      bool,
}

impl Opts {
	/// Creates a new Opts instance from command-line arguments
	///
	/// # Returns
	///
	/// A new Opts instance with the parsed command-line options
	pub fn new() -> Self {
		let args = std::env::args();

		let mut build_mode = Some(BuildMode::Debug,);
		let mut arch = Some(Architecture::Aarch64,);
		let mut features = Some(vec![],);
		let mut feature_zone = false;
		let mut debug = false;
		args.for_each(|s| match s.as_str() {
			"-r" | "--release" => {
				build_mode = Some(BuildMode::Release,);
			},
			"-86" | "-x86_64" => {
				arch = Some(Architecture::X86_64,);
			},
			"--features" => feature_zone = true,
			flag if feature_zone => {
				features.replace(Feature::from_str(flag,),);
				todo!()
			},
			"--debug" => {
				debug = true;
			},
			_ => (),
		},);

		Self {
			build_mode: build_mode.unwrap(),
			arch: arch.unwrap(),
			features: features.unwrap(),
			debug,
		}
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

	#[test]
	fn arch_boot_file_name() {
		let aarch64 = Architecture::Aarch64;
		assert_eq!(aarch64.boot_file_name(), "bootaa64.efi");
		let riscv64 = Architecture::Riscv64;
		assert_eq!(riscv64.boot_file_name(), "bootriscv64.efi");
		let x86 = Architecture::X86_64;
		assert_eq!(x86.boot_file_name(), "bootx64.efi");
	}

	#[test]
	fn arch_try_from_string() {
		let build_target =
			["aarch64-apple-darwin", "riscv64gc-unknown-linux-gnu", "x86_64-pc-windows-gnu",];
		let arch = [Architecture::Aarch64, Architecture::Riscv64, Architecture::X86_64,];

		build_target.into_iter().zip(arch,).for_each(|(b, a,)| {
			assert_eq!(Architecture::try_from(&b.to_string()).unwrap(), a);
		},);

		let not_build_target = "lol";
		assert!(Architecture::try_from(&not_build_target.to_string()).is_err());
	}

	#[test]
	fn arch_to_string() {
		let from_to = [
			(Architecture::Aarch64, "aarch64".to_string(),),
			(Architecture::Riscv64, "riscv64".to_string(),),
			(Architecture::X86_64, "x86_64".to_string(),),
		];

		from_to.iter().for_each(|(a, s,)| {
			assert_eq!(&a.to_string(), s);
		},);
	}

	#[test]
	fn build_mode_is_release() {
		assert!(BuildMode::Release.is_release());
		assert!(!BuildMode::Debug.is_release());
	}

	#[test]
	fn build_mode_to_string() {
		let release = BuildMode::Release.to_string();
		let debug = BuildMode::Debug.to_string();
		assert_eq!(release.as_str(), "release");
		assert_eq!(debug.as_str(), "debug");
	}

	#[test]
	fn feature_from_string() {
		let features = ["rgb", "bgr", "bitmask", "bltonly",].map(|s| Feature::from_str(s,),);
		let answer = [
			vec![Feature::Loader(format!("rgb"),), Feature::Kernel(format!("rgb"),)],
			vec![Feature::Loader(format!("bgr"),), Feature::Kernel(format!("bgr"),)],
			vec![Feature::Loader(format!("bitmask"),), Feature::Kernel(format!("bitmask"),)],
			vec![Feature::Loader(format!("bltonly"),), Feature::Kernel(format!("bltonly"),)],
		];
		features.into_iter().zip(answer,).for_each(|(f, a,)| assert_eq!(f, a),);

		let not_feature = Feature::from_str("this is not feature",);
		assert_eq!(not_feature.len(), 0);
	}

	#[test]
	fn feature_as_ref_os_str() {
		assert_eq!(Feature::Loader("rgb".to_string()).as_ref(), OsStr::new("rgb"));
	}
}
