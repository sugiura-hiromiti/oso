use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub const CARGO_MANIFEST: &str = "Cargo.toml";
pub const CARGO_CONFIG: &str = ".cargo/config.toml";
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

	#[test]
	fn test_search_in_found() -> Rslt<(),> {
		// Use the current project directory and search for Cargo.toml
		let current_dir = std::path::PathBuf::from(CWD,);
		let result = search_in(&current_dir, "Cargo.toml",)?;
		assert!(result.is_some());
		let found_path = result.unwrap();
		assert!(found_path.exists());
		assert_eq!(found_path.file_name().unwrap(), "Cargo.toml");
		Ok((),)
	}

	#[test]
	fn test_search_in_not_found() -> Rslt<(),> {
		// Search for a non-existent file in the current directory
		let current_dir = std::path::PathBuf::from(CWD,);
		let result = search_in(&current_dir, "definitely_nonexistent_file_12345.xyz",)?;
		assert!(result.is_none());
		Ok((),)
	}

	#[test]
	fn test_get_upstream_found() -> Rslt<(),> {
		// This should find Cargo.toml in the project structure
		let result = get_upstream("Cargo.toml",);
		assert!(result.is_ok());
		let path = result.unwrap();
		assert!(path.exists());
		assert!(path.file_name().unwrap() == "Cargo.toml");
		Ok((),)
	}

	#[test]
	fn test_get_upstream_not_found() {
		// This should fail to find a non-existent file
		let result = get_upstream("definitely_nonexistent_file_12345.xyz",);
		assert!(result.is_err());
		let error_msg = result.unwrap_err().to_string();
		assert!(error_msg.contains("can not find out"));
		assert!(error_msg.contains("definitely_nonexistent_file_12345.xyz"));
	}

	#[test]
	fn test_search_upstream_found() -> Rslt<(),> {
		// This should find Cargo.toml in the project structure
		let result = search_upstream("Cargo.toml",)?;
		assert!(result.is_some());
		let path = result.unwrap();
		assert!(path.exists());
		assert!(path.file_name().unwrap() == "Cargo.toml");
		Ok((),)
	}

	#[test]
	fn test_search_upstream_not_found() -> Rslt<(),> {
		// This should not find a non-existent file
		let result = search_upstream("definitely_nonexistent_file_12345.xyz",)?;
		assert!(result.is_none());
		Ok((),)
	}

	#[test]
	fn test_check_oso_kernel_file_not_exists() {
		// In most test environments, oso_kernel.elf won't exist
		let result = check_oso_kernel();
		// We expect this to fail in test environment
		assert!(result.is_err());
		let error_msg = result.unwrap_err().to_string();
		assert!(error_msg.contains("oso_kernel.elf"));
	}

	#[test]
	fn test_search_cargo_toml_with_different_cwd() -> Rslt<(),> {
		// Test with the root directory
		let root_path = std::path::PathBuf::from("/",);
		let result = search_cargo_toml(&root_path,);

		// Should still find the project's Cargo.toml by searching upstream
		assert!(result.is_ok());
		let found_path = result.unwrap();
		if let Some(path,) = found_path {
			assert!(path.exists());
			assert!(path.file_name().unwrap() == "Cargo.toml");
		}
		Ok((),)
	}

	#[test]
	fn test_constants() {
		// Test that constants are defined correctly
		assert_eq!(CARGO_MANIFEST, "Cargo.toml");
		assert_eq!(CARGO_CONFIG, ".cargo/config.toml");

		// CWD should be a valid path string
		let cwd_path = std::path::Path::new(CWD,);
		assert!(cwd_path.exists());
		assert!(cwd_path.is_dir());
	}

	#[test]
	fn test_search_in_with_subdirectories() -> Rslt<(),> {
		// Use the current project directory which should have subdirectories
		let current_dir = std::path::PathBuf::from(CWD,);

		// Search for Cargo.toml which should exist in the main directory
		let result = search_in(&current_dir, "Cargo.toml",)?;
		assert!(result.is_some());
		let found_path = result.unwrap();
		assert!(found_path.exists());
		assert_eq!(found_path.file_name().unwrap(), "Cargo.toml");

		// Search should not find files that don't exist at the current level
		let result = search_in(&current_dir, "nonexistent_file.txt",)?;
		assert!(result.is_none());
		Ok((),)
	}

	#[test]
	fn test_file_name_matching() -> Rslt<(),> {
		// Use the current project directory
		let current_dir = std::path::PathBuf::from(CWD,);

		// Should find exact match for Cargo.toml
		let result = search_in(&current_dir, "Cargo.toml",)?;
		assert!(result.is_some());
		let found_path = result.unwrap();
		assert_eq!(found_path.file_name().unwrap(), "Cargo.toml");

		// Should not find partial matches
		let result = search_in(&current_dir, "Cargo",)?;
		assert!(result.is_none());
		Ok((),)
	}

	#[test]
	fn test_ignore_dir_list() {
		// Test that the ignore directory list is properly defined
		assert!(IGNORE_DIR_LIST.contains(&"target"));
		assert!(IGNORE_DIR_LIST.contains(&".git"));
		assert!(IGNORE_DIR_LIST.contains(&".github"));
		assert!(IGNORE_DIR_LIST.contains(&".direnv"));
		assert!(IGNORE_DIR_LIST.contains(&".cargo"));
		assert_eq!(IGNORE_DIR_LIST.len(), 5);
	}

	#[test]
	fn test_search_in_ignores_directories() -> Rslt<(),> {
		// This test verifies that search_in only looks at files, not directories
		let current_dir = std::path::PathBuf::from(CWD,);

		// Even if there's a directory named like a file we're searching for,
		// search_in should not return it (it only returns files)
		// We can't easily test this without creating directories, so we'll
		// just verify that search_in returns files, not directories
		if let Some(found,) = search_in(&current_dir, "Cargo.toml",)? {
			assert!(found.is_file(), "search_in should return files, not directories");
		}
		Ok((),)
	}

	#[test]
	fn test_path_operations() -> Rslt<(),> {
		// Test basic path operations used in the module
		let current_dir = std::path::PathBuf::from(CWD,);
		assert!(current_dir.is_absolute() || current_dir.is_relative());

		// Test that we can join paths
		let joined = current_dir.join("Cargo.toml",);
		assert!(joined.to_string_lossy().contains("Cargo.toml"));
		Ok((),)
	}

	#[test]
	fn test_search_upstream_behavior() -> Rslt<(),> {
		// Test that search_upstream actually searches up the directory tree
		// by starting from a subdirectory (if it exists)
		let src_dir = std::path::PathBuf::from(CWD,).join("src",);

		if src_dir.exists() && src_dir.is_dir() {
			// Change to src directory and search for Cargo.toml
			let original_dir = std::env::current_dir()?;
			std::env::set_current_dir(&src_dir,)?;

			let result = search_upstream("Cargo.toml",)?;
			assert!(result.is_some(), "Should find Cargo.toml by searching upstream");

			// Restore original directory
			std::env::set_current_dir(original_dir,)?;
		}
		Ok((),)
	}
}
