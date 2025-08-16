use crate::Rslt;
use crate::cargo::CompileOpt;
use crate::decl_manage::crate_::CrateAction;
use crate::decl_manage::crate_::CrateInfo;
use crate::decl_manage::crate_::CrateSurvey;
use std::path::PathBuf;

pub trait Package: PackageAction + PackageSurvey {
	fn as_action(&self,) -> &impl PackageAction {
		self
	}

	fn as_survey(&self,) -> &impl PackageSurvey {
		self
	}
}

pub trait PackageAction: PackageInfo + CrateAction {}
pub trait PackageSurvey: PackageInfo + CrateSurvey {
	fn default_target(&self,) -> Rslt<impl Into<String,>,>;
	fn build_artifact(&self, target: Option<impl CompileOpt,>,) -> Rslt<PathBuf,>;
}

pub trait PackageInfo: Sized + CrateInfo {}
