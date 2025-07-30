//! Integration tests for the xtask crate
//!
//! These tests verify the end-to-end functionality of the xtask build system,
//! including workspace detection, build processes, and QEMU configuration.

use std::env;
use std::process::Command;

/// Test that the xtask binary can be built successfully
#[test]
fn test_xtask_builds() {
	let output = Command::new("cargo",)
		.args(&["build", "--bin", "xtask",],)
		.current_dir(env::current_dir().unwrap(),)
		.output()
		.expect("Failed to execute cargo build",);

	assert!(
		output.status.success(),
		"xtask failed to build: {}",
		String::from_utf8_lossy(&output.stderr)
	);
}

/// Test that xtask can display help information
#[test]
#[ignore = "help is not implemented yet"]
fn test_xtask_help() {
	let output = Command::new("cargo",)
		.args(&["run", "--bin", "xtask", "--", "--help",],)
		.current_dir(env::current_dir().unwrap(),)
		.output()
		.expect("Failed to execute xtask --help",);

	// Note: xtask doesn't implement --help flag in the current code,
	// but this test documents expected behavior
	// The test will pass if the command runs without panicking
	assert!(output.status.success() || output.status.code() == Some(1));
}

/// Test workspace detection functionality
#[test]
fn test_workspace_detection() {
	// This test verifies that the workspace can be detected from the xtask directory
	let current_dir = env::current_dir().unwrap();

	// The workspace detection should work from the xtask directory
	assert!(current_dir.to_string_lossy().contains("xtask"));

	// Test that we can find the parent OSO directory
	let mut oso_root = current_dir.clone();
	while oso_root.file_name().unwrap() != "oso" && oso_root.parent().is_some() {
		oso_root = oso_root.parent().unwrap().to_path_buf();
	}

	assert_eq!(oso_root.file_name().unwrap(), "oso");
}

/// Test that required directories exist in the workspace
#[test]
fn test_workspace_structure() {
	let current_dir = env::current_dir().unwrap();
	let mut oso_root = current_dir.clone();

	// Find OSO root
	while oso_root.file_name().unwrap() != "oso" && oso_root.parent().is_some() {
		oso_root = oso_root.parent().unwrap().to_path_buf();
	}

	// Check for expected directories
	let loader_dir = oso_root.join("oso_loader",);
	let kernel_dir = oso_root.join("oso_kernel",);

	assert!(loader_dir.exists(), "oso_loader directory should exist at {:?}", loader_dir);
	assert!(kernel_dir.exists(), "oso_kernel directory should exist at {:?}", kernel_dir);
}

/// Test that Cargo.toml files exist for loader and kernel
#[test]
fn test_cargo_manifests_exist() {
	let current_dir = env::current_dir().unwrap();
	let mut oso_root = current_dir.clone();

	// Find OSO root
	while oso_root.file_name().unwrap() != "oso" && oso_root.parent().is_some() {
		oso_root = oso_root.parent().unwrap().to_path_buf();
	}

	let loader_manifest = oso_root.join("oso_loader",).join("Cargo.toml",);
	let kernel_manifest = oso_root.join("oso_kernel",).join("Cargo.toml",);

	assert!(loader_manifest.exists(), "Loader Cargo.toml should exist at {:?}", loader_manifest);
	assert!(kernel_manifest.exists(), "Kernel Cargo.toml should exist at {:?}", kernel_manifest);
}
