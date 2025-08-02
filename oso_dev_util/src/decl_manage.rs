
use crate::Rslt;
use std::path::PathBuf;

use crate::decl_manage::package::Package;
use crate::decl_manage::workspace::Workspace;

pub mod package;
pub mod workspace;

pub trait Crate: Workspace + Package {}

pub trait CrateBase {
	fn is_package(&self,) -> bool;
	fn is_workspace(&self,) -> bool;
	fn is_pkg_and_ws(&self,) -> bool {
		self.is_package() && self.is_workspace()
	}
	fn path(&self,) -> Rslt<PathBuf,>;
	fn toml(&self,) -> Rslt<toml::Table,>;
	fn cargo_conf(&self,) -> Rslt<toml::Table,>;
}
