use crate::decl_manage::package::Package;
use anyhow::Result as Rslt;
use std::path::PathBuf;

pub trait Workspace: WorkspaceAction + WorkspaceSurvey {
	fn as_action(&self,) -> &impl WorkspaceAction {
		self
	}
}

pub trait WorkspaceAction: WorkspaceInfo {
	// actions for all packages

	fn build(&self,) -> Rslt<(),>;
	fn test(&self,) -> Rslt<(),>;
	fn run(&self,) -> Rslt<(),>;
	fn lint(&self,) -> Rslt<(),>;
	fn cargo_xxx(&self, cmd: impl Into<String,>,) -> Rslt<(),>;

	// actions for all packages with specific options

	fn build_with(&self, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn test_with(&self, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn run_with(&self, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn lint_with(&self, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn cargo_xxx_with(&self, cmd: impl Into<String,>, opt: &[impl Into<String,>],) -> Rslt<(),>;

	// actions for specific package

	fn build_at(&self, at: impl Package,) -> Rslt<(),>;
	fn test_at(&self, at: impl Package,) -> Rslt<(),>;
	fn run_at(&self, at: impl Package,) -> Rslt<(),>;
	fn lint_at(&self, at: impl Package,) -> Rslt<(),>;
	fn cargo_xxx_at(&self, at: impl Package, cmd: impl Into<String,>,) -> Rslt<(),>;

	// actions for specific package with specific options

	fn build_at_with(&self, at: impl Package, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn test_at_with(&self, at: impl Package, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn run_at_with(&self, at: impl Package, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn lint_at_with(&self, at: impl Package, opt: &[impl Into<String,>],) -> Rslt<(),>;
	fn cargo_xxx_at_with(
		&self,
		at: impl Package,
		cmd: impl Into<String,>,
		opt: &[impl Into<String,>],
	) -> Rslt<(),>;
}

pub trait WorkspaceSurvey: WorkspaceInfo {}

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
pub trait WorkspaceInfo: Sized {
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
	fn root(&self,) -> PathBuf;

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
	fn crates(&self,) -> &[PathBuf];

	fn crates_with_target(&self, target: impl Into<String,>,) -> &[PathBuf];
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
pub struct OsoWorkspace {
	/// The root directory of the workspace
	root: PathBuf,
}

impl OsoWorkspace {
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
