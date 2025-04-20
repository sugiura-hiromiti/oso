use crate::builder::Builder;
use crate::shell::Architecture;
use anyhow::Result as Rslt;
use ovmf_prebuilt::Arch;
use ovmf_prebuilt::FileType;
use ovmf_prebuilt::Prebuilt;
use ovmf_prebuilt::Source;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

impl Builder {
	pub fn qemu(&self,) -> String {
		format!("qemu-system-{}", self.arch().to_string())
	}

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

/// manage ovmf files
#[derive(Debug,)]
pub struct Firmware {
	code: PathBuf,
	vars: PathBuf,
}

impl Firmware {
	pub fn new(arch: &Architecture,) -> Rslt<Self,> {
		let path = PathBuf::from_str("/tmp/",)?;
		// NOTE: consider uncomment & implement if aarch64 boot failed
		//
		// let (code, vars,) = match arch {
		// 	Architecture::Aarch64 => {
		// 		let code = PathBuf::from_str(
		// 			"/opt/homebrew/Cellar/qemu/9.2.3/share/qemu/edk2-aarch64-code.fd",
		// 		)?;
		// 		let vars = PathBuf::from_str(
		// 			"/opt/homebrew/Cellar/qemu/9.2.3/share/qemu/edk2-arm-vars.fd",
		// 		)?;
		// 		(code, vars,)
		// 	},
		// 	Architecture::Riscv64 => todo!(),
		// 	Architecture::X86_64 => {
		// 		let ovmf_files = Prebuilt::fetch(Source::LATEST, path,)?;
		// 		let code = ovmf_files.get_file(arch.into(), FileType::Code,);
		// 		let vars = ovmf_files.get_file(arch.into(), FileType::Vars,);
		// 		(code, vars,)
		// 	},
		// };

		let ovmf_files = Prebuilt::fetch(Source::LATEST, path,)?;
		let code = ovmf_files.get_file(arch.into(), FileType::Code,);
		let vars = ovmf_files.get_file(arch.into(), FileType::Vars,);
		Ok(Self { code, vars, },)
	}

	pub fn code(&self,) -> &PathBuf {
		&self.code
	}

	pub fn vars(&self,) -> &PathBuf {
		&self.vars
	}
}

/// # Params
///
/// - path
/// root path where fetched files are places
///
/// # Return
///
/// returns path to (code.fd, vars.fd)
fn fetch_aa64_firmware(path: PathBuf,) -> (PathBuf, PathBuf,) {
	todo!()
}

impl From<&Architecture,> for Arch {
	fn from(value: &Architecture,) -> Self {
		match value {
			Architecture::Aarch64 => Arch::Aarch64,
			Architecture::Riscv64 => Arch::Riscv64,
			Architecture::X86_64 => Arch::X64,
		}
	}
}

enum PflashMode {
	ReadOnly,
	ReadWrite,
}

// fn devices() -> Vec<String,> {
// 	vec![
// 		"-nodefaults".to_string(),
// 		"-device".to_string(),
// 		"virtio-rng-pci".to_string(),
// 		"-device".to_string(),
// 		"virtio-scsi-pci".to_string(),
// 	]
// }

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

/// configure persistent flash memory aka. pflash
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

fn block_device(disk_img: &Path, arch: &Architecture,) -> Vec<String,> {
	vec![
		"-monitor".to_string(),
		"stdio".to_string(),
		"-drive".to_string(),
		format!("file={},format=raw,if=none,id=hd0", disk_img.display()),
		"-device".to_string(),
		match arch {
			Architecture::X86_64 => "virtio-blk-pci,drive=hd0",
			_ => "virtio-blk-device,drive=hd0",
		}
		.to_string(),
	]
}
