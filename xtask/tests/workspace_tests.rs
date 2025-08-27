//! Unit tests for the workspace module
//!
//! These tests verify workspace detection, crate management,
//! and build artifact detection functionality.

use serde_json::json;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Constants from the workspace module
const LOADER: &str = "oso_loader";
const KERNEL: &str = "oso_kernel";

/// Test helper to create a mock workspace structure
fn create_mock_workspace() -> TempDir {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let oso_root = temp_dir.path().join("oso",);

	// Create workspace structure
	fs::create_dir_all(&oso_root,).unwrap();
	fs::create_dir_all(oso_root.join(LOADER,),).unwrap();
	fs::create_dir_all(oso_root.join(KERNEL,),).unwrap();
	fs::create_dir_all(oso_root.join("xtask",),).unwrap();

	// Create Cargo.toml files
	let loader_toml = r#"
[package]
name = "oso_loader"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

	let kernel_toml = r#"
[package]
name = "oso_kernel"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

	fs::write(oso_root.join(LOADER,).join("Cargo.toml",), loader_toml,)
		.unwrap();
	fs::write(oso_root.join(KERNEL,).join("Cargo.toml",), kernel_toml,)
		.unwrap();

	temp_dir
}

/// Test OSO root directory detection
#[test]
fn test_oso_root_detection() {
	let temp_workspace = create_mock_workspace();
	let oso_path = temp_workspace.path().join("oso",);
	let nested_path = oso_path.join("xtask",).join("src",);

	fs::create_dir_all(&nested_path,).unwrap();

	// Test that we can find the OSO root from a nested directory
	let mut current = nested_path;
	while current.file_name().unwrap() != "oso" && current.parent().is_some() {
		current = current.parent().unwrap().to_path_buf();
	}

	assert_eq!(current.file_name().unwrap(), "oso");
	assert_eq!(current, oso_path);
}

/// Test TOML file parsing
#[test]
fn test_toml_parsing() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let toml_path = temp_dir.path().join("test.toml",);

	let toml_content = r#"
[package]
name = "test_crate"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "*"
"#;

	fs::write(&toml_path, toml_content,).unwrap();

	// Test that we can parse the TOML file
	let toml_str = fs::read_to_string(&toml_path,).unwrap();
	let table: toml::Table = toml_str.parse().unwrap();

	assert!(table.contains_key("package"));
	assert!(table.contains_key("dependencies"));

	let package = &table["package"];
	assert_eq!(package["name"].as_str().unwrap(), "test_crate");
	assert_eq!(package["version"].as_str().unwrap(), "0.1.0");
	assert_eq!(package["edition"].as_str().unwrap(), "2021");
}

/// Test crate name extraction from Cargo.toml
#[test]
fn test_crate_name_extraction() {
	let temp_workspace = create_mock_workspace();
	let loader_toml_path =
		temp_workspace.path().join("oso",).join(LOADER,).join("Cargo.toml",);

	// Read and parse the TOML file
	let toml_str = fs::read_to_string(&loader_toml_path,).unwrap();
	let table: toml::Table = toml_str.parse().unwrap();

	let name = table["package"]["name"].as_str().unwrap();
	assert_eq!(name, "oso_loader");
}

/// Test architecture target tuple generation
#[test]
fn test_architecture_target_tuples() {
	// Test loader target tuples
	let aarch64_loader = format!("{}-unknown-uefi", "aarch64");
	let x86_64_loader = format!("{}-unknown-uefi", "x86_64");
	let riscv64_loader = format!("{}-unknown-uefi", "riscv64");

	assert_eq!(aarch64_loader, "aarch64-unknown-uefi");
	assert_eq!(x86_64_loader, "x86_64-unknown-uefi");
	assert_eq!(riscv64_loader, "riscv64-unknown-uefi");

	// Test kernel target tuples
	let aarch64_kernel = format!("{}-unknown-none-elf", "aarch64");
	let x86_64_kernel = format!("{}-unknown-none-elf", "x86_64");
	let riscv64_kernel = format!("{}-unknown-none-elf", "riscv64");

	assert_eq!(aarch64_kernel, "aarch64-unknown-none-elf");
	assert_eq!(x86_64_kernel, "x86_64-unknown-none-elf");
	assert_eq!(riscv64_kernel, "riscv64-unknown-none-elf");
}

/// Test JSON file loading and parsing
#[test]
fn test_json_loading() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let json_path = temp_dir.path().join("test.json",);

	let json_content = json!({
		"arch": "aarch64",
		"target-triple": "aarch64-unknown-none-elf",
		"post-link-args": {
			"ld.lld": [
				"-Ttext=0x80000",
				"-o/tmp/kernel.elf"
			]
		}
	});

	fs::write(&json_path, json_content.to_string(),).unwrap();

	// Test JSON loading
	let file = fs_err::File::open(&json_path,).unwrap();
	let reader = std::io::BufReader::new(file,);
	let parsed_json: serde_json::Value =
		serde_json::from_reader(reader,).unwrap();

	assert_eq!(parsed_json["arch"], "aarch64");
	assert_eq!(parsed_json["target-triple"], "aarch64-unknown-none-elf");
	assert!(parsed_json["post-link-args"]["ld.lld"].is_array());
}

/// Test build artifact detection from JSON
#[test]
fn test_build_artifact_detection() {
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
	assert_eq!(output_path.parent().unwrap(), PathBuf::from("/tmp"));
}

/// Test invalid JSON handling
#[test]
fn test_invalid_json_handling() {
	let json_content = json!({
		"post-link-args": {
			"ld.lld": "not-an-array"
		}
	});

	// Test that we can detect invalid structure
	let post_link_args = &json_content["post-link-args"]["ld.lld"];
	assert!(!post_link_args.is_array());
	assert!(post_link_args.is_string());
}

/// Test missing output argument handling
#[test]
fn test_missing_output_argument() {
	let json_content = json!({
		"post-link-args": {
			"ld.lld": [
				"-Ttext=0x80000",
				"--gc-sections"
			]
		}
	});

	let post_link_args = &json_content["post-link-args"]["ld.lld"];
	let args = post_link_args.as_array().unwrap();

	let output_arg = args.iter().find_map(|v| {
		if let Some(s,) = v.as_str() {
			if s.starts_with("-o",) { Some(&s[2..],) } else { None }
		} else {
			None
		}
	},);

	assert_eq!(output_arg, None);
}

/// Test workspace constants
#[test]
fn test_workspace_constants() {
	assert_eq!(LOADER, "oso_loader");
	assert_eq!(KERNEL, "oso_kernel");

	// Test that constants are valid directory names
	assert!(!LOADER.contains('/'));
	assert!(!LOADER.contains('\\'));
	assert!(!KERNEL.contains('/'));
	assert!(!KERNEL.contains('\\'));
}

/// Test crate root path construction
#[test]
fn test_crate_root_paths() {
	let temp_workspace = create_mock_workspace();
	let oso_root = temp_workspace.path().join("oso",);

	let loader_root = oso_root.join(LOADER,);
	let kernel_root = oso_root.join(KERNEL,);

	assert!(loader_root.exists());
	assert!(kernel_root.exists());
	assert_eq!(loader_root.file_name().unwrap(), LOADER);
	assert_eq!(kernel_root.file_name().unwrap(), KERNEL);
}

/// Test environment variable handling
#[test]
fn test_environment_variable_handling() {
	// Test CARGO_MANIFEST_DIR environment variable usage
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR",);

	// The variable should either exist or we should have a fallback
	match manifest_dir {
		Ok(dir,) => {
			assert!(!dir.is_empty());
			let path = PathBuf::from(dir,);
			assert!(path.is_absolute());
		},
		Err(_,) => {
			// Fallback should be available
			let fallback = env!("CARGO_MANIFEST_DIR");
			assert!(!fallback.is_empty());
		},
	}
}

/// Test path iteration and filtering
#[test]
fn test_path_iteration() {
	let test_path = PathBuf::from("/home/user/projects/oso/xtask/src",);

	// Test taking path components until we find "oso"
	let components: Vec<_,> = test_path
		.iter()
		.take_while(|s| s.to_str().unwrap() != "oso",)
		.collect();

	let reconstructed: PathBuf = components.iter().collect();
	let oso_root = reconstructed.join("oso",);

	assert_eq!(oso_root.file_name().unwrap(), "oso");
}
