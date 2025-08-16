use crate::Rslt;
use crate::cargo::BuildMode;
use crate::cargo::CompileOpt;
use crate::decl_manage::package::Package;
use crate::decl_manage::package::PackageAction;
use crate::decl_manage::package::PackageInfo;
use crate::decl_manage::package::PackageSurvey;
use crate::decl_manage::workspace::Workspace;
use crate::decl_manage::workspace::WorkspaceAction;
use crate::decl_manage::workspace::WorkspaceInfo;
use crate::decl_manage::workspace::WorkspaceSurvey;
use anyhow::anyhow;
use oso_dev_util_helper::cli::Run;
use oso_dev_util_helper::fs::CARGO_CONFIG;
use oso_dev_util_helper::fs::CARGO_MANIFEST;
use oso_dev_util_helper::fs::all_crates_in;
use oso_dev_util_helper::fs::project_root_path;
use oso_dev_util_helper::fs::read_toml;
use oso_proc_macro::FromPathBuf;
use std::ffi::OsStr;
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
	/// return type is not Result because macro ensures that path exists and self has path in
	/// compile time
	fn path(&self,) -> PathBuf;

	fn toml(&self,) -> Rslt<toml::Table,> {
		let cargo_toml = self.path().join(CARGO_MANIFEST,);
		read_toml(cargo_toml,).unwrap_or_else(|| panic!("{CARGO_MANIFEST} must exist"),)
	}

	fn cargo_conf(&self,) -> Option<Rslt<toml::Table,>,> {
		let config_toml = self.path().join(CARGO_CONFIG,);
		read_toml(config_toml,)
	}
}

#[allow(dead_code)]
#[derive(FromPathBuf,)]
pub struct __OsoCrate {
	path: PathBuf,
	#[chart]
	i_am: (),
}

impl From<OsoCrateChart,> for OsoCrate {
	fn from(value: OsoCrateChart,) -> Self {
		Self::from(value.to_path_buf(),)
	}
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

	fn fix(&self,) -> Rslt<(),> {
		todo!()
	}
}

impl CrateInfo for OsoCrate {
	fn path(&self,) -> PathBuf {
		self.path.clone()
	}
}

impl CrateCalled for OsoCrate {
	type F = OsoCrateChart;

	fn whoami(&self,) -> Self::F {
		self.i_am.clone()
	}

	fn path_buf(&self,) -> PathBuf {
		self.path.clone()
	}
}

impl CrateCalled for OsoCrateChart {
	type F = OsoCrateChart;

	fn whoami(&self,) -> Self::F {
		self.clone()
	}

	fn path_buf(&self,) -> PathBuf {
		self.to_path_buf()
	}
}

impl Workspace for OsoCrate {}
impl WorkspaceAction for OsoCrate {}
impl WorkspaceSurvey for OsoCrate {
	#[allow(refining_impl_trait)]
	fn land_on(&mut self, on: impl CrateCalled,) {
		let path = on.path_buf();
		*self = Self::from(path,);
	}
}
impl WorkspaceInfo for OsoCrate {
	#[allow(refining_impl_trait)]
	fn members(&self,) -> Vec<OsoCrate,> {
		all_crates_in(&self.path(),)
			.expect("failed to get some crates within workspace",)
			.iter()
			.map(|p| OsoCrate::from(p.clone(),),)
			.collect()
	}

	#[allow(refining_impl_trait)]
	fn members_with_target(&self, target: impl Into<String,> + Clone,) -> Vec<OsoCrate,> {
		self.members()
			.into_iter()
			.filter(|c| {
				let dflt_targeet: String =
					c.default_target().expect("failed to determine default target",).into();
				let target: String = target.clone().into();
				dflt_targeet == target
			},)
			.collect()
	}
}
impl Package for OsoCrate {}
impl PackageAction for OsoCrate {}
impl PackageSurvey for OsoCrate {
	fn default_target(&self,) -> Rslt<impl Into<String,>,> {
		let host_tuple = || {
			let target = Command::new("rustc",).arg("-vV",).output()?.stdout;
			let target = String::from_utf8(target,)?;
			target
				.lines()
				.find_map(|l| {
					if l.contains("host: ",) {
						Some(l.replace("host: ", "",).to_string(),)
					} else {
						None
					}
				},)
				.ok_or(anyhow!("can't get host target tuple"),)
		};

		Ok(match self.cargo_conf() {
			Some(conf,) => {
				let conf = conf?;
				let conf = conf.get("build",);

				if let Some(toml::Value::Table(t,),) = conf
					&& let Some(toml::Value::String(s,),) = t.get("target",)
				{
					s.clone()
				} else {
					host_tuple()?
				}
			},
			None => host_tuple()?,
		},)
	}

	fn build_artifact(&self, opt: Option<impl CompileOpt,>,) -> Rslt<PathBuf,> {
		let (target_tuple, build_mode,): (String, String,) = match opt {
			Some(opt,) => (opt.target().into(), opt.build_mode().into(),),
			None => (self.default_target()?.into(), BuildMode::default().as_ref().to_string(),),
		};

		let project_root =
			project_root_path()?.join("target",).join(target_tuple,).join(build_mode,);

		Ok(project_root,)
	}
}
impl PackageInfo for OsoCrate {}

pub trait CrateCalled: Eq + Sized + Clone + From<Self::F,> {
	type F: CrateCalled;
	fn whoami(&self,) -> Self::F;
	fn path_buf(&self,) -> PathBuf;
}
