//! # Workspace Module
//!
//! This module handles workspace management and path resolution.
//!
//! It provides:
//! - Functions for detecting the OSO workspace structure
//! - Functions for loading and parsing target JSON files
//! - Functions for detecting build artifacts
//! - Structs for representing crates and workspaces

use crate::shell::Architecture;
use anyhow::Result as Rslt;
use anyhow::anyhow;
use anyhow::bail;
use std::env;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use toml::Table;

/// Name of the OSO loader crate
pub const LOADER: &str = "oso_loader";
/// Name of the OSO kernel crate
pub const KERNEL: &str = "oso_kernel";

/// Represents a crate in the OSO workspace
#[derive(Debug,)]
pub struct Crate {
	/// Name of the crate
	pub name: String,
	/// Root directory of the crate
	pub root: PathBuf,
}

impl Crate {
	/// Creates a new Crate instance from a root directory
	///
	/// # Parameters
	///
	/// * `root_dir` - The root directory of the crate
	///
	/// # Returns
	///
	/// A new Crate instance or an error if initialization fails
	fn new(root_dir: &PathBuf,) -> Rslt<Self,> {
		let manifest = de_toml(&root_dir.join("Cargo.toml",),)?;
		let toml::Value::String(name,) = &manifest["package"]["name"] else {
			panic!("failed to get crate name. check your crate directory: {:?}", root_dir.to_str());
		};

		Ok(Self { root: root_dir.clone(), name: name.clone(), },)
	}
}

impl Architecture {
	/// Gets the target triple for the loader
	///
	/// # Returns
	///
	/// The target triple for the loader (e.g., "aarch64-unknown-uefi")
	pub fn loader_tuple(&self,) -> String {
		format!("{}-unknown-uefi", self.to_string())
	}

	/// Gets the target triple for the kernel
	///
	/// # Returns
	///
	/// The target triple for the kernel (e.g., "aarch64-unknown-none-elf")
	pub fn kernel_tuple(&self,) -> String {
		format!("{}-unknown-none-elf", self.to_string())
	}
}

/// Represents the OSO workspace
#[derive(Debug,)]
pub struct OsoWorkSpace {
	/// Root directory of the OSO workspace
	pub root:   PathBuf,
	/// Loader crate
	pub loader: Crate,
	/// Kernel crate
	pub kernel: Crate,
}

impl OsoWorkSpace {
	/// Creates a new OsoWorkSpace instance
	///
	/// Detects the OSO workspace structure and initializes the loader and kernel crates.
	///
	/// # Returns
	///
	/// A new OsoWorkSpace instance or an error if initialization fails
	pub fn new() -> Rslt<Self,> {
		let cur_root = env::var("CARGO_MANIFEST_DIR",).unwrap_or_else(|e| {
			eprintln!("error of getting `CARGO_MANIFEST_DIR`: {e}");
			env!("CARGO_MANIFEST_DIR").to_string()
		},);
		let root = oso_root(Path::new(&cur_root,),);
		let loader_root = root.join(LOADER,);
		let kernel_root = root.join(KERNEL,);

		let loader = Crate::new(&loader_root,)?;
		let kernel = Crate::new(&kernel_root,)?;

		Ok(Self { root, loader, kernel, },)
	}
}

/// Parses a TOML file
///
/// # Parameters
///
/// * `path` - The path to the TOML file
///
/// # Returns
///
/// A parsed TOML table or an error if parsing fails
fn de_toml(path: &Path,) -> Rslt<Table,> {
	let toml_str = fs::read_to_string(&path,)?;
	let table = toml_str.parse::<Table>()?;
	Ok(table,)
}

/// Finds the OSO root directory
///
/// # Parameters
///
/// * `path` - The starting path
///
/// # Returns
///
/// The path to the OSO root directory
fn oso_root(path: &Path,) -> PathBuf {
	let p: PathBuf = path
		.iter()
		.take_while(|s| {
			let s = s.to_str().unwrap();
			s != "oso"
		},)
		.collect();
	p.join("oso",)
}

/// Loads and parses a JSON file
///
/// # Parameters
///
/// * `path` - The path to the JSON file
///
/// # Returns
///
/// A parsed JSON value or an error if parsing fails
pub fn load_json(path: &Path,) -> Rslt<serde_json::Value,> {
	// get content of target json file
	let json = fs_err::File::open(path,)?;
	let reader = BufReader::new(json,);
	let json: serde_json::Value = serde_json::from_reader(reader,)?;

	Ok(json,)
}

/// Detects the location of the output binary based on the target JSON file
///
/// # Parameters
///
/// * `json` - The parsed JSON value from the target JSON file
///
/// # Returns
///
/// The path to the output binary or an error if detection fails
pub fn detect_build_artifact(json: serde_json::Value,) -> Rslt<PathBuf,> {
	let serde_json::Value::Array(opts,) = &json["post-link-args"]["ld.lld"] else {
		bail!("[\"post-link-args\"][\"ld.lld\"] in target json is not array that must be array");
	};

	let out = opts
		.iter()
		.find_map(|v| {
			if let Some(v,) = v.as_str() {
				if &v[..2] == "-o" { Some(&v[2..],) } else { None }
			} else {
				None
			}
		},)
		.ok_or(anyhow!(
			"output location of kernel binary does not specified in target json file"
		),)?;

	Ok(PathBuf::from_str(out,)?,)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn crate_new() -> Rslt<(),> {
		let loader_root = std::env::current_dir()?.parent().unwrap().to_path_buf().join(LOADER,);
		let n = Crate::new(&loader_root,)?;
		assert_eq!(n.name, "oso_loader");
		println!("{n:#?}");
		Ok((),)
	}
}
