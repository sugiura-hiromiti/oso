use std::path::PathBuf;
use std::str::FromStr;

use crate::OsoWorkSpace;
use crate::shell::Opts;
use crate::workspace::Architecture;
use anyhow::Result as Rslt;
use ovmf_prebuilt::Arch;
use ovmf_prebuilt::FileType;
use ovmf_prebuilt::Prebuilt;
use ovmf_prebuilt::Source;
use tempfile::TempDir;

pub fn qemu(arch: &Architecture,) -> String {
	format!("qemu-system-{}", arch.to_string())
}

pub fn qemu_args(opts: &Opts,) -> Vec<String,> {
	let mut args = basic_args(&self.arch,);

	// configure persistent flash memory
	let pflash_code = persistent_flash_memory_args(&ovmf_files.code, PflashMode::ReadOnly,);
	let pflash_var = persistent_flash_memory_args(&ovmf_files.vars, PflashMode::ReadWrite,);
	args.extend(pflash_code,);
	args.extend(pflash_var,);

	args.extend(devices(),);

	// setting the boot menu timeout to zero particularly speeds up the boot
	args.push("-boot".to_string(),);
	args.push("menu=on,splash-time=0".to_string(),);
	args
}

/// manage ovmf files
pub struct Ovmf {
	code:  PathBuf,
	vars:  PathBuf,
	shell: PathBuf,
}

impl Ovmf {
	fn new(arch: &Architecture,) -> Rslt<Self,> {
		let path = PathBuf::from_str("/tmp/",)?;
		let ovmf_files = Prebuilt::fetch(Source::LATEST, path,)?;
		let code = ovmf_files.get_file(arch.into(), FileType::Code,);
		let vars = ovmf_files.get_file(arch.into(), FileType::Vars,);
		let shell = ovmf_files.get_file(arch.into(), FileType::Shell,);

		let tmp_dir = TempDir::new()?;
		let tmp_dir = tmp_dir.path();
		let vars_tmp = tmp_dir.join("ovmf_vars",);
		fs_err::copy(vars, &vars_tmp,);

		Ok(Self { code, vars: vars_tmp, shell, },)
	}
}

impl From<&Architecture,> for Arch {
	fn from(value: &Architecture,) -> Self {
		match value {
			Architecture::X86_64 => Arch::X64,
			Architecture::Aarch64 => Arch::Aarch64,
		}
	}
}

enum PflashMode {
	ReadOnly,
	ReadWrite,
}

fn devices() -> Vec<String,> {
	vec![
		"-nodefaults".to_string(),
		"-device".to_string(),
		"virtio-rng-pci".to_string(),
		"-device".to_string(),
		"virtio-scsi-pci".to_string(),
	]
}

fn basic_args(arch: &Architecture,) -> Vec<String,> {
	match arch {
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
		],
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
