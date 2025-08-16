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
use oso_dev_util_helper::cli::Run;
use std::env::set_current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Directory path for EFI boot files
const BOOT_DIR: &str = "efi/boot";
/// Filename for the OSO kernel
const KERNEL_FILE: &str = "oso_kernel.elf";

/// Main builder struct that orchestrates the build and run process
///
/// The `Builder` struct is the central component of the OSO build system. It manages
/// the entire workflow from building the kernel and loader to creating disk images
/// and running QEMU. It encapsulates all the necessary configuration and state
/// needed for the build process.
///
/// # Architecture Support
///
/// The builder supports multiple target architectures:
/// - **AArch64**: ARM 64-bit architecture (primary target)
/// - **x86_64**: Intel/AMD 64-bit architecture (partial support)
/// - **RISC-V 64**: RISC-V 64-bit architecture (planned)
///
/// # Build Process
///
/// The typical build process involves:
/// 1. **Initialization**: Set up workspace, firmware, and host OS detection
/// 2. **Building**: Compile the kernel and loader for the target architecture
/// 3. **Disk Image Creation**: Create and format a FAT32 disk image
/// 4. **File Deployment**: Mount the disk image and copy built artifacts
/// 5. **QEMU Execution**: Launch QEMU with appropriate firmware and configuration
/// 6. **Cleanup**: Unmount disk images and clean up temporary files
///
/// # Examples
///
/// ```rust,ignore
/// use xtask::builder::Builder;
///
/// // Create a new builder with default options
/// let builder = Builder::new()?;
///
/// // Build the kernel and loader
/// builder.build()?;
///
/// // Run in QEMU
/// builder.run()?;
/// ```
#[derive(Debug,)]
pub struct Builder {
	/// Command-line options and build configuration
	opts:      Opts,
	/// OSO workspace information and paths
	workspace: OsoWorkSpace,
	/// OVMF firmware configuration for UEFI boot
	firmware:  Firmware,
	/// Host operating system detection for platform-specific operations
	host_os:   HostOs,
}

impl Builder {
	/// Creates a new Builder instance with the specified options
	///
	/// This constructor initializes all the necessary components for the build process:
	/// - Parses command-line options and build configuration
	/// - Sets up the OSO workspace with project paths
	/// - Downloads and configures OVMF firmware for the target architecture
	/// - Detects the host operating system for platform-specific operations
	///
	/// # Initialization Process
	///
	/// 1. **Options Parsing**: Reads command-line arguments for architecture, build mode, etc.
	/// 2. **Workspace Setup**: Locates project root and validates workspace structure
	/// 3. **Firmware Download**: Fetches appropriate OVMF firmware files for UEFI boot
	/// 4. **Host Detection**: Identifies the host OS (macOS, Linux) for mount operations
	///
	/// # Returns
	///
	/// * `Ok(Builder)` - A fully initialized Builder instance ready for use
	/// * `Err(anyhow::Error)` - If initialization fails due to:
	///   - Invalid workspace structure
	///   - Firmware download failure
	///   - Unsupported host operating system
	///   - Network connectivity issues
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use xtask::builder::Builder;
	///
	/// // Create a builder with default configuration
	/// let builder = Builder::new()?;
	/// println!("Building for architecture: {:?}", builder.arch());
	/// ```
	///
	/// # Errors
	///
	/// This method can fail in several scenarios:
	/// - **Workspace Error**: If the OSO project structure is invalid or incomplete
	/// - **Firmware Error**: If OVMF firmware files cannot be downloaded or accessed
	/// - **Host OS Error**: If the host operating system is not supported (Windows)
	/// - **Network Error**: If firmware download requires internet access and fails
	pub fn new() -> Rslt<Self,> {
		let opts = Opts::new();
		let workspace = OsoWorkSpace::new()?;
		let ovmf = Firmware::new(&opts.arch,)?;
		let host_os = HostOs::new()?;
		Ok(Self { opts, workspace, firmware: ovmf, host_os, },)
	}

	/// Builds the OSO loader and kernel for the target architecture
	///
	/// This method orchestrates the complete build process for both the UEFI loader
	/// and the kernel. It ensures proper build order and handles architecture-specific
	/// compilation requirements.
	///
	/// # Build Order
	///
	/// The build order is critical due to dependencies:
	/// 1. **Kernel First**: The kernel must be built before the loader because the loader uses
	///    procedural macros that depend on kernel artifacts
	/// 2. **Loader Second**: The loader is built as a UEFI application that will load and execute
	///    the kernel
	///
	/// # Architecture-Specific Builds
	///
	/// - **AArch64**: Uses custom target specification for bare-metal ARM64
	/// - **x86_64**: Uses custom target specification for bare-metal x86_64
	/// - **RISC-V**: Planned support with custom target specification
	///
	/// # Build Artifacts
	///
	/// After successful completion, the following artifacts are created:
	/// - `target/{arch}/debug|release/oso_kernel.elf` - The kernel binary
	/// - `target/{loader_triple}/debug|release/oso_loader.efi` - The UEFI loader
	///
	/// # Returns
	///
	/// * `Ok(())` - If both kernel and loader build successfully
	/// * `Err(anyhow::Error)` - If any build step fails due to:
	///   - Compilation errors in kernel or loader code
	///   - Missing dependencies or tools
	///   - Invalid target specifications
	///   - Workspace or file system issues
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let builder = Builder::new()?;
	///
	/// // Build both kernel and loader
	/// builder.build()?;
	/// println!("Build completed successfully!");
	/// ```
	///
	/// # Dependencies
	///
	/// This method requires:
	/// - Rust nightly toolchain with the target architecture support
	/// - Custom target specification files in the workspace
	/// - All source dependencies and procedural macros
	pub fn build(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.root,)?;

		// Build order is critical: kernel -> loader
		// The loader depends on kernel artifacts through procedural macros
		self.build_kernel()?;
		self.build_loader()
	}

	/// Builds the OSO loader (UEFI application)
	///
	/// This method compiles the OSO loader as a UEFI application that can be executed
	/// by UEFI firmware. The loader is responsible for initializing the system,
	/// loading the kernel from disk, and transferring control to the kernel.
	///
	/// # Build Process
	///
	/// 1. Changes to the loader workspace directory
	/// 2. Executes `cargo build` with the appropriate loader target triple
	/// 3. Returns to the workspace root directory
	///
	/// # Target Specifications
	///
	/// The loader uses architecture-specific UEFI target triples:
	/// - **AArch64**: `aarch64-unknown-uefi`
	/// - **x86_64**: `x86_64-unknown-uefi`
	///
	/// # Output
	///
	/// Produces a `.efi` file that can be executed by UEFI firmware.
	///
	/// # Returns
	///
	/// * `Ok(())` - If the loader builds successfully
	/// * `Err(anyhow::Error)` - If the build fails
	fn build_loader(&self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.loader.root,)?;
		cargo_build(&self.opts,)?.arg("--target",).arg(self.opts.arch.loader_tuple(),).run()?;
		set_current_dir(&self.workspace.root,)?;
		Ok((),)
	}

	/// Builds the OSO kernel
	///
	/// This method compiles the OSO kernel as a bare-metal ELF binary that can be
	/// loaded and executed by the OSO loader. The kernel is built with custom
	/// target specifications for bare-metal execution.
	///
	/// # Build Process
	///
	/// 1. Changes to the kernel workspace directory
	/// 2. Executes `cargo build` with a custom target specification JSON file
	/// 3. Returns to the workspace root directory
	///
	/// # Target Specifications
	///
	/// The kernel uses custom JSON target specifications for bare-metal execution:
	/// - **AArch64**: `aarch64-oso.json` - ARM64 bare-metal configuration
	/// - **x86_64**: `x86_64-oso.json` - x86_64 bare-metal configuration
	///
	/// These target files define:
	/// - CPU features and instruction sets
	/// - Memory layout and linking requirements
	/// - ABI and calling conventions
	/// - Panic and exception handling behavior
	///
	/// # Output
	///
	/// Produces an ELF binary that can be loaded by the OSO loader.
	///
	/// # Returns
	///
	/// * `Ok(())` - If the kernel builds successfully
	/// * `Err(anyhow::Error)` - If the build fails
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
	pub fn run(&self,) -> Rslt<(),> {
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

		let mounted_disk = match &self.host_os {
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
				Command::new("sudo",)
					.args(["mount", "-o", "loop",],)
					.args([self.disk_img_path()?, self.mount_point_path()?,],)
					.run()?;
				println!("pre_drop6----------------------");
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
		println!("[pre_drop] from: {} line {}", module_path!(), line!());
		fs_err::create_dir_all(&boot_dir,)?;
		println!("[pre_drop] from: {} line {}", module_path!(), line!());

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

/// Represents the host operating system for platform-specific operations
///
/// The OSO build system needs to perform different operations depending on the
/// host operating system, particularly for disk image mounting and unmounting.
/// This enum encapsulates the supported host platforms and their specific behaviors.
///
/// # Supported Platforms
///
/// - **Mac (macOS)**: Uses `hdiutil` for disk image operations
/// - **Linux**: Uses `mount`/`umount` with `sudo` for disk operations
///
/// # Platform-Specific Operations
///
/// ## macOS (`hdiutil`)
/// - Mounting: `hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount <image>`
/// - Unmounting: `hdiutil detach <device>`
/// - File system mounting: `mount -t msdos <device> <mountpoint>`
///
/// ## Linux (`mount`/`umount`)
/// - Mounting: `sudo mount -o loop <image> <mountpoint>`
/// - Unmounting: `sudo umount <mountpoint>`
///
/// # Examples
///
/// ```rust,ignore
/// let host_os = HostOs::new()?;
/// match host_os {
///     HostOs::Mac => println!("Running on macOS"),
///     HostOs::Linux => println!("Running on Linux"),
/// }
/// ```
#[derive(Debug,)]
pub enum HostOs {
	/// macOS (Darwin) operating system
	Mac,
	/// Linux operating system
	Linux,
}

impl HostOs {
	/// Detects the current host operating system
	///
	/// This method uses the `uname -s` command to identify the host operating system
	/// and returns the appropriate `HostOs` variant. It's used to determine which
	/// platform-specific commands to use for disk operations.
	///
	/// # Detection Method
	///
	/// The detection is performed by:
	/// 1. Executing `uname -s` to get the system name
	/// 2. Parsing the output to identify the OS
	/// 3. Mapping known system names to `HostOs` variants
	///
	/// # Supported Systems
	///
	/// - **"Darwin"** → `HostOs::Mac` (macOS)
	/// - **"Linux"** → `HostOs::Linux` (Linux distributions)
	///
	/// # Returns
	///
	/// * `Ok(HostOs)` - The detected host operating system
	/// * `Err(anyhow::Error)` - If detection fails due to:
	///   - `uname` command not available
	///   - Unsupported operating system (e.g., Windows, FreeBSD)
	///   - Command execution failure
	///   - Invalid UTF-8 in command output
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let host_os = HostOs::new()?;
	/// println!("Detected host OS: {:?}", host_os);
	/// ```
	///
	/// # Errors
	///
	/// This method will return an error for unsupported operating systems:
	/// - Windows (not supported due to different disk mounting mechanisms)
	/// - FreeBSD, OpenBSD, NetBSD (not currently supported)
	/// - Other Unix-like systems not explicitly handled
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
/// This function constructs a `cargo build` command with the appropriate flags
/// and options based on the build configuration. It handles feature flags,
/// build modes, and other cargo-specific settings.
///
/// # Build Configuration
///
/// The function configures the cargo command based on the provided options:
/// - **Build Mode**: Adds `-r` flag for release builds
/// - **Features**: Adds `--features` flag with specified feature list
/// - **Command**: Uses `cargo b` (short form of `cargo build`)
///
/// # Parameters
///
/// * `opts` - The command-line options containing build configuration
///
/// # Returns
///
/// * `Ok(Command)` - A configured cargo build command ready for execution
/// * `Err(anyhow::Error)` - If command creation fails (rare)
///
/// # Generated Commands
///
/// ## Debug Build (default)
/// ```bash
/// cargo b --target <target-triple>
/// ```
///
/// ## Release Build
/// ```bash
/// cargo b -r --target <target-triple>
/// ```
///
/// ## With Features
/// ```bash
/// cargo b --features rgb,bltonly --target <target-triple>
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// use xtask::shell::Opts;
///
/// let opts = Opts::new();
/// let mut cmd = cargo_build(&opts)?;
/// cmd.arg("--target").arg("aarch64-unknown-uefi");
/// cmd.run()?;
/// ```
///
/// # Feature Handling
///
/// If the options specify features, they are added as a space-separated list:
/// - Single feature: `--features rgb`
/// - Multiple features: `--features "rgb bltonly"`
///
/// # Build Mode Handling
///
/// The build mode is determined by the `opts.build_mode` field:
/// - `BuildMode::Debug` - No additional flags (default)
/// - `BuildMode::Release` - Adds the `-r` flag for optimized builds
fn cargo_build(opts: &Opts,) -> Rslt<Command,> {
	let mut cmd = Command::new("cargo",);
	cmd.arg("b",); // Short form of "build"

	// Add feature flags if specified
	if opts.features.len() != 0 {
		cmd.arg("--features",);
		cmd.args(&opts.features,);
	}

	// Add release flag if building in release mode
	if opts.build_mode.is_release() {
		cmd.arg("-r",);
	}

	Ok(cmd,)
}
