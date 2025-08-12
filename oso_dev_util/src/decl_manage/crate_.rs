use crate::Rslt;
use crate::decl_manage::package::Package;
use crate::decl_manage::package::PackageAction;
use crate::decl_manage::package::PackageInfo;
use crate::decl_manage::package::PackageSurvey;
use crate::decl_manage::workspace::Workspace;
use crate::decl_manage::workspace::WorkspaceAction;
use crate::decl_manage::workspace::WorkspaceInfo;
use crate::decl_manage::workspace::WorkspaceSurvey;
use oso_dev_util_helper::cli::Run;
use oso_dev_util_helper::fs::CARGO_CONFIG;
use oso_dev_util_helper::fs::CARGO_MANIFEST;
use oso_proc_macro::FromPathBuf;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub trait Crate: Workspace + Package {
	fn as_pkg(&self,) -> &impl Package {
		self
	}

	fn as_wrkspc(&self,) -> &impl Workspace {
		self
	}
}

pub trait CrateSurvey: CrateInfo {
	fn has_parent(&self,) -> Rslt<bool,>;
	fn go_parent(&mut self,) -> Rslt<bool,>;
	fn build_artifact(&self, target: impl Into<String,>,) -> Rslt<PathBuf,>;
	fn fix(&self,) -> Rslt<(),>;
}

/// methods provided keeps environment e.g. current path
pub trait CrateAction: CrateInfo {
	// actions for all packages

	fn build(&self,) -> Rslt<(),> {
		self.cargo_xxx("build",)
	}
	fn test(&self,) -> Rslt<(),> {
		self.cargo_xxx("test",)
	}
	fn run(&self,) -> Rslt<(),> {
		self.cargo_xxx("run",)
	}
	fn ckeck(&self,) -> Rslt<(),> {
		self.cargo_xxx("check",)
	}
	fn fmt(&self,) -> Rslt<(),> {
		self.cargo_xxx("fmt",)
	}
	fn cargo_xxx(&self, cmd: impl AsRef<OsStr,>,) -> Rslt<(),> {
		self.cargo_xxx_with(cmd, &["",],)
	}

	// actions for all packages with specific options

	fn build_with(&self, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		self.cargo_xxx_with("build", opt,)
	}
	fn test_with(&self, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		self.cargo_xxx_with("test", opt,)
	}
	fn run_with(&self, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		self.cargo_xxx_with("run", opt,)
	}
	fn ckeck_with(&self, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		self.cargo_xxx_with("check", opt,)
	}
	fn fmt_with(&self, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		self.cargo_xxx_with("fmt", opt,)
	}
	fn cargo_xxx_with(&self, cmd: impl AsRef<OsStr,>, opt: &[impl AsRef<OsStr,>],) -> Rslt<(),> {
		Command::new("cargo",).arg(cmd,).args(opt,).run()
	}
}
pub trait CrateInfo: CrateCalled {
	fn is_package(&self,) -> Rslt<bool,> {
		let pkg_sec = self.toml()?;
		let pkg_sec = pkg_sec.get("package",);
		match pkg_sec {
			Some(_,) => Ok(true,),
			None => Ok(false,),
		}
	}
	fn is_workspace(&self,) -> Rslt<bool,> {
		let pkg_sec = self.toml()?;
		let pkg_sec = pkg_sec.get("workspace",);
		match pkg_sec {
			Some(_,) => Ok(true,),
			None => Ok(false,),
		}
	}
	fn is_pkg_and_ws(&self,) -> Rslt<bool,> {
		Ok(self.is_package()? && self.is_workspace()?,)
	}

	/// return path to the crate
	fn path(&self,) -> Rslt<PathBuf,>;
	fn toml(&self,) -> Rslt<toml::Table,> {
		let cargo_toml = self.path()?.join(CARGO_MANIFEST,);
		read_toml(cargo_toml,)
	}
	fn cargo_conf(&self,) -> Rslt<toml::Table,> {
		let config_toml = self.path()?.join(CARGO_CONFIG,);
		read_toml(config_toml,)
	}
}

fn read_toml(path: impl AsRef<Path,>,) -> Rslt<toml::Table,> {
	let be_toml = std::fs::read(path,)?;
	let be_toml = String::from_utf8(be_toml,)?;
	let be_toml = be_toml.as_str();
	let be_toml = toml::de::from_str(be_toml,)?;
	Ok(be_toml,)
}

#[derive(FromPathBuf,)]
pub struct __OsoCrate {
	path: PathBuf,
	#[chart]
	i_am: (),
}

impl Crate for OsoCrate {}
impl CrateAction for OsoCrate {}
impl CrateSurvey for OsoCrate {
	fn has_parent(&self,) -> Rslt<bool,> {
		todo!()
	}

	fn go_parent(&mut self,) -> Rslt<bool,> {
		todo!()
	}

	fn build_artifact(&self, target: impl Into<String,>,) -> Rslt<PathBuf,> {
		todo!()
	}

	fn fix(&self,) -> Rslt<(),> {
		todo!()
	}
}

impl CrateInfo for OsoCrate {
	fn path(&self,) -> Rslt<PathBuf,> {
		todo!()
	}
}

impl CrateCalled for OsoCrate {
	type F = PathBuf;
}

impl Workspace for OsoCrate {}
impl WorkspaceAction for OsoCrate {}
impl WorkspaceSurvey for OsoCrate {
	fn land(&mut self, on: impl CrateCalled,) -> impl Crate {
		todo!()
	}
}
impl WorkspaceInfo for OsoCrate {
	fn members(&self,) -> &[impl Crate] {
		todo!()
	}

	fn members_with_target(&self, target: impl Into<String,>,) -> &[impl Crate] {
		todo!()
	}
}
impl Package for OsoCrate {}
impl PackageAction for OsoCrate {}
impl PackageSurvey for OsoCrate {
	fn target(&self,) -> impl Into<String,> {
		todo!()
	}
}
impl PackageInfo for OsoCrate {}

pub trait CrateCalled: Eq + Sized + Clone + From<Self::F,> {
	type F;
	fn whoami(&self,) -> Self {
		self.clone()
	}
}
