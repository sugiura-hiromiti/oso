#![feature(exit_status_error)]

pub mod cli; // Will be added in feat/add-cli-module branch

use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result as Rslt;

const CWD: &str = std::env!("CARGO_MANIFEST_DIR");

pub fn derive_for_enum(item: syn::ItemEnum,) -> proc_macro2::TokenStream {
	let ident = item.ident;
	todo!()
}
pub fn derive_for_struct(item: syn::ItemStruct,) -> proc_macro2::TokenStream {
	todo!()
}

fn all_crates(path: PathBuf,) -> Vec<PathBuf,> {
	path.read_dir()
		.expect(&format!("failed to read {}", path.display()),)
		.filter_map(|entry| {
			if entry.as_ref().expect("failed to get entry",).path().is_file() {
				return None;
			}

			let path = entry.as_ref().expect("failed to get entry",).path();
			let name = path.file_name().unwrap();
			let name = name.to_str().unwrap();
			match name {
				"target" | ".git" | ".github" | ".direnv" | ".cargo" => None,
				_ => Some(path,),
			}
		},)
		.map(|p| {
			let mut paths = if search_cargo_toml(&p,).is_some() { vec![p.clone()] } else { vec![] };
			paths.append(&mut all_crates(p,),);
		},);

	todo!()
}

fn project_root() -> PathBuf {
	let mut p = PathBuf::from_str(CWD,).expect("failed to create PathBuf value",);
	let mut last_cargo_toml = None;

	while p.pop() {
		match search_cargo_toml(&p,) {
			Some(p,) => last_cargo_toml = Some(p,),
			None => {},
		}
	}

	last_cargo_toml.unwrap().parent().unwrap().to_path_buf()
}

/// depth 1 file search
fn search_cargo_toml(path: impl AsRef<Path,>,) -> Option<PathBuf,> {
	path.as_ref()
		.read_dir()
		.expect("failed to read dir",)
		.find(|entry| {
			entry.as_ref().expect("failed to get entry",).file_name().to_str().unwrap()
				== "Cargo.toml"
		},)
		.map(|entry| entry.unwrap().path(),)
}
