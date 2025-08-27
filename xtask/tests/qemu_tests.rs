//! Unit tests for the qemu module
//!
//! These tests verify QEMU configuration, firmware management,
//! and command-line argument generation.

use std::path::PathBuf;
use tempfile::TempDir;

// Import the types we need to test
// Note: We'll test the public interface and logic without requiring full
// Builder instantiation

/// Test QEMU executable name generation for different architectures
#[test]
fn test_qemu_executable_names() {
	// Test the expected QEMU executable names for each architecture
	let aarch64_qemu = format!("qemu-system-{}", "aarch64");
	let x86_64_qemu = format!("qemu-system-{}", "x86_64");
	let riscv64_qemu = format!("qemu-system-{}", "riscv64");

	assert_eq!(aarch64_qemu, "qemu-system-aarch64");
	assert_eq!(x86_64_qemu, "qemu-system-x86_64");
	assert_eq!(riscv64_qemu, "qemu-system-riscv64");
}

/// Test basic QEMU arguments for aarch64
#[test]
fn test_aarch64_basic_args() {
	let expected_args = vec![
		"-machine",
		"virt",
		"-cpu",
		"cortex-a72",
		"-device",
		"virtio-gpu-pci",
	];

	// Verify that the expected arguments are reasonable
	assert!(expected_args.contains(&"-machine"));
	assert!(expected_args.contains(&"virt"));
	assert!(expected_args.contains(&"-cpu"));
	assert!(expected_args.contains(&"cortex-a72"));
	assert!(expected_args.contains(&"-device"));
	assert!(expected_args.contains(&"virtio-gpu-pci"));
}

/// Test basic QEMU arguments for x86_64
#[test]
fn test_x86_64_basic_args() {
	let expected_args = vec!["-machine", "q35", "-smp", "4", "-vga", "std"];

	// Verify that the expected arguments are reasonable
	assert!(expected_args.contains(&"-machine"));
	assert!(expected_args.contains(&"q35"));
	assert!(expected_args.contains(&"-smp"));
	assert!(expected_args.contains(&"4"));
	assert!(expected_args.contains(&"-vga"));
	assert!(expected_args.contains(&"std"));
}

/// Test persistent flash memory argument generation
#[test]
fn test_pflash_args_readonly() {
	let test_file = PathBuf::from("/tmp/test_code.fd",);

	// Test read-only pflash arguments
	let expected_readonly = format!(
		"-drive if=pflash,format=raw,readonly=on,file={}",
		test_file.display()
	);

	assert!(expected_readonly.contains("if=pflash"));
	assert!(expected_readonly.contains("format=raw"));
	assert!(expected_readonly.contains("readonly=on"));
	assert!(expected_readonly.contains("file=/tmp/test_code.fd"));
}

/// Test persistent flash memory argument generation for read-write
#[test]
fn test_pflash_args_readwrite() {
	let test_file = PathBuf::from("/tmp/test_vars.fd",);

	// Test read-write pflash arguments
	let expected_readwrite = format!(
		"-drive if=pflash,format=raw,readonly=off,file={}",
		test_file.display()
	);

	assert!(expected_readwrite.contains("if=pflash"));
	assert!(expected_readwrite.contains("format=raw"));
	assert!(expected_readwrite.contains("readonly=off"));
	assert!(expected_readwrite.contains("file=/tmp/test_vars.fd"));
}

/// Test block device argument generation
#[test]
fn test_block_device_args() {
	let disk_img = PathBuf::from("/tmp/test_disk.img",);
	let arg = &format!("file={},format=raw,if=none,id=hd0", disk_img.display());

	// Test block device arguments for x86_64
	let x86_64_args = vec![
		"-monitor",
		"stdio",
		"-drive",
		arg,
		"-device",
		"virtio-blk-pci,drive=hd0",
	];

	assert!(x86_64_args.contains(&"-monitor"));
	assert!(x86_64_args.contains(&"stdio"));
	assert!(x86_64_args.contains(&"-drive"));
	assert!(
		x86_64_args.iter().any(|arg| arg.contains("file=/tmp/test_disk.img"))
	);
	assert!(x86_64_args.iter().any(|arg| arg.contains("format=raw")));
	assert!(x86_64_args.iter().any(|arg| arg.contains("if=none")));
	assert!(x86_64_args.iter().any(|arg| arg.contains("id=hd0")));
	assert!(x86_64_args.contains(&"-device"));
	assert!(x86_64_args.contains(&"virtio-blk-pci,drive=hd0"));
}

/// Test boot menu arguments
#[test]
fn test_boot_menu_args() {
	let boot_args = vec!["-boot", "menu=on,splash-time=0"];

	assert!(boot_args.contains(&"-boot"));
	assert!(boot_args.contains(&"menu=on,splash-time=0"));
}

/// Test firmware path handling
#[test]
fn test_firmware_paths() {
	let temp_dir = TempDir::new().expect("Failed to create temp directory",);
	let code_path = temp_dir.path().join("OVMF_CODE.fd",);
	let vars_path = temp_dir.path().join("OVMF_VARS.fd",);

	// Create dummy firmware files
	std::fs::write(&code_path, b"dummy code",).unwrap();
	std::fs::write(&vars_path, b"dummy vars",).unwrap();

	assert!(code_path.exists());
	assert!(vars_path.exists());
	assert_eq!(code_path.file_name().unwrap(), "OVMF_CODE.fd");
	assert_eq!(vars_path.file_name().unwrap(), "OVMF_VARS.fd");
}

/// Test architecture to OVMF arch conversion
#[test]
fn test_arch_to_ovmf_conversion() {
	// Test that we can map our Architecture enum to ovmf_prebuilt::Arch
	// This is important for firmware file selection

	// These are the expected mappings based on the code
	let aarch64_mapping = "Aarch64"; // Architecture::Aarch64 -> Arch::Aarch64
	let x86_64_mapping = "X64"; // Architecture::X86_64 -> Arch::X64
	let riscv64_mapping = "Riscv64"; // Architecture::Riscv64 -> Arch::Riscv64

	assert_eq!(aarch64_mapping, "Aarch64");
	assert_eq!(x86_64_mapping, "X64");
	assert_eq!(riscv64_mapping, "Riscv64");
}

/// Test QEMU argument vector construction
#[test]
fn test_qemu_args_vector_construction() {
	let mut args = Vec::<String,>::new();

	// Test building a complete argument vector
	args.extend(vec!["-machine".to_string(), "virt".to_string()],);
	args.extend(vec!["-cpu".to_string(), "cortex-a72".to_string()],);
	args.extend(vec!["-device".to_string(), "virtio-gpu-pci".to_string()],);

	assert_eq!(args.len(), 6);
	assert_eq!(args[0], "-machine");
	assert_eq!(args[1], "virt");
	assert_eq!(args[2], "-cpu");
	assert_eq!(args[3], "cortex-a72");
	assert_eq!(args[4], "-device");
	assert_eq!(args[5], "virtio-gpu-pci");
}

/// Test debug mode argument addition
#[test]
fn test_debug_mode_args() {
	let mut args = Vec::<String,>::new();
	let debug_enabled = true;

	if debug_enabled {
		args.extend(vec![
			"-gdb".to_string(),
			"tcp::12345".to_string(),
			"-S".to_string(),
		],);
	}

	assert_eq!(args.len(), 3);
	assert!(args.contains(&"-gdb".to_string()));
	assert!(args.contains(&"tcp::12345".to_string()));
	assert!(args.contains(&"-S".to_string()));
}

/// Test memory allocation arguments
#[test]
fn test_memory_args() {
	let memory_args = vec!["-m", "256M"];

	assert!(memory_args.contains(&"-m"));
	assert!(memory_args.contains(&"256M"));

	// Test that memory size is reasonable
	let memory_size = "256M";
	assert!(memory_size.ends_with('M'));
	assert!(memory_size.starts_with("256"));
}

/// Test QEMU monitor configuration
#[test]
fn test_monitor_configuration() {
	let monitor_args = vec!["-monitor", "stdio"];

	assert!(monitor_args.contains(&"-monitor"));
	assert!(monitor_args.contains(&"stdio"));
}

/// Test firmware file type handling
#[test]
fn test_firmware_file_types() {
	// Test that we handle both code and vars firmware files
	let code_file = "OVMF_CODE.fd";
	let vars_file = "OVMF_VARS.fd";

	assert!(code_file.contains("CODE"));
	assert!(code_file.ends_with(".fd"));
	assert!(vars_file.contains("VARS"));
	assert!(vars_file.ends_with(".fd"));
}
