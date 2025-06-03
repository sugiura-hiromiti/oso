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

pub const LOADER: &str = "oso_loader";
pub const KERNEL: &str = "oso_kernel";

#[derive(Debug,)]
pub struct Crate {
	/// this field is equivalent to build.target section of .cargo/config.toml file
	// target:             String,
	/// path to executable
	/// this is relative path to project root
	pub name: String,
	pub root: PathBuf,
}

impl Crate {
	fn new(root_dir: &PathBuf,) -> Rslt<Self,> {
		let manifest = de_toml(&root_dir.join("Cargo.toml",),)?;
		let toml::Value::String(name,) = &manifest["package"]["name"] else {
			panic!("failed to get crate name. check your crate directory: {:?}", root_dir.to_str());
		};

		Ok(Self { root: root_dir.clone(), name: name.clone(), },)
	}
}

impl Architecture {
	pub fn loader_tuple(&self,) -> String {
		format!("{}-unknown-uefi", self.to_string())
	}

	pub fn kernel_tuple(&self,) -> String {
		format!("{}-unknown-none-elf", self.to_string())
	}
}

#[derive(Debug,)]
pub struct OsoWorkSpace {
	pub root:   PathBuf,
	pub loader: Crate,
	pub kernel: Crate,
}

impl OsoWorkSpace {
	//pub fn new(xtask_root: PathBuf, loader_root: PathBuf, kernel_root: PathBuf,) -> Rslt<Self,> {
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

fn de_toml(path: &Path,) -> Rslt<Table,> {
	let toml_str = fs::read_to_string(&path,)?;
	let table = toml_str.parse::<Table>()?;
	Ok(table,)
}

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

pub fn load_json(path: &Path,) -> Rslt<serde_json::Value,> {
	// get content of target json file
	let json = fs_err::File::open(path,)?;
	let reader = BufReader::new(json,);
	let json: serde_json::Value = serde_json::from_reader(reader,)?;

	Ok(json,)
}

/// detect location of output binary which is built by cargo based on target json file
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
