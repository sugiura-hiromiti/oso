use crate::decl_manage::crate_::OsoCrate;
use std::path::PathBuf;

//  TODO: refactor to use clap
pub struct XtaskInfo {
	opts:   Opts,
	ws:     OsoCrate,
	assets: Assets,
	host:   Target,
}

pub struct Opts {
	build_mode: BuildMode,
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

pub struct Target {
	runs_on: RunsOn,
	arch:    Arch,
}

pub enum RunsOn {
	Mac,
	Uefi,
	Oso,
	Linux,
}
pub enum Arch {
	Aarch64,
	Riscv64,
}
