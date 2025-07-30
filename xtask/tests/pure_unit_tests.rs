//! Pure unit tests for xtask logic
//!
//! These tests verify the core logic without any dependencies on the xtask crate itself.
//! They test the algorithms and logic patterns used in xtask.

use std::path::PathBuf;
use tempfile::TempDir;

/// Test architecture string validation logic (pure function)
#[test]
fn test_architecture_validation() {
	fn is_supported_arch(target: &str,) -> bool {
		target.contains("aarch64",) || target.contains("x86_64",) || target.contains("riscv64",)
	}

	let valid_targets = [
		"aarch64-apple-darwin",
		"aarch64-unknown-linux-gnu",
		"x86_64-pc-windows-gnu",
		"x86_64-unknown-linux-gnu",
		"riscv64gc-unknown-linux-gnu",
	];

	let invalid_targets =
		["arm-unknown-linux-gnueabihf", "i686-pc-windows-gnu", "powerpc64-unknown-linux-gnu",];

	for target in &valid_targets {
		assert!(is_supported_arch(target), "Target {} should be supported", target);
	}

	for target in &invalid_targets {
		assert!(!is_supported_arch(target), "Target {} should not be supported", target);
	}
}

/// Test boot file name generation logic (pure function)
#[test]
fn test_boot_file_generation() {
	fn get_boot_file_name(arch: &str,) -> &'static str {
		match arch {
			"aarch64" => "bootaa64.efi",
			"riscv64" => "bootriscv64.efi",
			"x86_64" => "bootx64.efi",
			_ => panic!("Unsupported architecture: {}", arch),
		}
	}

	assert_eq!(get_boot_file_name("aarch64"), "bootaa64.efi");
	assert_eq!(get_boot_file_name("riscv64"), "bootriscv64.efi");
	assert_eq!(get_boot_file_name("x86_64"), "bootx64.efi");

	// Test that all boot files follow the expected pattern
	let architectures = ["aarch64", "riscv64", "x86_64",];
	for arch in &architectures {
		let boot_file = get_boot_file_name(arch,);
		assert!(boot_file.starts_with("boot"));
		assert!(boot_file.ends_with(".efi"));
	}
}

/// Test target tuple generation logic (pure function)
#[test]
fn test_target_tuple_generation() {
	fn loader_tuple(arch: &str,) -> String {
		format!("{}-unknown-uefi", arch)
	}

	fn kernel_tuple(arch: &str,) -> String {
		format!("{}-unknown-none-elf", arch)
	}

	let architectures = ["aarch64", "x86_64", "riscv64",];

	for arch in &architectures {
		let loader = loader_tuple(arch,);
		let kernel = kernel_tuple(arch,);

		assert!(loader.starts_with(arch));
		assert!(loader.ends_with("-unknown-uefi"));

		assert!(kernel.starts_with(arch));
		assert!(kernel.ends_with("-unknown-none-elf"));
	}
}

/// Test build mode logic (pure function)
#[test]
fn test_build_mode_logic() {
	#[derive(PartialEq, Debug,)]
	enum BuildMode {
		Release,
		Debug,
	}

	impl BuildMode {
		fn is_release(&self,) -> bool {
			self == &BuildMode::Release
		}

		fn to_string(&self,) -> &'static str {
			match self {
				BuildMode::Release => "release",
				BuildMode::Debug => "debug",
			}
		}
	}

	let debug_mode = BuildMode::Debug;
	let release_mode = BuildMode::Release;

	assert!(!debug_mode.is_release());
	assert!(release_mode.is_release());

	assert_eq!(debug_mode.to_string(), "debug");
	assert_eq!(release_mode.to_string(), "release");
}

/// Test feature parsing logic (pure function)
#[test]
fn test_feature_parsing() {
	fn parse_features(feature: &str,) -> Vec<String,> {
		match feature {
			f if ["rgb", "bgr", "bitmask", "bltonly",].contains(&f,) => {
				vec![format!("loader:{}", f), format!("kernel:{}", f)]
			},
			_ => vec![],
		}
	}

	let valid_features = ["rgb", "bgr", "bitmask", "bltonly",];
	let invalid_features = ["invalid", "not_a_feature", "",];

	for feature in &valid_features {
		let parsed = parse_features(feature,);
		assert_eq!(parsed.len(), 2);
		assert!(parsed[0].contains("loader"));
		assert!(parsed[0].contains(feature));
		assert!(parsed[1].contains("kernel"));
		assert!(parsed[1].contains(feature));
	}

	for feature in &invalid_features {
		let parsed = parse_features(feature,);
		assert_eq!(parsed.len(), 0);
	}
}

/// Test QEMU argument generation logic (pure function)
#[test]
fn test_qemu_args_generation() {
	fn basic_qemu_args(arch: &str,) -> Vec<&'static str,> {
		match arch {
			"aarch64" => {
				vec!["-machine", "virt", "-cpu", "cortex-a72", "-device", "virtio-gpu-pci"]
			},
			"x86_64" => vec!["-machine", "q35", "-smp", "4", "-vga", "std"],
			"riscv64" => vec![], // Not implemented yet
			_ => panic!("Unsupported architecture: {}", arch),
		}
	}

	let aarch64_args = basic_qemu_args("aarch64",);
	assert!(aarch64_args.contains(&"-machine"));
	assert!(aarch64_args.contains(&"virt"));
	assert!(aarch64_args.contains(&"-cpu"));
	assert!(aarch64_args.contains(&"cortex-a72"));

	let x86_64_args = basic_qemu_args("x86_64",);
	assert!(x86_64_args.contains(&"-machine"));
	assert!(x86_64_args.contains(&"q35"));
	assert!(x86_64_args.contains(&"-smp"));
	assert!(x86_64_args.contains(&"4"));
}

/// Test pflash argument generation logic (pure function)
#[test]
fn test_pflash_args() {
	fn pflash_arg(file_path: &str, readonly: bool,) -> String {
		format!(
			"if=pflash,format=raw,readonly={},file={}",
			if readonly { "on" } else { "off" },
			file_path
		)
	}

	let readonly_arg = pflash_arg("/tmp/code.fd", true,);
	assert!(readonly_arg.contains("readonly=on"));
	assert!(readonly_arg.contains("/tmp/code.fd"));

	let readwrite_arg = pflash_arg("/tmp/vars.fd", false,);
	assert!(readwrite_arg.contains("readonly=off"));
	assert!(readwrite_arg.contains("/tmp/vars.fd"));
}

/// Test workspace path detection logic (pure function)
#[test]
fn test_workspace_path_logic() {
	fn find_oso_root(path: &PathBuf,) -> Option<PathBuf,> {
		let components: Vec<_,> =
			path.iter().take_while(|s| s.to_str().unwrap() != "oso",).collect();

		if components.len() < path.iter().count() {
			let root: PathBuf = components.iter().collect();
			Some(root.join("oso",),)
		} else {
			None
		}
	}

	let test_path = PathBuf::from("/home/user/projects/oso/xtask/src",);
	let oso_root = find_oso_root(&test_path,).unwrap();
	assert_eq!(oso_root.file_name().unwrap(), "oso");

	let non_oso_path = PathBuf::from("/home/user/projects/other/src",);
	let result = find_oso_root(&non_oso_path,);
	assert!(result.is_none());
}

/// Test JSON artifact detection logic (pure function)
#[test]
fn test_json_artifact_detection() {
	fn extract_output_path(args: &[&str],) -> Option<String,> {
		args.iter()
			.find_map(|arg| if arg.starts_with("-o",) { Some(arg[2..].to_string(),) } else { None },)
	}

	let test_args = ["-Ttext=0x80000", "-o/tmp/kernel.elf", "--gc-sections",];
	let output = extract_output_path(&test_args,);
	assert_eq!(output, Some("/tmp/kernel.elf".to_string()));

	let no_output_args = ["-Ttext=0x80000", "--gc-sections",];
	let no_output = extract_output_path(&no_output_args,);
	assert_eq!(no_output, None);
}

/// Test command-line argument parsing logic (pure function)
#[test]
fn test_cli_parsing() {
	#[derive(Debug, Default,)]
	struct Options {
		release:  bool,
		x86_64:   bool,
		debug:    bool,
		features: Vec<String,>,
	}

	fn parse_args(args: &[&str],) -> Options {
		let mut opts = Options::default();
		let mut in_features = false;

		for arg in args {
			match *arg {
				"-r" | "--release" => opts.release = true,
				"-86" | "-x86_64" => opts.x86_64 = true,
				"--debug" => opts.debug = true,
				"--features" => in_features = true,
				feature if in_features => {
					opts.features.push(feature.to_string(),);
					in_features = false;
				},
				_ => in_features = false,
			}
		}

		opts
	}

	let test_args = ["xtask", "-r", "--debug", "-86", "--features", "rgb",];
	let opts = parse_args(&test_args,);

	assert!(opts.release);
	assert!(opts.debug);
	assert!(opts.x86_64);
	assert_eq!(opts.features, vec!["rgb"]);
}

/// Test path construction logic (pure function)
#[test]
fn test_path_construction() {
	fn construct_paths(oso_root: &PathBuf,) -> (PathBuf, PathBuf, PathBuf,) {
		let target_dir = oso_root.join("target",).join("xtask",);
		let disk_path = target_dir.join("disk.img",);
		let mount_point = target_dir.join("mount",);

		(target_dir, disk_path, mount_point,)
	}

	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let oso_root = temp_dir.path().join("oso",);

	let (target_dir, disk_path, mount_point,) = construct_paths(&oso_root,);

	assert!(target_dir.to_string_lossy().contains("target/xtask"));
	assert_eq!(disk_path.file_name().unwrap(), "disk.img");
	assert_eq!(mount_point.file_name().unwrap(), "mount");
}

/// Test file validation logic (pure function)
#[test]
fn test_file_validation() {
	fn validate_file_extension(path: &str, expected_ext: &str,) -> bool {
		path.ends_with(expected_ext,)
	}

	fn validate_file_prefix(path: &str, expected_prefix: &str,) -> bool {
		PathBuf::from(path,)
			.file_name()
			.and_then(|name| name.to_str(),)
			.map(|name| name.starts_with(expected_prefix,),)
			.unwrap_or(false,)
	}

	assert!(validate_file_extension("bootaa64.efi", ".efi"));
	assert!(validate_file_extension("kernel.elf", ".elf"));
	assert!(!validate_file_extension("kernel.bin", ".elf"));

	assert!(validate_file_prefix("bootaa64.efi", "boot"));
	assert!(validate_file_prefix("oso_kernel.elf", "oso_"));
	assert!(!validate_file_prefix("other_kernel.elf", "oso_"));
}

/// Test size validation logic (pure function)
#[test]
fn test_size_validation() {
	fn validate_disk_size(size_bytes: u64,) -> bool {
		let min_size = 1024 * 1024; // 1MB
		let max_size = 1024 * 1024 * 1024; // 1GB
		size_bytes >= min_size && size_bytes <= max_size
	}

	assert!(validate_disk_size(64 * 1024 * 1024)); // 64MB - valid
	assert!(!validate_disk_size(512 * 1024)); // 512KB - too small
	assert!(!validate_disk_size(2 * 1024 * 1024 * 1024)); // 2GB - too large
}

/// Test error condition detection (pure function)
#[test]
fn test_error_detection() {
	fn detect_json_structure_error(json_str: &str,) -> bool {
		serde_json::from_str::<serde_json::Value,>(json_str,).is_err()
	}

	fn detect_missing_field(json: &serde_json::Value, field: &str,) -> bool {
		json.get(field,).is_none()
	}

	let valid_json = r#"{"field": "value"}"#;
	let invalid_json = r#"{"field": value"#; // Missing quotes

	assert!(!detect_json_structure_error(valid_json));
	assert!(detect_json_structure_error(invalid_json));

	let json_value: serde_json::Value = serde_json::from_str(valid_json,).unwrap();
	assert!(!detect_missing_field(&json_value, "field"));
	assert!(detect_missing_field(&json_value, "missing_field"));
}

/// Test cleanup logic (pure function)
#[test]
fn test_cleanup_logic() {
	fn should_cleanup_file(path: &PathBuf,) -> bool {
		path.extension()
			.and_then(|ext| ext.to_str(),)
			.map(|ext| ["tmp", "img", "temp",].contains(&ext,),)
			.unwrap_or(false,)
	}

	assert!(should_cleanup_file(&PathBuf::from("test.tmp")));
	assert!(should_cleanup_file(&PathBuf::from("disk.img")));
	assert!(!should_cleanup_file(&PathBuf::from("important.txt")));
	assert!(!should_cleanup_file(&PathBuf::from("config.toml")));
}

/// Test configuration validation (pure function)
#[test]
fn test_config_validation() {
	fn validate_port(port: u16,) -> bool {
		port > 1024
	}

	fn validate_arch_combo(arch: &str, target: &str,) -> bool {
		target.contains(arch,)
	}

	assert!(validate_port(12345)); // Valid debug port
	assert!(!validate_port(80)); // Privileged port

	assert!(validate_arch_combo("aarch64", "aarch64-unknown-uefi"));
	assert!(!validate_arch_combo("aarch64", "x86_64-unknown-uefi"));
}
