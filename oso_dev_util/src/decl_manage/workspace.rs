use crate::decl_manage::crate_::Crate;
use crate::decl_manage::crate_::CrateAction;
use crate::decl_manage::crate_::CrateCalled;
use crate::decl_manage::crate_::CrateInfo;
use crate::decl_manage::crate_::CrateSurvey;
use anyhow::Result as Rslt;
use std::ffi::OsStr;

pub trait Workspace: WorkspaceAction + WorkspaceSurvey {
	fn as_action(&self,) -> &impl WorkspaceAction {
		self
	}

	fn as_survey(&self,) -> &impl WorkspaceSurvey {
		self
	}
}

pub trait WorkspaceAction: WorkspaceInfo + CrateAction {
	// actions for specific package

	fn build_at(&self, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at("build", at,)
	}
	fn test_at(&self, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at("test", at,)
	}
	fn run_at(&self, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at("run", at,)
	}
	fn check_at(&self, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at("check", at,)
	}
	fn fmt_at(&self, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at("fmt", at,)
	}
	fn cargo_xxx_at(&self, cmd: impl AsRef<OsStr,>, at: impl CrateCalled,) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with(cmd, at, &["",],)
	}

	// actions for specific package with specific options

	fn build_at_with(&self, at: impl CrateCalled, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with("build", at, opt,)
	}
	fn test_at_with(&self, at: impl CrateCalled, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with("test", at, opt,)
	}
	fn run_at_with(&self, at: impl CrateCalled, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with("run", at, opt,)
	}
	fn check_at_with(&self, at: impl CrateCalled, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with("check", at, opt,)
	}
	fn fmt_at_with(&self, at: impl CrateCalled, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),>
	where Self: WorkspaceSurvey {
		self.cargo_xxx_at_with("fmt", at, opt,)
	}
	fn cargo_xxx_at_with(
		&self,
		cmd: impl AsRef<OsStr,>,
		at: impl CrateCalled,
		opt: &[impl AsRef<OsStr,>],
	) -> Rslt<(),>
	where
		Self: WorkspaceSurvey,
	{
		let current = self.whoami();
		//  this operation is safe due to `&self` is valid
		let self_mut = unsafe { (self as *const Self).cast_mut().as_mut().unwrap() };
		self_mut.land_on(at,).cargo_xxx_with(cmd, opt,)?;
		self_mut.land_on(current,);
		Ok((),)
	}
}

pub trait WorkspaceSurvey: WorkspaceInfo + CrateSurvey {
	fn land_on(&mut self, on: impl CrateCalled,) -> impl Crate;
}

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
pub trait WorkspaceInfo: Sized + CrateInfo {
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
	fn members(&self,) -> &[impl Crate];

	fn members_with_target(&self, target: impl Into<String,>,) -> &[impl Crate];
}
