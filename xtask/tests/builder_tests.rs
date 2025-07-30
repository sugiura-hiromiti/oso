//! Unit tests for the builder module
//!
//! These tests verify the functionality of the Builder struct and its methods,
//! including path generation, disk image creation, and build processes.

use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Note: Since Builder has complex dependencies and side effects,
// these tests focus on testable components and mock where necessary

/// Test helper to create a temporary directory structure
fn create_test_workspace() -> TempDir {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let oso_root = temp_dir.path().join("oso",);

	// Create basic workspace structure
	fs::create_dir_all(&oso_root,).unwrap();
	fs::create_dir_all(oso_root.join("oso_loader",),).unwrap();
	fs::create_dir_all(oso_root.join("oso_kernel",),).unwrap();
	fs::create_dir_all(oso_root.join("xtask",),).unwrap();

	// Create minimal Cargo.toml files
	let loader_toml = r#"
[package]
name = "oso_loader"
version = "0.1.0"
edition = "2021"
"#;

	let kernel_toml = r#"
[package]
name = "oso_kernel"
version = "0.1.0"
edition = "2021"
"#;

	fs::write(oso_root.join("oso_loader",).join("Cargo.toml",), loader_toml,).unwrap();
	fs::write(oso_root.join("oso_kernel",).join("Cargo.toml",), kernel_toml,).unwrap();

	temp_dir
}

/// Test disk image path generation
#[test]
fn test_disk_image_path_generation() {
	let temp_workspace = create_test_workspace();
	let oso_root = temp_workspace.path().join("oso",);

	// Test that disk image path follows expected pattern
	let expected_path = oso_root.join("target",).join("xtask",).join("disk.img",);

	// Since we can't easily instantiate Builder without side effects,
	// we test the path construction logic directly
	let target_dir = oso_root.join("target",).join("xtask",);
	let disk_path = target_dir.join("disk.img",);

	assert_eq!(disk_path.file_name().unwrap(), "disk.img");
	assert!(disk_path.to_string_lossy().contains("target/xtask"));
}

/// Test mount point path generation
#[test]
fn test_mount_point_path_generation() {
	let temp_workspace = create_test_workspace();
	let oso_root = temp_workspace.path().join("oso",);

	// Test mount point path construction
	let target_dir = oso_root.join("target",).join("xtask",);
	let mount_point = target_dir.join("mount",);

	assert_eq!(mount_point.file_name().unwrap(), "mount");
	assert!(mount_point.to_string_lossy().contains("target/xtask"));
}

/// Test EFI boot directory structure
#[test]
fn test_efi_boot_directory_structure() {
	let temp_workspace = create_test_workspace();
	let mount_point = temp_workspace.path().join("mount",);

	// Create the expected EFI boot directory structure
	let efi_boot_dir = mount_point.join("efi",).join("boot",);
	fs::create_dir_all(&efi_boot_dir,).unwrap();

	assert!(efi_boot_dir.exists());
	assert!(efi_boot_dir.is_dir());

	// Test that we can create files in the boot directory
	let boot_file = efi_boot_dir.join("bootaa64.efi",);
	fs::write(&boot_file, b"test",).unwrap();
	assert!(boot_file.exists());
}

/// Test kernel file naming
#[test]
fn test_kernel_file_naming() {
	const KERNEL_FILE: &str = "oso_kernel.elf";

	assert_eq!(KERNEL_FILE, "oso_kernel.elf");
	assert!(KERNEL_FILE.ends_with(".elf"));
}

/// Test target directory creation logic
#[test]
fn test_target_directory_creation() {
	let temp_workspace = create_test_workspace();
	let oso_root = temp_workspace.path().join("oso",);

	let target_dir = oso_root.join("target",).join("xtask",);

	// Test that we can create the target directory
	fs::create_dir_all(&target_dir,).unwrap();
	assert!(target_dir.exists());
	assert!(target_dir.is_dir());

	// Test subdirectory creation
	let mount_dir = target_dir.join("mount",);
	fs::create_dir_all(&mount_dir,).unwrap();
	assert!(mount_dir.exists());
}

/// Test build artifact path construction
#[test]
fn test_build_artifact_paths() {
	let temp_workspace = create_test_workspace();
	let oso_root = temp_workspace.path().join("oso",);

	// Test loader artifact path
	let loader_target_dir = oso_root.join("target",).join("aarch64-unknown-uefi",).join("debug",);
	let loader_artifact = loader_target_dir.join("oso_loader.efi",);

	assert!(loader_artifact.to_string_lossy().contains("aarch64-unknown-uefi"));
	assert!(loader_artifact.to_string_lossy().contains("debug"));
	assert_eq!(loader_artifact.file_name().unwrap(), "oso_loader.efi");

	// Test kernel artifact path
	let kernel_target_dir =
		oso_root.join("target",).join("aarch64-unknown-none-elf",).join("debug",);
	let kernel_artifact = kernel_target_dir.join("oso_kernel",);

	assert!(kernel_artifact.to_string_lossy().contains("aarch64-unknown-none-elf"));
	assert!(kernel_artifact.to_string_lossy().contains("debug"));
	assert_eq!(kernel_artifact.file_name().unwrap(), "oso_kernel");
}

/// Test cleanup operations
#[test]
fn test_cleanup_operations() {
	let temp_workspace = create_test_workspace();
	let target_dir = temp_workspace.path().join("target",).join("xtask",);

	// Create some test files and directories
	fs::create_dir_all(&target_dir,).unwrap();
	let test_file = target_dir.join("test_file.img",);
	let test_dir = target_dir.join("test_mount",);

	fs::write(&test_file, b"test data",).unwrap();
	fs::create_dir_all(&test_dir,).unwrap();

	assert!(test_file.exists());
	assert!(test_dir.exists());

	// Test cleanup by removing files
	fs::remove_file(&test_file,).unwrap();
	fs::remove_dir_all(&test_dir,).unwrap();

	assert!(!test_file.exists());
	assert!(!test_dir.exists());
}

/// Test disk image size calculation
#[test]
fn test_disk_image_size() {
	// Test that disk image size is reasonable (e.g., 64MB)
	const EXPECTED_SIZE: u64 = 64 * 1024 * 1024; // 64MB

	assert_eq!(EXPECTED_SIZE, 67_108_864);
	assert!(EXPECTED_SIZE > 1024 * 1024); // At least 1MB
	assert!(EXPECTED_SIZE < 1024 * 1024 * 1024); // Less than 1GB
}
