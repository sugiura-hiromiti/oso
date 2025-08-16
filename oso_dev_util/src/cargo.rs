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
	pub build_mode:    BuildMode,
	pub feature_flags: Vec<Feature,>,
	pub target:        Target,
}

impl CompileOpt for Opts {
	fn build_mode(&self,) -> impl Into<String,> {
		self.build_mode.as_ref()
	}

	fn feature_flags(&self,) -> Vec<impl Into<String,>,> {
		self.feature_flags.iter().map(|f| f.as_ref(),).collect()
	}

	fn runs_on(&self,) -> impl Into<String,> {
		self.target.runs_on.as_ref()
	}

	fn arch(&self,) -> impl Into<String,> {
		self.target.arch.as_ref()
	}

	fn target(&self,) -> impl Into<String,> {
		format!(
			"{}-unknown-{}",
			self.target.arch.as_ref().to_lowercase(),
			self.target.runs_on.as_ref().to_lowercase()
		)
	}
}

#[derive(clap::Parser,)]
pub struct Cli {
	#[arg(value_enum, short)]
	pub build_mode:    Option<BuildMode,>,
	#[arg(short)]
	pub feature_flags: Option<Vec<Feature,>,>,
	#[arg(short)]
	pub runs_on:       Option<RunsOn,>,
	#[arg(short)]
	pub arch:          Option<Arch,>,
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
	Copy,
	clap::ValueEnum,
	Default,
	strum_macros::AsRefStr,
	strum_macros::EnumIs,
	strum_macros::EnumString,
	PartialEq,
	Eq,
	Debug,
)]
pub enum BuildMode {
	Relese,
	#[default]
	Debug,
}

pub struct Assets {
	pub firmware: Firmware,
}

/// Manages OVMF firmware files for UEFI boot
#[derive(Debug,)]
pub struct Firmware {
	/// Path to the OVMF code file
	pub code: PathBuf,
	/// Path to the OVMF variables file
	pub vars: PathBuf,
}

#[derive(Default, Clone,)]
pub struct Target {
	pub runs_on: RunsOn,
	pub arch:    Arch,
}

#[derive(
	Default,
	strum_macros::AsRefStr,
	strum_macros::EnumIs,
	strum_macros::EnumString,
	Clone,
	Copy,
	clap::ValueEnum,
	PartialEq,
	Eq,
	Debug,
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
	Copy,
	clap::ValueEnum,
	PartialEq,
	Eq,
	Debug,
)]
pub enum Arch {
	#[default]
	Aarch64,
	Riscv64,
}

#[cfg(test)]
// #[cfg(false)]
mod tests {
	use super::*;
	use proptest::prelude::*;
	use std::str::FromStr;

	#[test]
	fn test_build_mode_default() {
		let default_mode = BuildMode::default();
		assert!(default_mode.is_debug());
		assert_eq!(default_mode.as_ref(), "Debug");
	}

	#[test]
	fn test_build_mode_variants() {
		assert!(BuildMode::Debug.is_debug());
		assert!(!BuildMode::Debug.is_relese());
		assert!(BuildMode::Relese.is_relese());
		assert!(!BuildMode::Relese.is_debug());
	}

	#[test]
	fn test_build_mode_string_conversion() {
		assert_eq!(BuildMode::Debug.as_ref(), "Debug");
		assert_eq!(BuildMode::Relese.as_ref(), "Relese");
	}

	#[test]
	fn test_build_mode_from_string() {
		assert_eq!(BuildMode::from_str("Debug").unwrap(), BuildMode::Debug);
		assert_eq!(BuildMode::from_str("Relese").unwrap(), BuildMode::Relese);
		assert!(BuildMode::from_str("Invalid").is_err());
	}

	#[test]
	fn test_runs_on_default() {
		let default_runs_on = RunsOn::default();
		assert!(default_runs_on.is_oso());
		assert_eq!(default_runs_on.as_ref(), "Oso");
	}

	#[test]
	fn test_runs_on_variants() {
		assert!(RunsOn::Mac.is_mac());
		assert!(RunsOn::Uefi.is_uefi());
		assert!(RunsOn::Oso.is_oso());
		assert!(RunsOn::Linux.is_linux());

		assert!(!RunsOn::Mac.is_oso());
		assert!(!RunsOn::Uefi.is_linux());
	}

	#[test]
	fn test_runs_on_string_conversion() {
		assert_eq!(RunsOn::Mac.as_ref(), "Mac");
		assert_eq!(RunsOn::Uefi.as_ref(), "Uefi");
		assert_eq!(RunsOn::Oso.as_ref(), "Oso");
		assert_eq!(RunsOn::Linux.as_ref(), "Linux");
	}

	#[test]
	fn test_runs_on_from_string() {
		assert_eq!(RunsOn::from_str("Mac").unwrap(), RunsOn::Mac);
		assert_eq!(RunsOn::from_str("Uefi").unwrap(), RunsOn::Uefi);
		assert_eq!(RunsOn::from_str("Oso").unwrap(), RunsOn::Oso);
		assert_eq!(RunsOn::from_str("Linux").unwrap(), RunsOn::Linux);
		assert!(RunsOn::from_str("Windows").is_err());
	}

	#[test]
	fn test_arch_default() {
		let default_arch = Arch::default();
		assert!(default_arch.is_aarch_64());
		assert_eq!(default_arch.as_ref(), "Aarch64");
	}

	#[test]
	fn test_arch_variants() {
		assert!(Arch::Aarch64.is_aarch_64());
		assert!(Arch::Riscv64.is_riscv_64());

		assert!(!Arch::Aarch64.is_riscv_64());
		assert!(!Arch::Riscv64.is_aarch_64());
	}

	#[test]
	fn test_arch_string_conversion() {
		assert_eq!(Arch::Aarch64.as_ref(), "Aarch64");
		assert_eq!(Arch::Riscv64.as_ref(), "Riscv64");
	}

	#[test]
	fn test_arch_from_string() {
		assert_eq!(Arch::from_str("Aarch64").unwrap(), Arch::Aarch64);
		assert_eq!(Arch::from_str("Riscv64").unwrap(), Arch::Riscv64);
		assert!(Arch::from_str("x86_64").is_err());
	}

	#[test]
	fn test_target_default() {
		let default_target = Target::default();
		assert!(default_target.runs_on.is_oso());
		assert!(default_target.arch.is_aarch_64());
	}

	#[test]
	fn test_target_clone() {
		let target = Target { runs_on: RunsOn::Linux, arch: Arch::Riscv64, };
		let cloned = target.clone();

		assert!(cloned.runs_on.is_linux());
		assert!(cloned.arch.is_riscv_64());
	}

	#[test]
	fn test_cli_to_opts_with_values() {
		let cli = Cli {
			build_mode:    Some(BuildMode::Relese,),
			feature_flags: Some(vec![],),
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
	fn test_cli_to_opts_with_defaults() {
		let cli = Cli {
			build_mode:    None,
			feature_flags: None,
			runs_on:       None,
			arch:          None,
		};

		let opts = cli.to_opts();
		assert!(opts.build_mode.is_debug());
		assert!(opts.feature_flags.is_empty());
		assert!(opts.target.runs_on.is_oso());
		assert!(opts.target.arch.is_aarch_64());
	}

	#[test]
	fn test_compile_opt_implementation() {
		let opts = Opts {
			build_mode:    BuildMode::Relese,
			feature_flags: vec![],
			target:        Target { runs_on: RunsOn::Linux, arch: Arch::Riscv64, },
		};

		let build_mode: String = opts.build_mode().into();
		assert_eq!(build_mode, "Relese");

		let feature_flags = opts.feature_flags();
		assert!(feature_flags.is_empty());

		let runs_on: String = opts.runs_on().into();
		assert_eq!(runs_on, "Linux");

		let arch: String = opts.arch().into();
		assert_eq!(arch, "Riscv64");

		let target: String = opts.target().into();
		assert_eq!(target, "riscv64-unknown-linux");
	}

	#[test]
	fn test_target_tuple_generation() {
		let test_cases = vec![
			(Arch::Aarch64, RunsOn::Oso, "aarch64-unknown-oso",),
			(Arch::Aarch64, RunsOn::Linux, "aarch64-unknown-linux",),
			(Arch::Riscv64, RunsOn::Mac, "riscv64-unknown-mac",),
			(Arch::Riscv64, RunsOn::Uefi, "riscv64-unknown-uefi",),
		];

		for (arch, runs_on, expected,) in test_cases {
			let opts = Opts {
				build_mode:    BuildMode::Debug,
				feature_flags: vec![],
				target:        Target { runs_on, arch, },
			};

			let target: String = opts.target().into();
			assert_eq!(target, expected);
		}
	}

	#[test]
	fn test_firmware_creation() {
		let firmware = Firmware {
			code: PathBuf::from("/path/to/ovmf_code.fd",),
			vars: PathBuf::from("/path/to/ovmf_vars.fd",),
		};

		assert_eq!(firmware.code, PathBuf::from("/path/to/ovmf_code.fd"));
		assert_eq!(firmware.vars, PathBuf::from("/path/to/ovmf_vars.fd"));
	}

	#[test]
	fn test_firmware_debug() {
		let firmware = Firmware { code: PathBuf::from("/code",), vars: PathBuf::from("/vars",), };

		let debug_str = format!("{:?}", firmware);
		assert!(debug_str.contains("Firmware"));
		assert!(debug_str.contains("/code"));
		assert!(debug_str.contains("/vars"));
	}

	#[test]
	fn test_assets_creation() {
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
	fn test_feature_enum_exists() {
		// Test that Feature enum exists and can be used in collections
		let features: Vec<Feature,> = vec![];
		assert!(features.is_empty());

		// Test that Feature implements required traits
		let _phantom: std::marker::PhantomData<Feature,> = std::marker::PhantomData;
	}

	// Property-based tests
	proptest! {
		#[test]
		fn test_build_mode_roundtrip(mode in prop::sample::select(vec![BuildMode::Debug, BuildMode::Relese])) {
			let as_str = mode.as_ref();
			let parsed = BuildMode::from_str(as_str).unwrap();
			assert_eq!(mode, parsed);
		}

		#[test]
		fn test_runs_on_roundtrip(runs_on in prop::sample::select(vec![RunsOn::Mac, RunsOn::Uefi, RunsOn::Oso, RunsOn::Linux])) {
			let as_str = runs_on.as_ref();
			let parsed = RunsOn::from_str(as_str).unwrap();
			assert_eq!(runs_on, parsed);
		}

		#[test]
		fn test_arch_roundtrip(arch in prop::sample::select(vec![Arch::Aarch64, Arch::Riscv64])) {
			let as_str = arch.as_ref();
			let parsed = Arch::from_str(as_str).unwrap();
			assert_eq!(arch, parsed);
		}

		#[test]
		fn test_target_tuple_format(
			arch in prop::sample::select(vec![Arch::Aarch64, Arch::Riscv64]),
			runs_on in prop::sample::select(vec![RunsOn::Mac, RunsOn::Uefi, RunsOn::Oso, RunsOn::Linux])
		) {
			let opts = Opts {
				build_mode: BuildMode::Debug,
				feature_flags: vec![],
				target: Target { runs_on, arch },
			};

			let target: String = opts.target().into();

			// Should contain arch and runs_on in lowercase
			assert!(target.contains(&arch.as_ref().to_lowercase()));
			assert!(target.contains(&runs_on.as_ref().to_lowercase()));
			assert!(target.contains("unknown"));

			// Should follow the pattern: arch-unknown-os
			let parts: Vec<&str> = target.split('-').collect();
			assert_eq!(parts.len(), 3);
			assert_eq!(parts[1], "unknown");
		}

		#[test]
		fn test_cli_opts_conversion_preserves_values(
			build_mode in prop::option::of(prop::sample::select(vec![BuildMode::Debug, BuildMode::Relese])),
			runs_on in prop::option::of(prop::sample::select(vec![RunsOn::Mac, RunsOn::Uefi, RunsOn::Oso, RunsOn::Linux])),
			arch in prop::option::of(prop::sample::select(vec![Arch::Aarch64, Arch::Riscv64]))
		) {
			let cli = Cli {
				build_mode,
				feature_flags: Some(vec![]),
				runs_on,
				arch,
			};

			let opts = cli.to_opts();

			// Check that values are preserved or defaults are used
			match build_mode {
				Some(bm) => assert_eq!(opts.build_mode, bm),
				None => assert_eq!(opts.build_mode, BuildMode::default()),
			}

			match runs_on {
				Some(ro) => assert_eq!(opts.target.runs_on, ro),
				None => assert_eq!(opts.target.runs_on, RunsOn::default()),
			}

			match arch {
				Some(a) => assert_eq!(opts.target.arch, a),
				None => assert_eq!(opts.target.arch, Arch::default()),
			}
		}
	}

	#[test]
	fn test_enum_value_variants() {
		use clap::ValueEnum;

		// Test BuildMode variants
		let build_modes = BuildMode::value_variants();
		assert_eq!(build_modes.len(), 2);
		assert!(build_modes.contains(&BuildMode::Debug));
		assert!(build_modes.contains(&BuildMode::Relese));

		// Test RunsOn variants
		let runs_on_variants = RunsOn::value_variants();
		assert_eq!(runs_on_variants.len(), 4);
		assert!(runs_on_variants.contains(&RunsOn::Mac));
		assert!(runs_on_variants.contains(&RunsOn::Uefi));
		assert!(runs_on_variants.contains(&RunsOn::Oso));
		assert!(runs_on_variants.contains(&RunsOn::Linux));

		// Test Arch variants
		let arch_variants = Arch::value_variants();
		assert_eq!(arch_variants.len(), 2);
		assert!(arch_variants.contains(&Arch::Aarch64));
		assert!(arch_variants.contains(&Arch::Riscv64));
	}

	#[test]
	fn test_partial_eq_implementations() {
		// Test that enums implement PartialEq correctly
		assert_eq!(BuildMode::Debug, BuildMode::Debug);
		assert_ne!(BuildMode::Debug, BuildMode::Relese);

		assert_eq!(RunsOn::Oso, RunsOn::Oso);
		assert_ne!(RunsOn::Oso, RunsOn::Linux);

		assert_eq!(Arch::Aarch64, Arch::Aarch64);
		assert_ne!(Arch::Aarch64, Arch::Riscv64);
	}

	#[test]
	fn test_edge_cases() {
		// Test empty feature flags
		let opts = Opts {
			build_mode:    BuildMode::Debug,
			feature_flags: vec![],
			target:        Target::default(),
		};

		let flags = opts.feature_flags();
		assert!(flags.is_empty());

		// Test target tuple with default values
		let target: String = opts.target().into();
		assert_eq!(target, "aarch64-unknown-oso");
	}

	#[test]
	fn test_struct_field_access() {
		// Test that all struct fields are accessible
		let cli = Cli {
			build_mode:    Some(BuildMode::Debug,),
			feature_flags: Some(vec![],),
			runs_on:       Some(RunsOn::Linux,),
			arch:          Some(Arch::Riscv64,),
		};

		assert!(cli.build_mode.unwrap().is_debug());
		assert!(cli.feature_flags.unwrap().is_empty());
		assert!(cli.runs_on.unwrap().is_linux());
		assert!(cli.arch.unwrap().is_riscv_64());

		let opts = Opts {
			build_mode:    BuildMode::Relese,
			feature_flags: vec![],
			target:        Target { runs_on: RunsOn::Mac, arch: Arch::Aarch64, },
		};

		assert!(opts.build_mode.is_relese());
		assert!(opts.feature_flags.is_empty());
		assert!(opts.target.runs_on.is_mac());
		assert!(opts.target.arch.is_aarch_64());
	}
}
