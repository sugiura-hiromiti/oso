use oso_dev_util::cargo::*;
use oso_dev_util::decl_manage::crate_::CrateInfo;
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
		build_mode:    BuildMode::Release,
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
	let build_mode = BuildMode::Release;
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
	assert_ne!(BuildMode::Debug, BuildMode::Release);

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
	assert!(BuildMode::Release.is_relese());
	assert!(!BuildMode::Release.is_debug());

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

#[test]
fn test_comprehensive_enum_combinations() {
	// Test all possible combinations of enums
	use clap::ValueEnum;

	for build_mode in BuildMode::value_variants() {
		for runs_on in RunsOn::value_variants() {
			for arch in Arch::value_variants() {
				let target = Target { runs_on: *runs_on, arch: *arch, };
				let opts = Opts { build_mode: *build_mode, feature_flags: vec![], target, };

				// Test CompileOpt trait methods
				let build_mode_str: String = opts.build_mode().into();
				let runs_on_str: String = opts.runs_on().into();
				let arch_str: String = opts.arch().into();
				let target_str: String = opts.target().into();

				// Verify string representations
				assert_eq!(build_mode_str, build_mode.as_ref());
				assert_eq!(runs_on_str, runs_on.as_ref());
				assert_eq!(arch_str, arch.as_ref());

				// Verify target tuple format
				assert!(target_str.contains(&arch.as_ref().to_lowercase()));
				assert!(target_str.contains(&runs_on.as_ref().to_lowercase()));
				assert!(target_str.contains("unknown"));

				let parts: Vec<&str,> = target_str.split('-',).collect();
				assert_eq!(parts.len(), 3);
				assert_eq!(parts[1], "unknown");
			}
		}
	}
}

#[test]
fn test_cli_all_combinations() {
	// Test CLI with all possible combinations
	use clap::ValueEnum;

	for build_mode in [None, Some(BuildMode::Debug,), Some(BuildMode::Release,),] {
		for runs_on in [
			None,
			Some(RunsOn::Oso,),
			Some(RunsOn::Linux,),
			Some(RunsOn::Mac,),
			Some(RunsOn::Uefi,),
		] {
			for arch in [None, Some(Arch::Aarch64,), Some(Arch::Riscv64,),] {
				let cli = Cli { build_mode, feature_flags: Some(vec![],), runs_on, arch, };

				let opts = cli.to_opts();

				// Verify defaults are applied correctly
				match build_mode {
					Some(bm,) => assert_eq!(opts.build_mode, bm),
					None => assert_eq!(opts.build_mode, BuildMode::default()),
				}

				match runs_on {
					Some(ro,) => assert_eq!(opts.target.runs_on, ro),
					None => assert_eq!(opts.target.runs_on, RunsOn::default()),
				}

				match arch {
					Some(a,) => assert_eq!(opts.target.arch, a),
					None => assert_eq!(opts.target.arch, Arch::default()),
				}
			}
		}
	}
}

#[test]
fn test_error_cases() {
	// Test error handling in string parsing
	use std::str::FromStr;

	// Invalid enum values should fail
	assert!(BuildMode::from_str("Invalid").is_err());
	assert!(BuildMode::from_str("release").is_err()); // case sensitive
	assert!(BuildMode::from_str("").is_err());

	assert!(RunsOn::from_str("Windows").is_err());
	assert!(RunsOn::from_str("linux").is_err()); // case sensitive
	assert!(RunsOn::from_str("").is_err());

	assert!(Arch::from_str("x86_64").is_err());
	assert!(Arch::from_str("arm64").is_err());
	assert!(Arch::from_str("aarch64").is_err()); // case sensitive
	assert!(Arch::from_str("").is_err());
}

#[test]
fn test_memory_efficiency() {
	// Test that enums are memory efficient
	use std::mem;

	// Enums should be small
	assert!(mem::size_of::<BuildMode,>() <= 8);
	assert!(mem::size_of::<RunsOn,>() <= 8);
	assert!(mem::size_of::<Arch,>() <= 8);

	// Test that we can create many instances without issues
	let mut targets = Vec::new();
	for i in 0..1000 {
		let target = Target {
			runs_on: if i % 2 == 0 { RunsOn::Oso } else { RunsOn::Linux },
			arch:    if i % 3 == 0 { Arch::Aarch64 } else { Arch::Riscv64 },
		};
		targets.push(target,);
	}

	assert_eq!(targets.len(), 1000);
}

#[test]
fn test_thread_safety() {
	// Test that enums can be used across threads
	use std::sync::Arc;
	use std::thread;

	let build_mode = Arc::new(BuildMode::Debug,);
	let runs_on = Arc::new(RunsOn::Oso,);
	let arch = Arc::new(Arch::Aarch64,);

	let handles: Vec<_,> = (0..5)
		.map(|_| {
			let bm = Arc::clone(&build_mode,);
			let ro = Arc::clone(&runs_on,);
			let a = Arc::clone(&arch,);

			thread::spawn(move || {
				assert!(bm.is_debug());
				assert!(ro.is_oso());
				assert!(a.is_aarch_64());

				let opts = Opts {
					build_mode:    *bm,
					feature_flags: vec![],
					target:        Target { runs_on: *ro, arch: *a, },
				};

				let _target: String = opts.target().into();
			},)
		},)
		.collect();

	for handle in handles {
		handle.join().unwrap();
	}
}

#[test]
fn test_fs_integration() {
	// Test filesystem functions integration
	let project_result = project_root();
	let current_result = current_crate();

	// Functions should return consistent types
	match (project_result, current_result,) {
		(Ok(project_crate,), Ok(current_crate,),) => {
			// Both should be valid OsoCrate instances
			let project_path = project_crate.path();
			let current_path = current_crate.path();

			// Paths should be valid
			assert!(!project_path.as_os_str().is_empty());
			assert!(!current_path.as_os_str().is_empty());

			// Should be able to clone
			let _project_clone = project_crate.clone();
			let _current_clone = current_crate.clone();

			// Should be able to compare
			let project_crate2 = project_crate.clone();
			assert_eq!(project_crate, project_crate2);
		},
		(Err(project_err,), _,) => {
			// Error should be meaningful
			let error_msg = project_err.to_string();
			assert!(!error_msg.is_empty());
		},
		(_, Err(current_err,),) => {
			// Error should be meaningful
			let error_msg = current_err.to_string();
			assert!(!error_msg.is_empty());
		},
	}
}

#[test]
fn test_debug_formatting() {
	// Test that all types have useful Debug output
	let build_mode = BuildMode::Debug;
	let debug_str = format!("{:?}", build_mode);
	assert!(debug_str.contains("Debug"));

	let runs_on = RunsOn::Oso;
	let debug_str = format!("{:?}", runs_on);
	assert!(debug_str.contains("Oso"));

	let arch = Arch::Aarch64;
	let debug_str = format!("{:?}", arch);
	assert!(debug_str.contains("Aarch64"));

	let target = Target::default();
	let debug_str = format!("{:?}", target);
	assert!(!debug_str.is_empty());

	let firmware =
		Firmware { code: PathBuf::from("/test/code.fd",), vars: PathBuf::from("/test/vars.fd",), };
	let debug_str = format!("{:?}", firmware);
	assert!(debug_str.contains("Firmware"));
	assert!(debug_str.contains("code.fd"));
	assert!(debug_str.contains("vars.fd"));
}

#[test]
fn test_value_enum_completeness() {
	// Test that ValueEnum implementations are complete
	use clap::ValueEnum;

	// Test that all variants are included
	let build_mode_variants = BuildMode::value_variants();
	assert_eq!(build_mode_variants.len(), 2);
	assert!(build_mode_variants.contains(&BuildMode::Debug));
	assert!(build_mode_variants.contains(&BuildMode::Release));

	let runs_on_variants = RunsOn::value_variants();
	assert_eq!(runs_on_variants.len(), 4);
	assert!(runs_on_variants.contains(&RunsOn::Mac));
	assert!(runs_on_variants.contains(&RunsOn::Uefi));
	assert!(runs_on_variants.contains(&RunsOn::Oso));
	assert!(runs_on_variants.contains(&RunsOn::Linux));

	let arch_variants = Arch::value_variants();
	assert_eq!(arch_variants.len(), 2);
	assert!(arch_variants.contains(&Arch::Aarch64));
	assert!(arch_variants.contains(&Arch::Riscv64));
}

#[test]
fn test_feature_flags_empty_enum() {
	// Test that Feature enum is properly empty
	// Note: Feature enum doesn't implement ValueEnum since it's empty

	// Should be able to create empty vectors
	let features: Vec<Feature,> = vec![];
	assert!(features.is_empty());

	// Should work in Opts
	let opts = Opts {
		build_mode:    BuildMode::Debug,
		feature_flags: features,
		target:        Target::default(),
	};

	let returned_features = opts.feature_flags();
	assert!(returned_features.is_empty());
}

#[test]
fn test_target_tuple_consistency() {
	// Test that target tuple generation is consistent
	let opts = Opts {
		build_mode:    BuildMode::Debug,
		feature_flags: vec![],
		target:        Target { runs_on: RunsOn::Linux, arch: Arch::Riscv64, },
	};

	// Multiple calls should return the same result
	let target1: String = opts.target().into();
	let target2: String = opts.target().into();
	let target3: String = opts.target().into();

	assert_eq!(target1, target2);
	assert_eq!(target2, target3);
	assert_eq!(target1, "riscv64-unknown-linux");
}

#[test]
fn test_path_handling() {
	// Test PathBuf handling in Firmware and Assets
	let code_path = PathBuf::from("/usr/share/ovmf/OVMF_CODE.fd",);
	let vars_path = PathBuf::from("/usr/share/ovmf/OVMF_VARS.fd",);

	let firmware = Firmware { code: code_path.clone(), vars: vars_path.clone(), };

	let assets = Assets { firmware, };

	// Test that paths are preserved
	assert_eq!(assets.firmware.code, code_path);
	assert_eq!(assets.firmware.vars, vars_path);

	// Test that paths can be manipulated
	let code_parent = assets.firmware.code.parent();
	let vars_parent = assets.firmware.vars.parent();
	assert_eq!(code_parent, vars_parent);

	// Test path string conversion
	let code_str = assets.firmware.code.to_string_lossy();
	let vars_str = assets.firmware.vars.to_string_lossy();
	assert!(code_str.contains("OVMF_CODE.fd"));
	assert!(vars_str.contains("OVMF_VARS.fd"));
}
