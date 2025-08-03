use crate::decl_manage::crate_::CrateAction;
use crate::decl_manage::crate_::CrateInfo;
use crate::decl_manage::crate_::CrateSurvey;

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
	fn target(&self,) -> impl Into<String,>;
}

pub trait PackageInfo: Sized + CrateInfo {}
