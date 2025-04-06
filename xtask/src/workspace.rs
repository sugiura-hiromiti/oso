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

	pub fn post_process(&self,) -> Rslt<(),> {
		// // xtaskクレートのプロジェクトルートに戻る
		// env::set_current_dir(&self.xtask_root,)?;
		//
		// // パスの用意
		// let mount_point = self.mount_point()?;
		// let img_path = self.img_path()?;
		//
		// // 前回実行時のものを削除
		// if mount_point.exists() {
		// 	Command::new("rm",).arg("-rf",).arg(&mount_point,).run()?;
		// }
		// if img_path.exists() {
		// 	Command::new("rm",).arg("-rf",).arg(&img_path,).run()?;
		// }
		//
		// // raw disk imageを作成
		// let mut create_img = Command::new("qemu-img",);
		// create_img.args(["create", "-f", "raw",],);
		//
		// match &self.arch {
		// 	Architecture::X86_64 => create_img.arg(&img_path,).arg("200m",),
		// 	Architecture::Aarch64 => todo!(),
		// };
		// create_img.run()?;
		//
		// // disk.imgをフォーマット
		// Command::new("mkfs.fat",)
		// 	.args(["-n", "'OSO'", "-s", "2", "-f", "2", "-R", "32", "-F", "32",],)
		// 	.arg(&img_path,)
		// 	.run()?;
		//
		// // マウントポイント作成 & マウント
		// Command::new("mkdir",).arg("-p",).arg(&mount_point,).run()?;
		// let mounted_disk = Command::new("hdiutil",)
		// 	.args(["attach", "-imagekey", "diskimage-class=CRawDiskImage", "-nomount",],)
		// 	.arg(&img_path,)
		// 	.output()?
		// 	.stdout;
		// let mounted_disk = unsafe { String::from_utf8_unchecked(mounted_disk,) };
		// let mounted_disk = mounted_disk.trim().to_string();
		//
		// Command::new("mount",)
		// 	.args(["-t", "msdos",],)
		// 	.arg(&mounted_disk,)
		// 	.arg(&mount_point,)
		// 	.run()?;
		//
		// // bootloader, kernelを配置
		// Command::new("mkdir",).arg("-p",).arg(mount_point.join("efi/boot",),).run()?;
		// match &self.arch {
		// 	Architecture::X86_64 => {
		// 		Command::new("cp",)
		// 			.args([
		// 				self.root.join(&self.loader.build_artifact,),
		// 				mount_point.join("efi/boot/bootx64.efi",),
		// 			],)
		// 			.run()?;
		// 		Command::new("cp",)
		// 			.args([
		// 				self.root.join(&self.kernel.build_artifact,),
		// 				mount_point.join("oso_kernel.elf",),
		// 			],)
		// 			.run()?;
		// 	},
		// 	Architecture::Aarch64 => todo!(),
		// }
		//
		// // unmount
		// Command::new("hdiutil",).args(["detach", &mounted_disk,],).run()?;
		// Ok((),)
		todo!()
	}

	// fn mount_point(&self,) -> Rslt<PathBuf,> {
	// 	let path = PathBuf::from_str(MOUNT_POINT,)?;
	// 	let path = path.join("mnt",);
	// 	Ok(path,)
	// }
	//
	// fn img_path(&self,) -> Rslt<PathBuf,> {
	// 	let path = PathBuf::from_str(MOUNT_POINT,)?;
	// 	let path = path.join("disk.img",);
	// 	Ok(path,)
	// }
}

fn de_toml(path: &Path,) -> Rslt<Table,> {
	let toml_str = fs::read_to_string(&path,)?;
	let table = toml_str.parse::<Table>()?;
	Ok(table,)
}

// fn executable_location(crate_root: &Path, target: &String, crate_name: &String,) ->
// Rslt<PathBuf,> { 	let out = if target.contains(".json",) {
// 		let file = fs::File::open(&crate_root.join(target,),)?;
// 		let reader = BufReader::new(file,);
// 		let json: serde_json::Value = serde_json::from_reader(reader,)?;
// 		let serde_json::Value::Array(opts,) = &json["post-link-args"]["ld.lld"] else {
// 			panic!("your {target}[\"post-link-args\"][\"ld.lld\"] is not array that must be array");
// 		};
// 		let out = opts
// 			.iter()
// 			.find_map(|v| {
// 				let opt = v.as_str().unwrap();
// 				if &opt[..2] == "-o" { Some(&opt[2..],) } else { None }
// 			},)
// 			.expect(&format!(
// 				"you need to specify name of build artifact explicitly in \
// 				 {target}[\"post-link-args\"][\"ld.lld\"]",
// 			),);
// 		out.to_string()
// 	} else if target.contains("uefi",) {
// 		format!("target/{target}/debug/{crate_name}.efi")
// 	} else {
// 		format!("target/{target}/debug/{crate_name}")
// 	};
//
// 	Ok(PathBuf::from_str(&out,)?,)
// }

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
