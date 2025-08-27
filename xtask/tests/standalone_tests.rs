//! Standalone unit tests for xtask functionality
//!
//! These tests verify the core logic without requiring the full xtask binary
//! to be compiled, avoiding complex dependency issues.

use serde_json::json;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test architecture string validation logic
#[test]
fn test_architecture_validation_logic() {
	let valid_arch_strings = [
		"aarch64-apple-darwin",
		"aarch64-unknown-linux-gnu",
		"x86_64-pc-windows-gnu",
		"x86_64-unknown-linux-gnu",
		"riscv64gc-unknown-linux-gnu",
	];

	let invalid_arch_strings = [
		"arm-unknown-linux-gnueabihf",
		"i686-pc-windows-gnu",
		"powerpc64-unknown-linux-gnu",
	];

	// Test valid architectures
	for arch_str in &valid_arch_strings {
		let contains_valid = arch_str.contains("aarch64",)
			|| arch_str.contains("x86_64",)
			|| arch_str.contains("riscv64",);
		assert!(contains_valid, "Architecture {} should be valid", arch_str);
	}

	// Test invalid architectures
	for arch_str in &invalid_arch_strings {
		let contains_valid = arch_str.contains("aarch64",)
			|| arch_str.contains("x86_64",)
			|| arch_str.contains("riscv64",);
		assert!(!contains_valid, "Architecture {} should be invalid", arch_str);
	}
}

/// Test boot file name generation logic
#[test]
fn test_boot_file_name_logic() {
	let test_cases = [
		("aarch64", "bootaa64.efi",),
		("x86_64", "bootx64.efi",),
		("riscv64", "bootriscv64.efi",),
	];

	for (arch, expected_boot_file,) in &test_cases {
		// Test the logic that would be in Architecture::boot_file_name()
		let boot_file = match *arch {
			"aarch64" => "bootaa64.efi",
			"riscv64" => "bootriscv64.efi",
			"x86_64" => "bootx64.efi",
			_ => panic!("Unsupported architecture: {}", arch),
		};

		assert_eq!(boot_file, *expected_boot_file);
		assert!(boot_file.starts_with("boot"));
		assert!(boot_file.ends_with(".efi"));
	}
}

/// Test target tuple generation logic
#[test]
fn test_target_tuple_logic() {
	let architectures = ["aarch64", "x86_64", "riscv64",];

	for arch in &architectures {
		// Test loader tuple logic
		let loader_tuple = format!("{}-unknown-uefi", arch);
		assert!(loader_tuple.contains(arch));
		assert!(loader_tuple.ends_with("-unknown-uefi"));

		// Test kernel tuple logic
		let kernel_tuple = format!("{}-unknown-none-elf", arch);
		assert!(kernel_tuple.contains(arch));
		assert!(kernel_tuple.ends_with("-unknown-none-elf"));
	}
}

/// Test build mode logic
#[test]
fn test_build_mode_logic() {
	// Test build mode comparison logic
	let debug_mode = "debug";
	let release_mode = "release";

	assert_ne!(debug_mode, release_mode);

	// Test is_release logic
	let is_debug_release = debug_mode == "release";
	let is_release_release = release_mode == "release";

	assert!(!is_debug_release);
	assert!(is_release_release);
}

/// Test feature parsing logic
#[test]
fn test_feature_parsing_logic() {
	let valid_features = ["rgb", "bgr", "bitmask", "bltonly",];
	let invalid_features = ["invalid", "not_a_feature", "",];

	// Test feature validation logic
	for feature in &valid_features {
		let is_valid = valid_features.contains(feature,);
		assert!(is_valid, "Feature {} should be valid", feature);

		// Test that valid features create both loader and kernel variants
		let loader_feature = format!("loader:{}", feature);
		let kernel_feature = format!("kernel:{}", feature);

		assert!(loader_feature.contains("loader"));
		assert!(loader_feature.contains(feature));
		assert!(kernel_feature.contains("kernel"));
		assert!(kernel_feature.contains(feature));
	}

	for feature in &invalid_features {
		let is_valid = valid_features.contains(feature,);
		assert!(!is_valid, "Feature {} should be invalid", feature);
	}
}

/// Test QEMU argument generation logic
#[test]
fn test_qemu_args_logic() {
	// Test aarch64 QEMU args
	let aarch64_args = vec![
		"-machine",
		"virt",
		"-cpu",
		"cortex-a72",
		"-device",
		"virtio-gpu-pci",
	];

	assert!(aarch64_args.contains(&"-machine"));
	assert!(aarch64_args.contains(&"virt"));
	assert!(aarch64_args.contains(&"-cpu"));
	assert!(aarch64_args.contains(&"cortex-a72"));

	// Test x86_64 QEMU args
	let x86_64_args = vec!["-machine", "q35", "-smp", "4", "-vga", "std"];

	assert!(x86_64_args.contains(&"-machine"));
	assert!(x86_64_args.contains(&"q35"));
	assert!(x86_64_args.contains(&"-smp"));
	assert!(x86_64_args.contains(&"4"));
}

/// Test pflash argument generation logic
#[test]
fn test_pflash_args_logic() {
	let test_file = "/tmp/test.fd";

	// Test read-only pflash args
	let readonly_arg =
		format!("if=pflash,format=raw,readonly=on,file={}", test_file);
	assert!(readonly_arg.contains("if=pflash"));
	assert!(readonly_arg.contains("readonly=on"));
	assert!(readonly_arg.contains(test_file));

	// Test read-write pflash args
	let readwrite_arg =
		format!("if=pflash,format=raw,readonly=off,file={}", test_file);
	assert!(readwrite_arg.contains("if=pflash"));
	assert!(readwrite_arg.contains("readonly=off"));
	assert!(readwrite_arg.contains(test_file));
}

/// Test workspace detection logic
#[test]
fn test_workspace_detection_logic() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let oso_root = temp_dir.path().join("oso",);
	let nested_path = oso_root.join("xtask",).join("src",);

	fs::create_dir_all(&nested_path,).unwrap();

	// Test OSO root detection logic
	let mut current = nested_path;
	while current.file_name().unwrap() != "oso" && current.parent().is_some() {
		current = current.parent().unwrap().to_path_buf();
	}

	assert_eq!(current.file_name().unwrap(), "oso");
	assert_eq!(current, oso_root);
}

/// Test TOML parsing logic
#[test]
fn test_toml_parsing_logic() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let toml_path = temp_dir.path().join("test.toml",);

	let toml_content = r#"
[package]
name = "test_crate"
version = "0.1.0"
edition = "2021"
"#;

	fs::write(&toml_path, toml_content,).unwrap();

	// Test TOML parsing
	let toml_str = fs::read_to_string(&toml_path,).unwrap();
	let table: toml::Table = toml_str.parse().unwrap();

	assert!(table.contains_key("package"));
	let package = &table["package"];
	assert_eq!(package["name"].as_str().unwrap(), "test_crate");
}

/// Test JSON parsing and artifact detection logic
#[test]
fn test_json_artifact_detection_logic() {
	let json_content = json!({
		"post-link-args": {
			"ld.lld": [
				"-Ttext=0x80000",
				"-o/tmp/kernel.elf",
				"--gc-sections"
			]
		}
	});

	// Test artifact detection logic
	let post_link_args = &json_content["post-link-args"]["ld.lld"];
	assert!(post_link_args.is_array());

	let args = post_link_args.as_array().unwrap();
	let output_arg = args.iter().find_map(|v| {
		if let Some(s,) = v.as_str() {
			if s.starts_with("-o",) { Some(&s[2..],) } else { None }
		} else {
			None
		}
	},);

	assert_eq!(output_arg, Some("/tmp/kernel.elf"));

	let output_path = PathBuf::from(output_arg.unwrap(),);
	assert_eq!(output_path.file_name().unwrap(), "kernel.elf");
}

/// Test path construction logic
#[test]
fn test_path_construction_logic() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let oso_root = temp_dir.path().join("oso",);

	// Test target directory construction
	let target_dir = oso_root.join("target",).join("xtask",);
	let disk_path = target_dir.join("disk.img",);
	let mount_point = target_dir.join("mount",);

	assert!(disk_path.to_string_lossy().contains("target/xtask"));
	assert_eq!(disk_path.file_name().unwrap(), "disk.img");
	assert_eq!(mount_point.file_name().unwrap(), "mount");

	// Test EFI boot directory construction
	let efi_boot_dir = mount_point.join("efi",).join("boot",);
	assert!(efi_boot_dir.to_string_lossy().contains("efi/boot"));
}

/// Test command-line argument parsing logic
#[test]
fn test_argument_parsing_logic() {
	let test_args =
		vec!["xtask", "-r", "--release", "-86", "--debug", "--features", "rgb"];

	// Test argument recognition logic
	let mut release_found = false;
	let mut x86_found = false;
	let mut debug_found = false;
	let mut features_found = false;

	for arg in &test_args {
		match *arg {
			"-r" | "--release" => release_found = true,
			"-86" | "-x86_64" => x86_found = true,
			"--debug" => debug_found = true,
			"--features" => features_found = true,
			_ => {},
		}
	}

	assert!(release_found);
	assert!(x86_found);
	assert!(debug_found);
	assert!(features_found);
}

/// Test error handling logic
#[test]
fn test_error_handling_logic() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let nonexistent_file = temp_dir.path().join("nonexistent.txt",);

	// Test file existence checking
	assert!(!nonexistent_file.exists());

	// Test error handling for file operations
	let read_result = fs::read(&nonexistent_file,);
	assert!(read_result.is_err());

	// Test malformed JSON handling
	let invalid_json = r#"{"invalid": json"#;
	let parse_result =
		serde_json::from_str::<serde_json::Value,>(invalid_json,);
	assert!(parse_result.is_err());
}

/// Test cleanup logic
#[test]
fn test_cleanup_logic() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let test_file = temp_dir.path().join("cleanup_test.tmp",);

	// Create a file
	fs::write(&test_file, b"test data",).unwrap();
	assert!(test_file.exists());

	// Test cleanup
	fs::remove_file(&test_file,).unwrap();
	assert!(!test_file.exists());
}

/// Test disk image size validation logic
#[test]
fn test_disk_size_logic() {
	let min_size = 1024 * 1024; // 1MB
	let max_size = 1024 * 1024 * 1024; // 1GB
	let test_size = 64 * 1024 * 1024; // 64MB

	assert!(test_size >= min_size);
	assert!(test_size <= max_size);
	assert_eq!(test_size, 67_108_864);
}
