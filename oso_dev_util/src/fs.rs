use crate::Rslt;
use crate::decl_manage::crate_::OsoCrate;
use oso_dev_util_helper::fs::current_crate_path;
use oso_dev_util_helper::fs::project_root_path;

pub fn project_root() -> Rslt<OsoCrate,> {
	let pr = project_root_path();
	Ok(OsoCrate::from(pr,),)
}

pub fn current_crate() -> Rslt<OsoCrate,> {
	let ccp = current_crate_path()?;

	Ok(OsoCrate::from(ccp,),)
}
