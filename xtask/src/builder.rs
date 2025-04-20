use crate::qemu::Firmware;
use crate::shell::Architecture;
use crate::shell::Opts;
use crate::shell::Run;
use crate::workspace;
use crate::workspace::LOADER;
use crate::workspace::OsoWorkSpace;
use anyhow::Result as Rslt;
use colored::Colorize;
use std::env::set_current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

const BOOT_DIR: &str = "efi/boot";
const KERNEL_FILE: &str = "oso_kernel.elf";

#[derive(Debug,)]
pub struct Builder {
	opts:      Opts,
	workspace: OsoWorkSpace,
	firmware:      Firmware,
}

impl Builder {
	pub fn new() -> Rslt<Self,> {
		let opts = Opts::new();
		let workspace = OsoWorkSpace::new()?;
		let ovmf = Firmware::new(&opts.arch,)?;
		Ok(Self { opts, workspace, firmware: ovmf, },)
	}

	pub fn build(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.root,)?;
		self.build_loader()?;
		self.build_kernel()
	}

	fn build_loader(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.loader.root,)?;
		cargo_build(&self.opts,)?.arg("--target",).arg(self.opts.arch.loader_tuple(),).run()?;
		set_current_dir(&self.workspace.root,)?;
		Ok((),)
	}

	fn build_kernel(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.kernel.root,)?;
		cargo_build(&self.opts,)?
			.arg("--target",)
			.arg(format!("{}.json", self.opts.arch.kernel_tuple()),)
			.run()?;
		set_current_dir(&self.workspace.root,)?;
		Ok((),)
	}

	pub fn arch(&self,) -> &Architecture {
		&self.opts.arch
	}

	pub fn firmware_code(&self,) -> Rslt<PathBuf,> {
		let tmp_path = self.firmware_tmp_code()?;
		if !tmp_path.exists() {
			let original = self.firmware.code();
			fs_err::copy(original, &tmp_path,)?;
		}
		Ok(tmp_path,)
	}

	fn firmware_tmp_code(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mv_firmware_code",),)
	}

	pub fn firmware_vars(&self,) -> Rslt<PathBuf,> {
		let tmp_file = self.firmware_tmp_vars()?;
		if !tmp_file.exists() {
			let orignal = self.firmware.vars();
			fs_err::copy(orignal, &tmp_file,)?;
		}

		Ok(tmp_file,)
	}

	fn firmware_tmp_vars(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mv_firmware_vars",),)
	}

	pub fn run(self,) -> Rslt<(),> {
		let mounted_disk = self.mount_img()?;
		self.build_boot_dir()?;

		detatch(&mounted_disk,)?;

		// run qemu
		let qemu_system = self.qemu();
		let qemu_args = self.qemu_args()?;

		if !self.firmware_code()?.exists() {
			panic!("ovmf_code: {}, path does not exist", self.firmware_tmp_code()?.display());
		}
		if !self.firmware_vars()?.exists() {
			panic!("ovmf_vars: {}, path does not exist", self.firmware_tmp_vars()?.display());
		}

		Command::new("eza",).args(["/Users/a/Downloads/QwQ/oso/target/xtask", "-T",],).run()?;

		Command::new(qemu_system,).args(qemu_args,).run()?;
		Ok((),)
	}

	/// # Return
	///
	/// returns name of mounted_disk
	fn mount_img(&self,) -> Rslt<String,> {
		self.set_disk_img()?;

		// set mount point
		self.create_mount_point()?;
		let out = Command::new("hdiutil",)
			.args(["attach", "-imagekey", "diskimage-class=CRawDiskImage", "-nomount",],)
			.arg(self.disk_img_path()?,)
			.output()?;

		println!("{}", "mounting img:".bold().bright_green());

		// get name of mounted disk
		let stdout = unsafe { String::from_utf8_unchecked(out.stdout,) };
		let stderr = unsafe { String::from_utf8_unchecked(out.stderr,) };
		print!("\tstdout: {}", stdout);
		println!("\tstderr: {}", stderr);

		if let Err(e,) = out.status.exit_ok() {
			return Err(e.into(),);
		}
		let mounted_disk = stdout.trim();

		// mount disk image
		Command::new("mount",)
			.args(["-t", "msdos", mounted_disk,],)
			.arg(self.mount_point_path()?,)
			.run()?;
		Ok(mounted_disk.to_string(),)
	}

	fn set_disk_img(&self,) -> Rslt<(),> {
		let disk_img = self.disk_img_path()?;
		if disk_img.exists() {
			fs_err::remove_file(&disk_img,)?;
		}

		// create
		Command::new("qemu-img",)
			.args(["create", "-f", "raw",],)
			.arg(&disk_img,)
			.arg("200m",)
			.run()?;

		// format
		Command::new("mkfs.fat",)
			.args(["-n", "'OSO'", "-s", "2", "-f", "2", "-R", "32", "-F", "32",],)
			.arg(disk_img,)
			.run()
	}

	pub fn disk_img_path(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("disk.img",),)
	}

	fn create_mount_point(&self,) -> Rslt<(),> {
		let mount_point = self.mount_point_path()?;
		if mount_point.exists() {
			fs_err::remove_dir_all(&mount_point,)?;
		}
		fs_err::create_dir_all(&mount_point,)?;
		Ok((),)
	}

	fn mount_point_path(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mnt",),)
	}

	fn build_dir(&self,) -> Rslt<PathBuf,> {
		let build_dir = self.workspace.root.join("target",).join("xtask",);
		if !build_dir.exists() {
			fs_err::create_dir_all(&build_dir,)?;
		}
		Ok(build_dir,)
	}

	fn build_boot_dir(&self,) -> Rslt<(),> {
		let boot_dir = self.create_boot_dir()?;
		self.put_boot_loader(&boot_dir,)?;
		self.put_kernel()?;
		Ok((),)
	}

	fn create_boot_dir(&self,) -> Rslt<PathBuf,> {
		let boot_dir = self.mount_point_path()?.join(BOOT_DIR,);
		fs_err::create_dir_all(&boot_dir,)?;

		Ok(boot_dir,)
	}

	fn put_boot_loader(&self, boot_dir: &Path,) -> Rslt<(),> {
		fs_err::copy(self.loader_build_artifact(), boot_dir.join(self.arch().boot_file_name(),),)?;
		Ok((),)
	}

	fn loader_build_artifact(&self,) -> PathBuf {
		let build_artifact = self
			.workspace
			.root
			.join("target",)
			.join(self.arch().loader_tuple(),)
			.join(self.opts.build_mode.to_string(),)
			.join(format!("{LOADER}.efi"),);
		println!(
			"{}",
			format!("loader build artifact is {}", build_artifact.display()).bold().yellow()
		);
		build_artifact
	}

	fn put_kernel(&self,) -> Rslt<(),> {
		fs_err::copy(self.kernel_build_artifact()?, self.mount_point_path()?.join(KERNEL_FILE,),)?;
		Ok((),)
	}

	fn kernel_build_artifact(&self,) -> Rslt<PathBuf,> {
		let target_json =
			self.workspace.kernel.root.join(format!("{}.json", self.arch().kernel_tuple()),);
		let target_json = workspace::load_json(&target_json,)?;
		workspace::detect_build_artifact(target_json,)
	}
}

impl Drop for Builder {
	fn drop(&mut self,) {
		match self.build_dir() {
			Ok(p,) => {
				if let Err(e,) = fs_err::remove_dir_all(&p,) {
					eprintln!(
						"{}",
						format!(
							"failed to remove build_dir: {}\n{e}\n\nremove it manually",
							p.display()
						)
						.bold()
						.red()
					)
				}
			},
			Err(e,) => eprintln!(
				"{}",
				format!(
					"something went unsuccessfully. failed to get build_dir\n{e}\nremove it \
					 manually"
				)
				.bold()
				.yellow()
			),
		}
	}
}

fn cargo_build(opts: &Opts,) -> Rslt<Command,> {
	let mut cmd = Command::new("cargo",);
	cmd.arg("b",);
	if opts.features.len() != 0 {
		cmd.arg("--features",);
		cmd.args(&opts.features,);
	}
	if opts.build_mode.is_release() {
		cmd.arg("-r",);
	}

	Ok(cmd,)
}

pub fn detatch(mounted_disk: &String,) -> Rslt<(),> {
	Command::new("hdiutil",).args(["detach", mounted_disk,],).run()
}
