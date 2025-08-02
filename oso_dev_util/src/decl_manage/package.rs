use anyhow::Result as Rslt;
use std::path::PathBuf;

pub trait Package: PackageAction + PackageSurvey {}

pub trait PackageAction: PackageInfo {}
pub trait PackageSurvey: PackageInfo {}

pub trait PackageInfo: Sized {
	fn path(&self,) -> Rslt<PathBuf,>;
	fn toml(&self,) -> Rslt<toml::Table,>;
	fn target(&self,) -> Rslt<impl Into<String,>,>;
}

pub struct OsoPackage;
pub enum OsoPackageCalled {}
