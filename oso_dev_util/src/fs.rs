//  TODO: merge this with oso_proc_macro_logic_2 into oso_dev_util_helper
use anyhow::anyhow;

use crate::Rslt;
use crate::decl_manage::crate_::OsoCrate;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;

pub const CARGO_MANIFEST: &str = "Cargo.toml";
pub const CARGO_CONFIG: &str = ".cargo/config.toml";
pub const PROJECT_ROOT_MARKER: &str = ".git";

pub fn project_root() -> Rslt<OsoCrate,> {
	let prp = project_root_path()?;
	Ok(OsoCrate::from(prp,),)
}

pub fn project_root_path() -> Rslt<PathBuf,> {
	get_upstream(PROJECT_ROOT_MARKER,)
}

pub fn current_crate() -> Rslt<OsoCrate,> {
	let ccp = current_crate_path()?;
	Ok(OsoCrate::from(ccp,),)
}

pub fn current_crate_path() -> Rslt<PathBuf,> {
	get_upstream(CARGO_MANIFEST,)
}

pub fn search_in(
	place: &impl AsRef<Path,>,
	file_name: impl Into<String,> + Clone,
) -> Rslt<Option<PathBuf,>,> {
	let rslt = std::fs::read_dir(place,)?
		.find(|entry| {
			entry.as_ref().expect("failed to get dir entry",).file_name().to_str().unwrap()
				== file_name.clone().into()
		},)
		.map(|entry| entry.map(|entry| entry.path(),),)
		.transpose()?;
	Ok(rslt,)
}

/// not recursively
pub fn search_in_cwd(file_name: impl Into<String,> + Clone,) -> Rslt<Option<PathBuf,>,> {
	let cwd = current_dir()?;
	search_in(&cwd, file_name,)
}

pub fn get_upstream(file_name: impl Into<String,> + Clone,) -> Rslt<PathBuf,> {
	match search_upstream(file_name.clone(),) {
		Ok(None,) => Err(anyhow!("can not find out {} file", file_name.into()),),
		p => p.map(|p| p.unwrap(),),
	}
}

pub fn search_upstream(file_name: impl Into<String,> + Clone,) -> Rslt<Option<PathBuf,>,> {
	let mut place = current_dir()?;
	loop {
		if place.pop() {
			if let Some(p,) = search_in(&place, file_name.clone(),)? {
				break Ok(Some(p,),);
			}
		} else {
			break Ok(None,);
		}
	}
}
