use oso_dev_util::cargo::*;
use oso_dev_util::fs::*;
use std::path::PathBuf;

#[test]
fn test_basic_functionality() {
	// Test basic enum functionality
	let build_mode = BuildMode::Debug;
	assert!(build_mode.is_debug());
	assert_eq!(build_mode.as_ref(), "Debug");

	let runs_on = RunsOn::Oso;
	assert!(runs_on.is_oso());
	assert_eq!(runs_on.as_ref(), "Oso");

	let arch = Arch::Aarch64;
	assert!(arch.is_aarch_64());
	assert_eq!(arch.as_ref(), "Aarch64");
}

#[test]
fn test_cli_conversion() {
	let cli = Cli {
		build_mode:    Some(BuildMode::Debug,),
		feature_flags: None,
		runs_on:       Some(RunsOn::Linux,),
		arch:          Some(Arch::Riscv64,),
	};

	let opts = cli.to_opts();
	assert!(opts.build_mode.is_debug());
	assert!(opts.target.runs_on.is_linux());
	assert!(opts.target.arch.is_riscv_64());
}

#[test]
fn test_compile_opt_trait() {
	let opts = Opts {
		build_mode:    BuildMode::Relese,
		feature_flags: vec![],
		target:        Target { runs_on: RunsOn::Mac, arch: Arch::Aarch64, },
	};

	let build_mode: String = opts.build_mode().into();
	assert_eq!(build_mode, "Relese");

	let runs_on: String = opts.runs_on().into();
	assert_eq!(runs_on, "Mac");

	let arch: String = opts.arch().into();
	assert_eq!(arch, "Aarch64");

	let target: String = opts.target().into();
	assert_eq!(target, "aarch64-unknown-mac");
}

#[test]
fn test_fs_functions_exist() {
	// Test that fs functions exist and return Results
	let _project_result = project_root();
	let _current_result = current_crate();

	// These functions may fail in test environment, but they should exist
}

#[test]
fn test_firmware_struct() {
	let firmware =
		Firmware { code: PathBuf::from("/test/code",), vars: PathBuf::from("/test/vars",), };

	assert_eq!(firmware.code, PathBuf::from("/test/code"));
	assert_eq!(firmware.vars, PathBuf::from("/test/vars"));
}

#[test]
fn test_assets_struct() {
	let assets = Assets {
		firmware: Firmware {
			code: PathBuf::from("/ovmf/code",),
			vars: PathBuf::from("/ovmf/vars",),
		},
	};

	assert_eq!(assets.firmware.code, PathBuf::from("/ovmf/code"));
	assert_eq!(assets.firmware.vars, PathBuf::from("/ovmf/vars"));
}

#[test]
fn test_enum_defaults() {
	assert!(BuildMode::default().is_debug());
	assert!(RunsOn::default().is_oso());
	assert!(Arch::default().is_aarch_64());

	let target = Target::default();
	assert!(target.runs_on.is_oso());
	assert!(target.arch.is_aarch_64());
}

#[test]
fn test_enum_cloning() {
	let build_mode = BuildMode::Relese;
	let cloned = build_mode;
	assert_eq!(build_mode, cloned);

	let runs_on = RunsOn::Linux;
	let cloned = runs_on;
	assert_eq!(runs_on, cloned);

	let arch = Arch::Riscv64;
	let cloned = arch;
	assert_eq!(arch, cloned);
}

#[test]
fn test_enum_equality() {
	assert_eq!(BuildMode::Debug, BuildMode::Debug);
	assert_ne!(BuildMode::Debug, BuildMode::Relese);

	assert_eq!(RunsOn::Oso, RunsOn::Oso);
	assert_ne!(RunsOn::Oso, RunsOn::Linux);

	assert_eq!(Arch::Aarch64, Arch::Aarch64);
	assert_ne!(Arch::Aarch64, Arch::Riscv64);
}

#[test]
fn test_string_conversions() {
	use std::str::FromStr;

	// Test AsRefStr
	assert_eq!(BuildMode::Debug.as_ref(), "Debug");
	assert_eq!(RunsOn::Oso.as_ref(), "Oso");
	assert_eq!(Arch::Aarch64.as_ref(), "Aarch64");

	// Test FromStr
	assert_eq!(BuildMode::from_str("Debug").unwrap(), BuildMode::Debug);
	assert_eq!(RunsOn::from_str("Oso").unwrap(), RunsOn::Oso);
	assert_eq!(Arch::from_str("Aarch64").unwrap(), Arch::Aarch64);
}

#[test]
fn test_is_methods() {
	// BuildMode
	assert!(BuildMode::Debug.is_debug());
	assert!(!BuildMode::Debug.is_relese());
	assert!(BuildMode::Relese.is_relese());
	assert!(!BuildMode::Relese.is_debug());

	// RunsOn
	assert!(RunsOn::Oso.is_oso());
	assert!(RunsOn::Linux.is_linux());
	assert!(RunsOn::Mac.is_mac());
	assert!(RunsOn::Uefi.is_uefi());

	// Arch
	assert!(Arch::Aarch64.is_aarch_64());
	assert!(Arch::Riscv64.is_riscv_64());
	assert!(!Arch::Aarch64.is_riscv_64());
	assert!(!Arch::Riscv64.is_aarch_64());
}
