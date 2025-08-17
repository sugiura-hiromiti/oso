//! # OSO Development Utilities
//!
//! A collection of development utilities and helper functions for the OSO operating system project.
//! This crate provides tools for workspace management, command execution, and development workflow
//! automation.
//!
//! ## Features
//!
//! - **Workspace Management**: Tools for managing multi-crate workspaces
//! - **Command Execution**: Enhanced command execution with better error handling and output
//!   formatting
//! - **Development Workflow**: Utilities to streamline the development process
//! - **Cross-platform Support**: Works across different operating systems
//!
//! ## Key Components
//!
//! ### Command Execution
//!
//! The [`Run`] trait provides enhanced command execution capabilities with:
//! - Colored output formatting
//! - Automatic error handling
//! - Inherited stdio streams
//! - Command display with arguments
//!
//! ### Workspace Management
//!
//! The workspace management system provides:
//! - Root directory detection
//! - Crate enumeration and management
//! - Workspace-wide operations
//!
//! ## Usage
//!
//! ### Basic Command Execution
//!
//! ```rust,no_run
//! use oso_dev_util::Run;
//! use std::process::Command;
//!
//! // Execute a command with enhanced output
//! let mut cmd = Command::new("cargo",);
//! cmd.args(&["build", "--release",],);
//! cmd.run().expect("Build failed",);
//! ```
//!
//! ### Workspace Operations
//!
//! ```rust,ignore
//! use oso_dev_util::{OsoWorkspace, OsoWorkspaceManager};
//!
//! let workspace = OsoWorkspaceManager::new();
//! let root = workspace.root();
//! let crates = workspace.crates();
//!
//! println!("Workspace root: {}", root.display());
//! for crate_path in crates {
//!     println!("Crate: {}", crate_path.display());
//! }
//! ```
//!
//! ## Dependencies
//!
//! - [`anyhow`]: Error handling and context
//! - [`colored`]: Terminal color output
//! - [`toml`]: TOML configuration file parsing

#![feature(exit_status_error)]
#![feature(proc_macro_hygiene)]

use anyhow::Result as Rslt;

pub mod cargo;
#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// flowchart TD
/// A[Crate] --> B[Workspace]
/// A --> C[Package]
/// B --> D[CrateBase]
/// C --> D
/// ```
pub mod decl_manage;
pub mod fs;

/// The path to the oso_dev_util crate manifest, set at compile time
pub const OSO_DEV_UTIL_PATH: &str = std::env!("CARGO_MANIFEST_PATH");

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_oso_dev_util_path_constant() {
		// Test that the OSO_DEV_UTIL_PATH constant is set and valid
		assert!(!OSO_DEV_UTIL_PATH.is_empty());
		assert!(OSO_DEV_UTIL_PATH.contains("Cargo.toml"));

		// Verify the path exists
		let path = std::path::Path::new(OSO_DEV_UTIL_PATH,);
		assert!(path.exists(), "OSO_DEV_UTIL_PATH should point to an existing file");
		assert!(path.is_file(), "OSO_DEV_UTIL_PATH should point to a file");
	}

	#[test]
	fn test_module_accessibility() {
		// Test that all public modules are accessible
		// This is a compile-time test - if it compiles, the modules are accessible

		// Test cargo module
		let _build_mode = cargo::BuildMode::Debug;
		let _runs_on = cargo::RunsOn::Oso;
		let _arch = cargo::Arch::Aarch64;

		// Test that we can access the fs module functions
		// Note: These might fail in test environment, but we test they're callable
		let _project_root_result = fs::project_root();
		let _current_crate_result = fs::current_crate();
	}

	#[test]
	fn test_anyhow_integration() {
		// Test that anyhow integration works as expected
		use anyhow::Context;

		let result: Rslt<String,> =
			Err(anyhow::anyhow!("base error"),).context("additional context",);

		assert!(result.is_err());
		let error_string = result.unwrap_err().to_string();
		assert!(error_string.contains("additional context"));
		// Note: The exact error message format may vary, so we just check for context
	}

	#[test]
	fn test_error_chain() {
		// Test error chaining functionality
		use anyhow::Context;

		fn inner_function() -> Rslt<(),> {
			Err(anyhow::anyhow!("inner error"),)
		}

		fn outer_function() -> Rslt<(),> {
			inner_function().context("outer context",)
		}

		let result = outer_function();
		assert!(result.is_err());

		let error = result.unwrap_err();
		let error_string = error.to_string();
		assert!(error_string.contains("outer context"));

		// Check the error chain
		let mut chain = error.chain();
		assert!(chain.next().unwrap().to_string().contains("outer context"));
		assert!(chain.next().unwrap().to_string().contains("inner error"));
	}

	#[test]
	fn test_module_structure() {
		// Test that the expected module structure exists
		// This is primarily a compile-time test

		// Verify we can create instances of key types
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;
		use cargo::Target;

		let build_mode = BuildMode::Debug;
		assert!(build_mode.is_debug());

		let runs_on = RunsOn::Oso;
		assert!(runs_on.is_oso());

		let arch = Arch::Aarch64;
		assert!(arch.is_aarch_64());

		let target = Target::default();
		// Target should have default values
		assert_eq!(target.runs_on.as_ref(), "Oso");
		assert_eq!(target.arch.as_ref(), "Aarch64");
	}

	#[test]
	fn test_feature_flags() {
		// Test that feature flags are accessible
		use cargo::Feature;

		// Since Feature is an empty enum with the #[features] attribute,
		// we can't create instances, but we can verify it exists
		// This is primarily a compile-time test

		// Test that we can reference the Feature type
		let _feature_type = std::marker::PhantomData::<Feature,>;
	}

	#[test]
	fn test_compile_opt_trait() {
		// Test the CompileOpt trait functionality
		use cargo::BuildMode;
		use cargo::CompileOpt;
		use cargo::Feature;
		use cargo::Opts;
		use cargo::Target;

		let opts = Opts {
			build_mode:    BuildMode::Debug,
			feature_flags: Vec::<Feature,>::new(),
			target:        Target::default(),
		};

		// Test trait methods
		let build_mode: String = opts.build_mode().into();
		assert_eq!(build_mode, "Debug");

		let feature_flags = opts.feature_flags();
		assert!(feature_flags.is_empty());

		let runs_on: String = opts.runs_on().into();
		assert_eq!(runs_on, "Oso");

		let arch: String = opts.arch().into();
		assert_eq!(arch, "Aarch64");

		let target: String = opts.target().into();
		// Target should be formatted as "arch-vendor-os"
		assert!(!target.is_empty());
	}

	#[test]
	fn test_cli_to_opts_conversion() {
		// Test CLI to Opts conversion
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::Cli;
		use cargo::RunsOn;

		let cli = Cli {
			build_mode:    Some(BuildMode::Relese,),
			feature_flags: None,
			runs_on:       Some(RunsOn::Linux,),
			arch:          Some(Arch::Riscv64,),
		};

		let opts = cli.to_opts();
		assert!(opts.build_mode.is_relese());
		assert!(opts.feature_flags.is_empty());
		assert!(opts.target.runs_on.is_linux());
		assert!(opts.target.arch.is_riscv_64());
	}

	#[test]
	fn test_cli_defaults() {
		// Test CLI with default values
		use cargo::Cli;

		let cli = Cli {
			build_mode:    None,
			feature_flags: None,
			runs_on:       None,
			arch:          None,
		};

		let opts = cli.to_opts();
		assert!(opts.build_mode.is_debug()); // Default should be Debug
		assert!(opts.feature_flags.is_empty());
		assert!(opts.target.runs_on.is_oso()); // Default should be Oso
		assert!(opts.target.arch.is_aarch_64()); // Default should be Aarch64
	}

	#[test]
	fn test_firmware_structure() {
		// Test Firmware struct
		use cargo::Firmware;
		use std::path::PathBuf;

		let firmware = Firmware {
			code: PathBuf::from("/path/to/code",),
			vars: PathBuf::from("/path/to/vars",),
		};

		// Test Debug implementation
		let debug_string = format!("{:?}", firmware);
		assert!(debug_string.contains("Firmware"));
		assert!(debug_string.contains("/path/to/code"));
		assert!(debug_string.contains("/path/to/vars"));
	}

	#[test]
	fn test_assets_structure() {
		// Test Assets struct
		use cargo::Assets;
		use cargo::Firmware;
		use std::path::PathBuf;

		let assets = Assets {
			firmware: Firmware {
				code: PathBuf::from("/ovmf/code",),
				vars: PathBuf::from("/ovmf/vars",),
			},
		};

		// Verify the structure exists and is accessible
		assert_eq!(assets.firmware.code, PathBuf::from("/ovmf/code"));
		assert_eq!(assets.firmware.vars, PathBuf::from("/ovmf/vars"));
	}

	#[test]
	fn test_enum_string_conversions() {
		// Test AsRefStr implementations
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;

		assert_eq!(BuildMode::Debug.as_ref(), "Debug");
		assert_eq!(BuildMode::Relese.as_ref(), "Relese");

		assert_eq!(RunsOn::Oso.as_ref(), "Oso");
		assert_eq!(RunsOn::Linux.as_ref(), "Linux");
		assert_eq!(RunsOn::Mac.as_ref(), "Mac");
		assert_eq!(RunsOn::Uefi.as_ref(), "Uefi");

		assert_eq!(Arch::Aarch64.as_ref(), "Aarch64");
		assert_eq!(Arch::Riscv64.as_ref(), "Riscv64");
	}

	#[test]
	fn test_enum_is_methods() {
		// Test EnumIs implementations
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;

		// BuildMode
		assert!(BuildMode::Debug.is_debug());
		assert!(!BuildMode::Debug.is_relese());
		assert!(BuildMode::Relese.is_relese());
		assert!(!BuildMode::Relese.is_debug());

		// RunsOn
		assert!(RunsOn::Oso.is_oso());
		assert!(!RunsOn::Oso.is_linux());
		assert!(RunsOn::Linux.is_linux());
		assert!(RunsOn::Mac.is_mac());
		assert!(RunsOn::Uefi.is_uefi());

		// Arch
		assert!(Arch::Aarch64.is_aarch_64());
		assert!(!Arch::Aarch64.is_riscv_64());
		assert!(Arch::Riscv64.is_riscv_64());
		assert!(!Arch::Riscv64.is_aarch_64());
	}

	#[test]
	fn test_clone_implementations() {
		// Test Clone implementations
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;
		use cargo::Target;

		let build_mode = BuildMode::Debug;
		let cloned_build_mode = build_mode;
		assert_eq!(build_mode.as_ref(), cloned_build_mode.as_ref());

		let runs_on = RunsOn::Oso;
		let cloned_runs_on = runs_on;
		assert_eq!(runs_on.as_ref(), cloned_runs_on.as_ref());

		let arch = Arch::Aarch64;
		let cloned_arch = arch;
		assert_eq!(arch.as_ref(), cloned_arch.as_ref());

		let target = Target::default();
		let cloned_target = target.clone();
		assert_eq!(target.runs_on.as_ref(), cloned_target.runs_on.as_ref());
		assert_eq!(target.arch.as_ref(), cloned_target.arch.as_ref());
	}

	#[test]
	fn test_default_implementations() {
		// Test Default implementations
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;
		use cargo::Target;

		let default_build_mode = BuildMode::default();
		assert!(default_build_mode.is_debug());

		let default_runs_on = RunsOn::default();
		assert!(default_runs_on.is_oso());

		let default_arch = Arch::default();
		assert!(default_arch.is_aarch_64());

		let default_target = Target::default();
		assert!(default_target.runs_on.is_oso());
		assert!(default_target.arch.is_aarch_64());
	}

	#[test]
	fn test_value_enum_implementations() {
		// Test that ValueEnum is implemented for CLI enums
		use cargo::Arch;
		use cargo::BuildMode;
		use cargo::RunsOn;
		use clap::ValueEnum;

		// Test that we can get possible values
		let build_mode_values = BuildMode::value_variants();
		assert_eq!(build_mode_values.len(), 2);
		assert!(build_mode_values.contains(&BuildMode::Debug));
		assert!(build_mode_values.contains(&BuildMode::Relese));

		let runs_on_values = RunsOn::value_variants();
		assert_eq!(runs_on_values.len(), 4);
		assert!(runs_on_values.contains(&RunsOn::Oso));
		assert!(runs_on_values.contains(&RunsOn::Linux));
		assert!(runs_on_values.contains(&RunsOn::Mac));
		assert!(runs_on_values.contains(&RunsOn::Uefi));

		let arch_values = Arch::value_variants();
		assert_eq!(arch_values.len(), 2);
		assert!(arch_values.contains(&Arch::Aarch64));
		assert!(arch_values.contains(&Arch::Riscv64));
	}
}
