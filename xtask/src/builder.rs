//! # Builder Module
//!
//! Core functionality for building the OSO loader and kernel, creating disk images,
//! and running QEMU.
//!
//! This module handles:
//! - Building the OSO loader and kernel for the target architecture
//! - Creating and formatting a disk image
//! - Mounting the disk image and copying the built artifacts
//! - Configuring and running QEMU with the appropriate firmware and disk image
//! - Cleanup of temporary files and unmounting disk images

use crate::qemu::Firmware;
use crate::shell::Architecture;
use crate::shell::Opts;
use crate::workspace;
use crate::workspace::LOADER;
use crate::workspace::OsoWorkSpace;
use anyhow::Result as Rslt;
use anyhow::anyhow;
use colored::Colorize;
use std::env::set_current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use util_common_code::Run;

/// Directory path for EFI boot files
const BOOT_DIR: &str = "efi/boot";
/// Filename for the OSO kernel
const KERNEL_FILE: &str = "oso_kernel.elf";

/// Main builder struct that orchestrates the build and run process
#[derive(Debug,)]
pub struct Builder {
	/// Command-line options
	opts:      Opts,
	/// OSO workspace information
	workspace: OsoWorkSpace,
	/// OVMF firmware information
	firmware:  Firmware,
	/// current host os
	host_os:   HostOs,
}

impl Builder {
	/// Creates a new Builder instance with the specified options
	///
	/// Initializes the workspace and firmware based on the command-line options.
	///
	/// # Returns
	///
	/// A new Builder instance or an error if initialization fails
	pub fn new() -> Rslt<Self,> {
		let opts = Opts::new();
		let workspace = OsoWorkSpace::new()?;
		let ovmf = Firmware::new(&opts.arch,)?;
		let host_os = HostOs::new()?;
		Ok(Self { opts, workspace, firmware: ovmf, host_os, },)
	}

	/// Builds the OSO loader and kernel
	///
	/// # Returns
	///
	/// Ok(()) if the build succeeds, or an error if it fails
	pub fn build(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.root,)?;

		// order should not be loader -> kernel.
		// because loader is depending kernel by proc macro
		self.build_kernel()?;
		self.build_loader()
	}

	/// Builds the OSO loader (UEFI application)
	///
	/// # Returns
	///
	/// Ok(()) if the build succeeds, or an error if it fails
	fn build_loader(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.loader.root,)?;
		cargo_build(&self.opts,)?.arg("--target",).arg(self.opts.arch.loader_tuple(),).run()?;
		set_current_dir(&self.workspace.root,)?;
		Ok((),)
	}

	/// Builds the OSO kernel
	///
	/// # Returns
	///
	/// Ok(()) if the build succeeds, or an error if it fails
	fn build_kernel(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.kernel.root,)?;
		cargo_build(&self.opts,)?
			.arg("--target",)
			.arg(format!("{}.json", self.opts.arch.kernel_tuple()),)
			.run()?;
		set_current_dir(&self.workspace.root,)?;
		Ok((),)
	}

	/// Gets the target architecture
	///
	/// # Returns
	///
	/// A reference to the Architecture enum
	pub fn arch(&self,) -> &Architecture {
		&self.opts.arch
	}

	/// Gets the command-line options
	///
	/// # Returns
	///
	/// A reference to the Opts struct
	pub fn opts(&self,) -> &Opts {
		&self.opts
	}

	/// Gets the path to the firmware code file
	///
	/// Copies the firmware code file to a temporary location if it doesn't exist.
	///
	/// # Returns
	///
	/// The path to the firmware code file or an error if it fails
	pub fn firmware_code(&self,) -> Rslt<PathBuf,> {
		let tmp_path = self.firmware_tmp_code()?;
		if !tmp_path.exists() {
			let original = self.firmware.code();
			fs_err::copy(original, &tmp_path,)?;
		}
		Ok(tmp_path,)
	}

	/// Gets the path to the temporary firmware code file
	///
	/// # Returns
	///
	/// The path to the temporary firmware code file or an error if it fails
	fn firmware_tmp_code(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mv_firmware_code",),)
	}

	/// Gets the path to the firmware variables file
	///
	/// Copies the firmware variables file to a temporary location if it doesn't exist.
	///
	/// # Returns
	///
	/// The path to the firmware variables file or an error if it fails
	pub fn firmware_vars(&self,) -> Rslt<PathBuf,> {
		let tmp_file = self.firmware_tmp_vars()?;
		if !tmp_file.exists() {
			let orignal = self.firmware.vars();
			fs_err::copy(orignal, &tmp_file,)?;
		}

		Ok(tmp_file,)
	}

	/// Gets the path to the temporary firmware variables file
	///
	/// # Returns
	///
	/// The path to the temporary firmware variables file or an error if it fails
	fn firmware_tmp_vars(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mv_firmware_vars",),)
	}

	/// Runs QEMU with the built OSO loader and kernel
	///
	/// Mounts the disk image, copies the built artifacts, and runs QEMU.
	///
	/// # Returns
	///
	/// Ok(()) if QEMU runs successfully, or an error if it fails
	pub fn run(self,) -> Rslt<(),> {
		let mounted_disk = self.mount_img()?;
		self.build_boot_dir()?;

		self.detatch(&mounted_disk,)?;

		// run qemu
		let qemu_system = self.qemu();
		let qemu_args = self.qemu_args()?;

		if self.opts().debug {
			let mut qemu_args = qemu_args;
			let dbg_args = ["-gdb", "tcp::12345", "-S",];
			dbg_args.iter().for_each(|s| {
				qemu_args.push(s.to_string(),);
			},);

			Command::new(qemu_system,).args(qemu_args,).run()?;
			/*
						//add args for debugging
			if self.opts().debug {
				let dbg_args = [
					"-gdb",
					"tcp::12345",
					"-S",
					"'",
					"&",
					"&&",
					"lldb",
					"-s",
					"./lldb_startup_command.txt",
				];
				dbg_args.into_iter().for_each(|arg| {
					args.push_back(arg.to_string(),);
				},);

				let launch_qemu_in_child_process = ["zsh", "-c", "'",];
				launch_qemu_in_child_process.iter().rev().for_each(|s| {
					args.push_front(s.to_string(),);
				},);
			}

				*/
		} else {
			Command::new(qemu_system,).args(qemu_args,).run()?;
		}
		Ok((),)
	}

	/// Mounts the disk image
	///
	/// Creates a disk image, formats it, and mounts it.
	///
	/// # Returns
	///
	/// The name of the mounted disk or an error if it fails
	fn mount_img(&self,) -> Rslt<String,> {
		self.set_disk_img()?;

		// set mount point
		self.create_mount_point()?;

		let mounted_disk = match self.host_os {
			HostOs::Mac => {
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
				mounted_disk.to_string()
			},
			HostOs::Linux => {
				Command::new("mount",)
					.args(["-o", "loop0",],)
					.args([self.disk_img_path()?, self.mount_point_path()?,],)
					.run()?;
				"".to_string()
			},
		};
		Ok(mounted_disk,)
	}

	/// Creates and formats a disk image
	///
	/// # Returns
	///
	/// Ok(()) if the disk image is created and formatted successfully, or an error if it fails
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

	/// Gets the path to the disk image
	///
	/// # Returns
	///
	/// The path to the disk image or an error if it fails
	pub fn disk_img_path(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("disk.img",),)
	}

	/// Creates a mount point for the disk image
	///
	/// # Returns
	///
	/// Ok(()) if the mount point is created successfully, or an error if it fails
	fn create_mount_point(&self,) -> Rslt<(),> {
		let mount_point = self.mount_point_path()?;
		if mount_point.exists() {
			fs_err::remove_dir_all(&mount_point,)?;
		}
		fs_err::create_dir_all(&mount_point,)?;
		Ok((),)
	}

	/// Gets the path to the mount point
	///
	/// # Returns
	///
	/// The path to the mount point or an error if it fails
	fn mount_point_path(&self,) -> Rslt<PathBuf,> {
		Ok(self.build_dir()?.join("mnt",),)
	}

	/// Gets the path to the build directory
	///
	/// Creates the build directory if it doesn't exist.
	///
	/// # Returns
	///
	/// The path to the build directory or an error if it fails
	fn build_dir(&self,) -> Rslt<PathBuf,> {
		let build_dir = self.workspace.root.join("target",).join("xtask",);
		if !build_dir.exists() {
			fs_err::create_dir_all(&build_dir,)?;
		}
		Ok(build_dir,)
	}

	/// Creates the boot directory structure and copies the built artifacts
	///
	/// # Returns
	///
	/// Ok(()) if the boot directory is created and populated successfully, or an error if it fails
	fn build_boot_dir(&self,) -> Rslt<(),> {
		let boot_dir = self.create_boot_dir()?;
		self.put_boot_loader(&boot_dir,)?;
		self.put_kernel()?;
		Ok((),)
	}

	/// Creates the EFI boot directory structure
	///
	/// # Returns
	///
	/// The path to the boot directory or an error if it fails
	fn create_boot_dir(&self,) -> Rslt<PathBuf,> {
		let boot_dir = self.mount_point_path()?.join(BOOT_DIR,);
		fs_err::create_dir_all(&boot_dir,)?;

		Ok(boot_dir,)
	}

	/// Copies the boot loader to the boot directory
	///
	/// # Parameters
	///
	/// * `boot_dir` - The path to the boot directory
	///
	/// # Returns
	///
	/// Ok(()) if the boot loader is copied successfully, or an error if it fails
	fn put_boot_loader(&self, boot_dir: &Path,) -> Rslt<(),> {
		fs_err::copy(self.loader_build_artifact(), boot_dir.join(self.arch().boot_file_name(),),)?;
		Ok((),)
	}

	/// Gets the path to the loader build artifact
	///
	/// # Returns
	///
	/// The path to the loader build artifact
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

	/// Copies the kernel to the disk image
	///
	/// # Returns
	///
	/// Ok(()) if the kernel is copied successfully, or an error if it fails
	fn put_kernel(&self,) -> Rslt<(),> {
		fs_err::copy(self.kernel_build_artifact()?, self.mount_point_path()?.join(KERNEL_FILE,),)?;
		Ok((),)
	}

	/// Gets the path to the kernel build artifact
	///
	/// # Returns
	///
	/// The path to the kernel build artifact or an error if it fails
	fn kernel_build_artifact(&self,) -> Rslt<PathBuf,> {
		let target_json =
			self.workspace.kernel.root.join(format!("{}.json", self.arch().kernel_tuple()),);
		let target_json = workspace::load_json(&target_json,)?;
		workspace::detect_build_artifact(target_json,)
	}

	/// Detaches a mounted disk image
	///
	/// # Parameters
	///
	/// * `mounted_disk` - The name of the mounted disk
	///
	/// # Returns
	///
	/// Ok(()) if the disk is detached successfully, or an error if it fails
	pub fn detatch(&self, mounted_disk: &String,) -> Rslt<(),> {
		Command::new("eza",)
			.args(
				"-ahlF --icons --group-directories-first --sort=extension --time-style=iso --git \
				 --no-user --no-time -T target/xtask"
					.split_whitespace(),
			)
			.run()?;

		match self.host_os {
			HostOs::Mac => Command::new("hdiutil",).args(["detach", mounted_disk,],).run(),
			HostOs::Linux => {
				Command::new("sudo",).arg("umount",).arg(self.mount_point_path()?,).run()
			},
		}
	}
}

/// Automatically cleans up temporary files and unmounts disk images when the Builder is dropped
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

#[derive(Debug,)]
pub enum HostOs {
	Mac,
	Linux,
}

impl HostOs {
	pub fn new() -> Rslt<Self,> {
		let a = Command::new("uname",).arg("-s",).output()?;
		let os_name = String::from_utf8(a.stdout,)?;
		let os_name = os_name.trim();
		match os_name {
			"Linux" => Ok(Self::Linux,),
			"Darwin" => Ok(Self::Mac,),
			_ => Err(anyhow!("building on {os_name} does not supported"),),
		}
	}
}

/// Creates a cargo build command with the specified options
///
/// # Parameters
///
/// * `opts` - The command-line options
///
/// # Returns
///
/// A Command object configured for building with cargo or an error if it fails
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
