//! Test runner and configuration for xtask tests
//!
//! This module provides utilities for running tests and setting up
//! test environments for the xtask crate.

use std::env;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Test configuration struct
pub struct TestConfig {
	pub temp_dir:     TempDir,
	pub oso_root:     PathBuf,
	pub original_dir: PathBuf,
}

impl TestConfig {
	/// Creates a new test configuration with a temporary workspace
	pub fn new() -> Self {
		let temp_dir = TempDir::new().expect("Failed to create temp directory",);
		let oso_root = temp_dir.path().join("oso",);
		let original_dir = env::current_dir().expect("Failed to get current directory",);

		// Create basic workspace structure
		std::fs::create_dir_all(&oso_root,).unwrap();
		std::fs::create_dir_all(oso_root.join("oso_loader",),).unwrap();
		std::fs::create_dir_all(oso_root.join("oso_kernel",),).unwrap();
		std::fs::create_dir_all(oso_root.join("xtask",),).unwrap();
		std::fs::create_dir_all(oso_root.join("target",),).unwrap();

		Self { temp_dir, oso_root, original_dir, }
	}

	/// Creates minimal Cargo.toml files for the workspace
	pub fn create_cargo_manifests(&self,) {
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

		let xtask_toml = r#"
[package]
name = "xtask"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "*"
colored = "*"
"#;

		std::fs::write(self.oso_root.join("oso_loader",).join("Cargo.toml",), loader_toml,)
			.unwrap();

		std::fs::write(self.oso_root.join("oso_kernel",).join("Cargo.toml",), kernel_toml,)
			.unwrap();

		std::fs::write(self.oso_root.join("xtask",).join("Cargo.toml",), xtask_toml,).unwrap();
	}

	/// Creates mock target JSON files
	pub fn create_target_files(&self,) {
		let aarch64_target = serde_json::json!({
			"arch": "aarch64",
			"target-triple": "aarch64-unknown-none-elf",
			"post-link-args": {
				"ld.lld": [
					"-Ttext=0x80000",
					"-o/tmp/kernel_aarch64.elf"
				]
			}
		});

		let x86_64_target = serde_json::json!({
			"arch": "x86_64",
			"target-triple": "x86_64-unknown-none-elf",
			"post-link-args": {
				"ld.lld": [
					"-Ttext=0x100000",
					"-o/tmp/kernel_x86_64.elf"
				]
			}
		});

		let target_dir = self.oso_root.join("target",);
		std::fs::write(
			target_dir.join("aarch64-unknown-none-elf.json",),
			aarch64_target.to_string(),
		)
		.unwrap();

		std::fs::write(
            target_dir.join("x86_64-unknown-none-elf.json"),
            x86_64_target.to_string(),
        ).unwrap();
	}

	/// Creates mock build artifacts
	pub fn create_build_artifacts(&self,) {
		let target_dir = self.oso_root.join("target",);

		// Create loader artifacts
		let loader_debug_dir = target_dir.join("aarch64-unknown-uefi",).join("debug",);
		std::fs::create_dir_all(&loader_debug_dir,).unwrap();
		std::fs::write(loader_debug_dir.join("oso_loader.efi",), b"mock loader",).unwrap();

		let loader_release_dir = target_dir.join("aarch64-unknown-uefi",).join("release",);
		std::fs::create_dir_all(&loader_release_dir,).unwrap();
		std::fs::write(loader_release_dir.join("oso_loader.efi",), b"mock loader release",)
			.unwrap();

		// Create kernel artifacts
		let kernel_debug_dir = target_dir.join("aarch64-unknown-none-elf",).join("debug",);
		std::fs::create_dir_all(&kernel_debug_dir,).unwrap();
		std::fs::write(kernel_debug_dir.join("oso_kernel",), b"mock kernel",).unwrap();

		let kernel_release_dir = target_dir.join("aarch64-unknown-none-elf",).join("release",);
		std::fs::create_dir_all(&kernel_release_dir,).unwrap();
		std::fs::write(kernel_release_dir.join("oso_kernel",), b"mock kernel release",).unwrap();
	}
}

/// Test utilities
pub struct TestUtils;

impl TestUtils {
	/// Checks if a command exists in PATH
	pub fn command_exists(cmd: &str,) -> bool {
		Command::new("which",)
			.arg(cmd,)
			.output()
			.map(|output| output.status.success(),)
			.unwrap_or(false,)
	}

	/// Checks if QEMU is available for testing
	pub fn qemu_available() -> bool {
		Self::command_exists("qemu-system-aarch64",) || Self::command_exists("qemu-system-x86_64",)
	}

	/// Checks if required build tools are available
	pub fn build_tools_available() -> bool {
		Self::command_exists("cargo",) && Self::command_exists("rustc",)
	}

	/// Creates a mock disk image file
	pub fn create_mock_disk_image(path: &PathBuf, size_mb: u64,) {
		let size_bytes = size_mb * 1024 * 1024;
		let data = vec![0u8; size_bytes as usize];
		std::fs::write(path, data,).unwrap();
	}

	/// Creates a mock firmware file
	pub fn create_mock_firmware(path: &PathBuf,) {
		std::fs::write(path, b"mock firmware data",).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_config_creation() {
		let config = TestConfig::new();

		assert!(config.oso_root.exists());
		assert!(config.oso_root.join("oso_loader").exists());
		assert!(config.oso_root.join("oso_kernel").exists());
		assert!(config.oso_root.join("xtask").exists());
	}

	#[test]
	fn test_cargo_manifest_creation() {
		let config = TestConfig::new();
		config.create_cargo_manifests();

		let loader_manifest = config.oso_root.join("oso_loader",).join("Cargo.toml",);
		let kernel_manifest = config.oso_root.join("oso_kernel",).join("Cargo.toml",);

		assert!(loader_manifest.exists());
		assert!(kernel_manifest.exists());

		let loader_content = std::fs::read_to_string(&loader_manifest,).unwrap();
		assert!(loader_content.contains("oso_loader"));
	}

	#[test]
	fn test_target_file_creation() {
		let config = TestConfig::new();
		config.create_target_files();

		let aarch64_target = config.oso_root.join("target",).join("aarch64-unknown-none-elf.json",);
		let x86_64_target = config.oso_root.join("target",).join("x86_64-unknown-none-elf.json",);

		assert!(aarch64_target.exists());
		assert!(x86_64_target.exists());
	}

	#[test]
	fn test_build_artifact_creation() {
		let config = TestConfig::new();
		config.create_build_artifacts();

		let loader_artifact = config
			.oso_root
			.join("target",)
			.join("aarch64-unknown-uefi",)
			.join("debug",)
			.join("oso_loader.efi",);

		assert!(loader_artifact.exists());

		let content = std::fs::read(&loader_artifact,).unwrap();
		assert_eq!(content, b"mock loader");
	}

	#[test]
	fn test_command_existence_check() {
		// Test with a command that should exist
		assert!(TestUtils::command_exists("ls") || TestUtils::command_exists("dir"));

		// Test with a command that shouldn't exist
		assert!(!TestUtils::command_exists("definitely_not_a_real_command_12345"));
	}

	#[test]
	fn test_mock_disk_image_creation() {
		let temp_dir = TempDir::new().unwrap();
		let disk_path = temp_dir.path().join("test_disk.img",);

		TestUtils::create_mock_disk_image(&disk_path, 1,); // 1MB

		assert!(disk_path.exists());
		let metadata = std::fs::metadata(&disk_path,).unwrap();
		assert_eq!(metadata.len(), 1024 * 1024);
	}

	#[test]
	fn test_mock_firmware_creation() {
		let temp_dir = TempDir::new().unwrap();
		let firmware_path = temp_dir.path().join("OVMF_CODE.fd",);

		TestUtils::create_mock_firmware(&firmware_path,);

		assert!(firmware_path.exists());
		let content = std::fs::read(&firmware_path,).unwrap();
		assert_eq!(content, b"mock firmware data");
	}
}
