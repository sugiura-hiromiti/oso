//! # QEMU Module
//!
//! This module handles QEMU configuration and firmware management.
//!
//! It provides functionality for:
//! - Configuring QEMU command-line arguments based on the target architecture
//! - Managing OVMF firmware files for UEFI boot
//! - Setting up block devices and persistent flash memory

use anyhow::Result as Rslt;
use ovmf_prebuilt::Arch;
use ovmf_prebuilt::FileType;
use ovmf_prebuilt::Prebuilt;
use ovmf_prebuilt::Source;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use xtask::builder::Builder;
use xtask::shell::Architecture;

impl Builder {
	/// Gets the QEMU executable name based on the target architecture
	///
	/// # Returns
	///
	/// The name of the QEMU executable (e.g., "qemu-system-aarch64")
	pub fn qemu(&self,) -> String {
		format!("qemu-system-{}", self.arch().to_string())
	}

	/// Generates QEMU command-line arguments based on the target architecture and configuration
	///
	/// # Returns
	///
	/// A vector of command-line arguments for QEMU or an error if it fails
	pub fn qemu_args(&self,) -> Rslt<Vec<String,>,> {
		let mut args = basic_args(self.arch(),);

		// configure persistent flash memory
		let pflash_code =
			persistent_flash_memory_args(&self.firmware_code()?, PflashMode::ReadOnly,);
		let pflash_var =
			persistent_flash_memory_args(&self.firmware_vars()?, PflashMode::ReadWrite,);
		args.extend(pflash_code,);
		args.extend(pflash_var,);

		// args.extend(devices(),);

		// let esp_dir = self.build_mount_point()?;
		// args.push("-drive".to_string(),);
		// args.push("format=raw,file=fat:rw:",);

		let block_device = block_device(&self.disk_img_path()?, self.arch(),);
		args.extend(block_device,);

		// setting the boot menu timeout to zero particularly speeds up the boot
		args.push("-boot".to_string(),);
		args.push("menu=on,splash-time=0".to_string(),);

		Ok(args,)
	}
}

/// Manages OVMF firmware files for UEFI boot
#[derive(Debug,)]
pub struct Firmware {
	/// Path to the OVMF code file
	code: PathBuf,
	/// Path to the OVMF variables file
	vars: PathBuf,
}

impl Firmware {
	/// Creates a new Firmware instance for the specified architecture
	///
	/// Downloads the latest OVMF firmware files if they don't exist.
	///
	/// # Parameters
	///
	/// * `arch` - The target architecture
	///
	/// # Returns
	///
	/// A new Firmware instance or an error if initialization fails
	pub fn new(arch: &Architecture,) -> Rslt<Self,> {
		let path = PathBuf::from_str("/tmp/",)?;
		let ovmf_files = Prebuilt::fetch(Source::LATEST, path,)?;
		let code = ovmf_files.get_file(arch.into(), FileType::Code,);
		let vars = ovmf_files.get_file(arch.into(), FileType::Vars,);
		Ok(Self { code, vars, },)
	}

	/// Gets the path to the OVMF code file
	///
	/// # Returns
	///
	/// A reference to the path to the OVMF code file
	pub fn code(&self,) -> &PathBuf {
		&self.code
	}

	/// Gets the path to the OVMF variables file
	///
	/// # Returns
	///
	/// A reference to the path to the OVMF variables file
	pub fn vars(&self,) -> &PathBuf {
		&self.vars
	}
}

/// Converts an Architecture enum to an ovmf_prebuilt::Arch enum
impl From<&Architecture,> for Arch {
	fn from(value: &Architecture,) -> Self {
		match value {
			Architecture::Aarch64 => Arch::Aarch64,
			Architecture::Riscv64 => Arch::Riscv64,
			Architecture::X86_64 => Arch::X64,
		}
	}
}

/// Defines the mode for persistent flash memory
enum PflashMode {
	/// Read-only mode
	ReadOnly,
	/// Read-write mode
	ReadWrite,
}

/// Generates basic QEMU arguments based on the target architecture
///
/// # Parameters
///
/// * `arch` - The target architecture
///
/// # Returns
///
/// A vector of basic QEMU command-line arguments
fn basic_args(arch: &Architecture,) -> Vec<String,> {
	match arch {
		Architecture::Aarch64 => vec![
			// generic arm enviromnent
			"-machine".to_string(),
			"virt".to_string(),
			// a72 is a very generic 64-bit arm cpu
			"-cpu".to_string(),
			"cortex-a72".to_string(),
			// graphics device
			"-device".to_string(),
			"virtio-gpu-pci".to_string(),
			// // keep using ramfb until implementing Linux-style driver
			// "ramfb".to_string(),
		],
		Architecture::Riscv64 => todo!(),
		Architecture::X86_64 => {
			vec![
				"-machine".to_string(),
				"q35".to_string(),
				"-smp".to_string(),
				"4".to_string(),
				// allocate some memory
				// "-m".to_string(),
				// "256M".to_string(),

				// graphics device
				"-vga".to_string(),
				"std".to_string(),
			]
		},
	}
}

/// Generates QEMU arguments for persistent flash memory
///
/// # Parameters
///
/// * `pflash_file` - The path to the flash file
/// * `mode` - The mode for the flash file (read-only or read-write)
///
/// # Returns
///
/// A vector of QEMU command-line arguments for persistent flash memory
fn persistent_flash_memory_args(pflash_file: &PathBuf, mode: PflashMode,) -> Vec<String,> {
	let mut args = vec!["-drive".to_string()];
	let mut arg = String::from("if=pflash,format=raw,readonly=",);
	arg.push_str(match mode {
		PflashMode::ReadOnly => "on",
		PflashMode::ReadWrite => "off",
	},);

	arg.push_str(",file=",);
	arg.push_str(pflash_file.to_str().unwrap(),);
	args.push(arg,);

	args
}

/// Generates QEMU arguments for block devices
///
/// # Parameters
///
/// * `disk_img` - The path to the disk image
/// * `arch` - The target architecture
///
/// # Returns
///
/// A vector of QEMU command-line arguments for block devices
fn block_device(disk_img: &Path, arch: &Architecture,) -> Vec<String,> {
	vec![
		"-monitor".to_string(),
		"stdio".to_string(),
		"-drive".to_string(),
		format!("file={},format=raw,if=none,id=hd0", disk_img.display()),
		"-device".to_string(),
		match arch {
			Architecture::X86_64 => "virtio-blk-pci,drive=hd0",
			// _ => "virtio-blk-device,drive=hd0",
			_ => "virtio-blk-pci,drive=hd0",
		}
		.to_string(),
	]
}
