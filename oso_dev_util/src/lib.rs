//! # OSO Development Utilities
//!
//! A collection of development utilities and helper functions for the OSO operating system project.
//! This crate provides tools for workspace management, command execution, and development workflow
//! automation.
//!
//! ## Features
//!
//! - **Workspace Management**: Tools for managing multi-crate workspaces
//! - **Command Execution**: Enhanced command execution with better error handling and output
//!   formatting
//! - **Development Workflow**: Utilities to streamline the development process
//! - **Cross-platform Support**: Works across different operating systems
//!
//! ## Key Components
//!
//! ### Command Execution
//!
//! The [`Run`] trait provides enhanced command execution capabilities with:
//! - Colored output formatting
//! - Automatic error handling
//! - Inherited stdio streams
//! - Command display with arguments
//!
//! ### Workspace Management
//!
//! The workspace management system provides:
//! - Root directory detection
//! - Crate enumeration and management
//! - Workspace-wide operations
//!
//! ## Usage
//!
//! ### Basic Command Execution
//!
//! ```rust,no_run
//! use oso_dev_util::Run;
//! use std::process::Command;
//!
//! // Execute a command with enhanced output
//! let mut cmd = Command::new("cargo",);
//! cmd.args(&["build", "--release",],);
//! cmd.run().expect("Build failed",);
//! ```
//!
//! ### Workspace Operations
//!
//! ```rust,ignore
//! use oso_dev_util::{OsoWorkspace, OsoWorkspaceManager};
//!
//! let workspace = OsoWorkspaceManager::new();
//! let root = workspace.root();
//! let crates = workspace.crates();
//!
//! println!("Workspace root: {}", root.display());
//! for crate_path in crates {
//!     println!("Crate: {}", crate_path.display());
//! }
//! ```
//!
//! ## Dependencies
//!
//! - [`anyhow`]: Error handling and context
//! - [`colored`]: Terminal color output
//! - [`toml`]: TOML configuration file parsing

#![feature(exit_status_error)]

use anyhow::Result as Rslt;
use colored::Colorize;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

/// The path to the oso_dev_util crate manifest, set at compile time
const OSO_DEV_UTIL_PATH: &'static str = std::env!("CARGO_MANIFEST_PATH");

/// Trait for managing OSO workspace operations
///
/// This trait provides an interface for workspace management operations including
/// root directory access and crate enumeration. It's designed to work with
/// multi-crate Rust workspaces and provides a consistent API for workspace operations.
///
/// # Type Parameters
///
/// * `'a` - Lifetime parameter for borrowed path references
///
/// # Examples
///
/// ```rust,ignore
/// use oso_dev_util::OsoWorkspace;
///
/// fn process_workspace<W: OsoWorkspace>(workspace: &W) {
///     let root = workspace.root();
///     println!("Processing workspace at: {}", root.display());
///     
///     for crate_path in workspace.crates() {
///         println!("Found crate: {}", crate_path.display());
///     }
/// }
/// ```
pub trait OsoWorkspace<'a,> {
	/// Returns the root directory of the workspace
	///
	/// # Returns
	///
	/// A reference to the [`Path`] representing the workspace root directory.
	/// This is typically the directory containing the workspace `Cargo.toml` file.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let root = workspace.root();
	/// assert!(root.join("Cargo.toml").exists());
	/// ```
	fn root(&self,) -> &'a Path;

	/// Returns a slice of paths to all crates in the workspace
	///
	/// # Returns
	///
	/// A slice of [`Path`] references, each pointing to a crate directory
	/// within the workspace. These paths are relative to the workspace root.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let crates = workspace.crates();
	/// for crate_path in crates {
	///     let cargo_toml = crate_path.join("Cargo.toml");
	///     assert!(cargo_toml.exists());
	/// }
	/// ```
	fn crates(&self,) -> &'a [&'a Path];
}

/// Concrete implementation of workspace management for OSO projects
///
/// `OsoWorkspaceManager` provides a concrete implementation of the [`OsoWorkspace`] trait,
/// managing workspace operations for OSO operating system development. It handles
/// workspace root detection, crate enumeration, and workspace-wide operations.
///
/// # Fields
///
/// * `root` - The root directory of the workspace
/// * `crates` - A slice of paths to individual crates within the workspace
///
/// # Examples
///
/// ```rust,ignore
/// use oso_dev_util::{OsoWorkspaceManager, OsoWorkspace};
///
/// let manager = OsoWorkspaceManager::new();
/// let root = manager.root();
/// let crates = manager.crates();
///
/// println!("Managing workspace at: {}", root.display());
/// println!("Found {} crates", crates.len());
/// ```
pub struct OsoWorkspaceManager<'a,> {
	/// The root directory of the workspace
	root:   &'a Path,
	/// Paths to all crates in the workspace
	crates: &'a [&'a Path],
}

impl<'a,> OsoWorkspaceManager<'a,> {
	/// Creates a new workspace manager instance
	///
	/// This constructor initializes a new `OsoWorkspaceManager` by detecting the
	/// workspace root and enumerating all crates within the workspace.
	///
	/// # Returns
	///
	/// A new `OsoWorkspaceManager` instance configured for the current workspace.
	///
	/// # Panics
	///
	/// This function may panic if:
	/// - The workspace root cannot be determined
	/// - The workspace configuration is invalid
	/// - Required workspace files are missing
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let manager = OsoWorkspaceManager::new();
	/// ```
	///
	/// # TODO
	///
	/// - Implement workspace root detection
	/// - Add crate enumeration logic
	/// - Handle workspace configuration parsing
	fn new() -> Self {
		todo!("Implement workspace detection and crate enumeration")
	}
}

impl<'a,> OsoWorkspace<'a,> for OsoWorkspaceManager<'a,> {
	/// Returns the root directory of the workspace
	///
	/// # Returns
	///
	/// A reference to the [`Path`] representing the workspace root directory.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let manager = OsoWorkspaceManager::new();
	/// let root = manager.root();
	/// println!("Workspace root: {}", root.display());
	/// ```
	fn root(&self,) -> &'a Path {
		todo!("Return the workspace root path")
	}

	/// Returns a slice of paths to all crates in the workspace
	///
	/// # Returns
	///
	/// A slice of [`Path`] references pointing to crate directories.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// let manager = OsoWorkspaceManager::new();
	/// let crates = manager.crates();
	/// for crate_path in crates {
	///     println!("Crate: {}", crate_path.display());
	/// }
	/// ```
	fn crates(&self,) -> &'a [&'a Path] {
		todo!("Return the list of crate paths")
	}
}

/// Trait for enhanced command execution with better error handling and output formatting
///
/// The `Run` trait extends the standard [`Command`] functionality with:
/// - Colored command output display
/// - Automatic stdio inheritance
/// - Enhanced error handling with context
/// - Command argument formatting
///
/// This trait is particularly useful for development tools and build scripts where
/// clear command output and error reporting are essential.
///
/// # Examples
///
/// ```rust,no_run
/// use oso_dev_util::Run;
/// use std::process::Command;
///
/// let mut cmd = Command::new("ls",);
/// cmd.args(&["-la", "/tmp",],);
///
/// match cmd.run() {
/// 	Ok((),) => println!("Command executed successfully"),
/// 	Err(e,) => eprintln!("Command failed: {}", e),
/// }
/// ```
pub trait Run {
	/// Executes the command with enhanced output and error handling
	///
	/// This method runs the command while providing:
	/// - Colored display of the command being executed
	/// - Inherited stdio streams for interactive commands
	/// - Proper error handling with exit code checking
	/// - Formatted command argument display
	///
	/// # Returns
	///
	/// * `Ok(())` - If the command executed successfully (exit code 0)
	/// * `Err(anyhow::Error)` - If the command failed or returned a non-zero exit code
	///
	/// # Errors
	///
	/// This method will return an error if:
	/// - The command cannot be found or executed
	/// - The command returns a non-zero exit code
	/// - There are I/O errors during command execution
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use oso_dev_util::Run;
	/// use std::process::Command;
	///
	/// // Execute a simple command
	/// let mut cmd = Command::new("echo",);
	/// cmd.arg("Hello, World!",);
	/// cmd.run().expect("Echo command failed",);
	///
	/// // Execute a build command
	/// let mut build_cmd = Command::new("cargo",);
	/// build_cmd.args(&["build", "--release",],);
	/// build_cmd.run().expect("Build failed",);
	/// ```
	///
	/// # Output Format
	///
	/// The method displays the command in the following format:
	/// ```text
	/// program_name arg1 arg2 arg3
	/// ```
	/// The command line is displayed in bold blue text for easy identification.
	fn run(&mut self,) -> Rslt<(),>;
}

impl Run for Command {
	/// Executes the command with enhanced formatting and error handling
	///
	/// This implementation provides a user-friendly command execution experience
	/// with colored output, proper error handling, and stdio inheritance.
	///
	/// # Implementation Details
	///
	/// 1. **Command Display**: Formats and displays the command with arguments in bold blue
	/// 2. **Stdio Configuration**: Inherits stdout, stderr, and stdin from the parent process
	/// 3. **Execution**: Runs the command and waits for completion
	/// 4. **Error Checking**: Validates the exit status and converts errors to `anyhow::Error`
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use oso_dev_util::Run;
	/// use std::process::Command;
	///
	/// let mut cmd = Command::new("git",);
	/// cmd.args(&["status", "--porcelain",],);
	///
	/// // This will display: git status --porcelain
	/// // in bold blue, then execute the command
	/// cmd.run().expect("Git command failed",);
	/// ```
	fn run(&mut self,) -> Rslt<(),> {
		// Format the command display string with program and arguments
		let cmd_dsply = format!(
			"{} {}",
			self.get_program().display(),
			self.get_args().collect::<Vec<&OsStr,>>().join(OsStr::new(" ")).display()
		);

		// Display the command in bold blue for visibility
		println!("\n{}", cmd_dsply.bold().blue());

		// Configure stdio inheritance and execute the command
		let out = self
			.stdout(Stdio::inherit(),)  // Inherit stdout for real-time output
			.stderr(Stdio::inherit(),)  // Inherit stderr for error messages
			.stdin(Stdio::inherit(),)   // Inherit stdin for interactive commands
			.status()?; // Execute and get exit status

		// Check exit status and convert to Result
		out.exit_ok()?; // This will return an error if exit code != 0
		Ok((),)
	}
}
