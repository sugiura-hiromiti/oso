use crate::decl_manage::crate_::OsoCrate;
use oso_proc_macro::features;
use std::path::PathBuf;

//  TODO: refactor to use clap
pub struct XtaskInfo {
	opts:   Opts,
	ws:     OsoCrate,
	assets: Assets,
	host:   Target,
}

#[features]
pub enum Feature {}

pub struct Opts {
	build_mode:    BuildMode,
	//  TODO: auto generate `Feature` enum from macro
	//  attribute macro may suitable
	feature_flags: Vec<Feature,>,
	target:        Target,
}

pub enum BuildMode {
	Relese,
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

#[derive(Default,)]
pub struct Target {
	runs_on: RunsOn,
	arch:    Arch,
}

#[derive(Default,)]
pub enum RunsOn {
	#[default]
	Mac,
	Uefi,
	Oso,
	Linux,
}

#[derive(Default,)]
pub enum Arch {
	#[default]
	Aarch64,
	Riscv64,
}
