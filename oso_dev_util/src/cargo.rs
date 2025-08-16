use oso_proc_macro::features;
use std::path::PathBuf;

pub trait CompileOpt {
	fn build_mode(&self,) -> impl Into<String,>;
	fn feature_flags(&self,) -> Vec<impl Into<String,>,>;
	fn runs_on(&self,) -> impl Into<String,>;
	fn arch(&self,) -> impl Into<String,>;
	/// return target tuple
	fn target(&self,) -> impl Into<String,>;
}

#[features]
#[derive(strum_macros::AsRefStr, strum_macros::EnumIs, strum_macros::EnumString, Clone,)]
pub enum Feature {}

pub struct Opts {
	build_mode:    BuildMode,
	feature_flags: Vec<Feature,>,
	target:        Target,
}

#[derive(clap::Parser,)]
pub struct Cli {
	#[arg(value_enum, short)]
	build_mode:    Option<BuildMode,>,
	#[arg(short)]
	feature_flags: Option<Vec<Feature,>,>,
	#[arg(short)]
	runs_on:       Option<RunsOn,>,
	#[arg(short)]
	arch:          Option<Arch,>,
}

impl Cli {
	pub fn to_opts(self,) -> Opts {
		Opts {
			build_mode:    self.build_mode.unwrap_or_default(),
			feature_flags: self.feature_flags.unwrap_or_default(),
			target:        Target {
				runs_on: self.runs_on.unwrap_or_default(),
				arch:    self.arch.unwrap_or_default(),
			},
		}
	}
}

#[derive(
	Clone,
	clap::ValueEnum,
	Default,
	strum_macros::AsRefStr,
	strum_macros::EnumIs,
	strum_macros::EnumString,
)]
pub enum BuildMode {
	Relese,
	#[default]
	Debug,
}

pub struct Assets {
	firmware: Firmware,
}

/// Manages OVMF firmware files for UEFI boot
#[derive(Debug,)]
pub struct Firmware {
	/// Path to the OVMF code file
	code: PathBuf,
	/// Path to the OVMF variables file
	vars: PathBuf,
}

#[derive(Default, Clone,)]
pub struct Target {
	runs_on: RunsOn,
	arch:    Arch,
}

#[derive(
	Default,
	strum_macros::AsRefStr,
	strum_macros::EnumIs,
	strum_macros::EnumString,
	Clone,
	clap::ValueEnum,
)]
pub enum RunsOn {
	Mac,
	Uefi,
	#[default]
	Oso,
	Linux,
}

#[derive(
	Default,
	strum_macros::AsRefStr,
	strum_macros::EnumIs,
	strum_macros::EnumString,
	Clone,
	clap::ValueEnum,
)]
pub enum Arch {
	#[default]
	Aarch64,
	Riscv64,
}
