use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

const CARGO_MANIFEST: &str = "Cargo.toml";
const CARGO_CONFIG: &str = ".cargo/config.toml";
const CWD: &str = std::env!("CARGO_MANIFEST_DIR");
const IGNORE_DIR_LIST: [&str; 5] = ["target", ".git", ".github", ".direnv", ".cargo",];

/// Checks if the OSO kernel ELF file exists in the target directory
///
/// This function verifies that `target/oso_kernel.elf` exists relative to the current
/// working directory. This is typically used as a prerequisite check before performing
/// ELF analysis operations.
///
/// # Returns
///
/// - `Ok(())` if the kernel file exists
/// - `Err(anyhow::Error)` if the file doesn't exist or if there's an error accessing the current
///   directory
///
/// # Errors
///
/// This function will return an error if:
/// - The current directory cannot be determined
/// - The `oso_kernel.elf` file doesn't exist in the target directory
/// TODO: move to oso_dev_util_helper
pub fn check_oso_kernel() -> Rslt<(),> {
	// Construct the expected path to the kernel ELF file
	let target_path = current_dir()?.join("target/oso_kernel.elf",);

	// Check if the file exists and return appropriate result
	if target_path.exists() { Ok((),) } else { Err(anyhow!("oso_kernel.elf not exist"),) }
}

pub fn all_crates() -> Rslt<Vec<PathBuf,>,> {
	all_crates_in(project_root_path()?,)
}

pub fn all_crates_in(path: PathBuf,) -> Rslt<Vec<PathBuf,>,> {
	Ok(path
		.read_dir()
		.expect(&format!("failed to read {}", path.display()),)
		.filter_map(|entry| {
			if entry.as_ref().expect("failed to get entry",).path().is_file() {
				return None;
			}

			let path = entry.as_ref().expect("failed to get entry",).path();
			let name = path.file_name().unwrap();
			let name = name.to_str().unwrap();
			match name {
				_ if IGNORE_DIR_LIST.contains(&name,) => None,
				_ => Some(path,),
			}
		},)
		.map(|p| {
			let mut paths =
				if search_cargo_toml(&p,)?.is_some() { vec![p.clone()] } else { vec![] };
			paths.append(&mut all_crates_in(p,)?,);
			Ok(paths,)
		},)
		.flat_map(|v: Rslt<Vec<PathBuf,>,>| v.unwrap(),)
		.collect(),)
}

pub fn project_root_path() -> Rslt<PathBuf,> {
	let mut p = PathBuf::from_str(CWD,)?;
	let mut last_cargo_toml = None;

	while p.pop() {
		match search_cargo_toml(&p,)? {
			Some(p,) => last_cargo_toml = Some(p,),
			None => {},
		}
	}

	Ok(last_cargo_toml.unwrap().parent().unwrap().to_path_buf(),)
}

pub fn current_crate_path() -> Rslt<PathBuf,> {
	match search_upstream(CARGO_MANIFEST,) {
		Ok(Some(p,),) => Ok(p,),
		e => Err(anyhow::anyhow!("failed to detect current_crate_path: {e:?}"),),
	}
}

/// depth 1 file search
/// TODO: sophisticate implementation
pub fn search_cargo_toml(path: impl AsRef<Path,>,) -> Rslt<Option<PathBuf,>,> {
	search_in(&path, CARGO_MANIFEST,)
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_search_cargo_toml() -> Rslt<(),> {
		let cargo_toml = search_cargo_toml(CWD,)?.expect("failed to find Cargo.toml",);
		assert_eq!(cargo_toml.to_str().unwrap(), std::env!("CARGO_MANIFEST_PATH"));
		Ok((),)
	}
}
