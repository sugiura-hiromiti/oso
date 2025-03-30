#![feature(string_from_utf8_lossy_owned)]

use anyhow::Result as Rslt;
use anyhow::anyhow;
use colored::Colorize;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;
use toml::Table;

const LOADER: &str = "oso_loader";
const KERNEL: &str = "oso_kernel";

trait Run {
	fn run(&mut self,) -> Rslt<String,>;
}

impl Run for Command {
	fn run(&mut self,) -> Rslt<String,> {
		let full_cmd = format!(
			"{:?} {:?}",
			self.get_program(),
			self.get_args().collect::<Vec<_,>>().join(OsStr::new(" "))
		)
		.green();
		println!("🫠 {full_cmd}");
		let out = self.output()?;
		let stdout = String::from_utf8_lossy_owned(out.stdout,);
		let stderr = String::from_utf8_lossy_owned(out.stderr,);

		if !stdout.is_empty() {
			println!("{stdout}");
		}
		if !stderr.is_empty() {
			println!("{stderr}");
		}

		if out.status.success() {
			Ok(stdout.trim().to_string(),)
		} else {
			Err(anyhow!("command execution has failed\n\tstderr: {stderr}"),)
		}
	}
}

#[derive(Debug,)]
enum Architecture {
	X86_64,
	Aarch64,
}

impl TryFrom<&String,> for Architecture {
	type Error = anyhow::Error;

	fn try_from(value: &String,) -> Result<Self, Self::Error,> {
		let arch = if value.contains("x86_64",) {
			Self::X86_64
		} else if value.contains("aarch64",) {
			Self::Aarch64
		} else {
			return Err(anyhow!("target {value} is not supported"),);
		};

		Ok(arch,)
	}
}

impl ToString for Architecture {
	fn to_string(&self,) -> String {
		match self {
			Architecture::X86_64 => "x86_64",
			Architecture::Aarch64 => "aarch64",
		}
		.to_string()
	}
}

#[derive(Debug,)]
struct Crate {
	/// this field is equivalent to build.target section of .cargo/config.toml file
	target:         String,
	/// path to executable
	/// this is relative path to project root
	build_artifact: PathBuf,
}

impl Crate {
	fn new(root_dir: &PathBuf,) -> Rslt<Self,> {
		let manifest = de_toml(&root_dir.join("Cargo.toml",),)?;
		let toml::Value::String(name,) = &manifest["package"]["name"] else {
			panic!("failed to get crate name. check your crate directory: {:?}", root_dir.to_str());
		};
		let target = target_tuple(&root_dir,)?;
		let build_artifact = executable_location(&root_dir, &target, &name,)?;

		Ok(Self { target, build_artifact, },)
	}
}

#[derive(Debug,)]
struct OsoWorkSpace {
	arch:       Architecture,
	root:       PathBuf,
	xtask_root: PathBuf,
	loader:     Crate,
	kernel:     Crate,
}

impl OsoWorkSpace {
	fn new(xtask_root: PathBuf, loader_root: PathBuf, kernel_root: PathBuf,) -> Rslt<Self,> {
		// let loader_target = target_tuple(&loader_root,)?;
		// let kernel_target = target_tuple(&kernel_root,)?;
		let root = xtask_root.parent().unwrap().to_path_buf();
		let loader = Crate::new(&loader_root,)?;
		let kernel = Crate::new(&kernel_root,)?;
		let arch = Architecture::try_from(&loader.target,)?;

		Ok(Self { arch, root, xtask_root, loader, kernel, },)
	}

	fn post_process(&self,) -> Rslt<(),> {
		// xtaskクレートのプロジェクトルートに戻る
		env::set_current_dir(&self.xtask_root,)?;

		// パスの用意
		let mount_point = self.mount_point();
		let img_path = self.img_path();

		// 前回実行時のものを削除
		Command::new("rm",).arg("-rf",).args([&mount_point, &img_path,],).run()?;

		// raw disk imageを作成
		let mut create_img = Command::new("qemu-img",);
		create_img.args(["create", "-f", "raw",],);

		match &self.arch {
			Architecture::X86_64 => create_img.arg(&img_path,).arg("200m",),
			Architecture::Aarch64 => todo!(),
		};
		create_img.run()?;

		// disk.imgをフォーマット
		Command::new("mkfs.fat",)
			.args(["-n", "'OSO'", "-s", "2", "-f", "2", "-R", "32", "-F", "32",],)
			.arg(&img_path,)
			.run()?;

		// マウントポイント作成 & マウント
		Command::new("mkdir",).arg("-p",).arg(&mount_point,).run()?;
		let mounted_disk = Command::new("hdiutil",)
			.args(["attach", "-imagekey", "diskimage-class=CRawDiskImage", "-nomount",],)
			.arg(&img_path,)
			.run()?;
		Command::new("mount",)
			.args(["-t", "msdos",],)
			.arg(&mounted_disk,)
			.arg(&mount_point,)
			.run()?;

		// bootloader, kernelを配置
		Command::new("mkdir",).arg("-p",).arg(mount_point.join("efi/boot",),).run()?;
		match &self.arch {
			Architecture::X86_64 => {
				Command::new("cp",)
					.args([
						self.root.join(&self.loader.build_artifact,),
						mount_point.join("efi/boot/bootx64.efi",),
					],)
					.run()?;
				Command::new("cp",)
					.args([
						self.root.join(&self.kernel.build_artifact,),
						mount_point.join("oso_kernel.elf",),
					],)
					.run()?;
			},
			Architecture::Aarch64 => todo!(),
		}

		// unmount
		Command::new("hdiutil",).args(["detach", &mounted_disk,],).run()?;
		Ok((),)
	}

	fn qemu(&self,) -> String {
		format!("qemu-system-{}", self.arch.to_string())
	}

	fn qemu_args(&self,) -> Vec<String,> {
		match &self.arch {
			Architecture::X86_64 => vec![
				"-vga".to_string(),
				"virtio".to_string(),
				"-drive".to_string(),
				format!(
					"if=pflash,file={},format=raw,readonly=on",
					self.xtask_root.join("assets/OVMF_CODE.fd").display()
				),
				"-drive".to_string(),
				format!(
					"if=pflash,file={},format=raw",
					self.xtask_root.join("assets/OVMF_VARS.fd").display()
				),
				"-hda".to_string(),
				format!("{}", self.img_path().display(),),
				"-monitor".to_string(),
				"stdio".to_string(),
			],
			Architecture::Aarch64 => todo!(),
		}
	}

	fn mount_point(&self,) -> PathBuf {
		self.xtask_root.join("assets/mnt",)
	}

	fn img_path(&self,) -> PathBuf {
		self.xtask_root.join("assets/disk.img",)
	}
}

fn de_toml(path: &Path,) -> Rslt<Table,> {
	let toml_str = fs::read_to_string(&path,)?;
	let table = toml_str.parse::<Table>()?;
	Ok(table,)
}

// fn target_arch(crate_root: &Path,) -> Rslt<Architecture,> {
// 	let target = target_tuple(crate_root,)?;
//
// 	Architecture::try_from(&target,)
// }

fn target_tuple(crate_root: &Path,) -> Rslt<String,> {
	let config_toml = de_toml(&crate_root.join(".cargo/config.toml",),)?;
	let toml::Value::String(target,) = config_toml["build"]["target"].clone() else {
		return Err(anyhow!("mismatch toml type:\n\tconfig.toml is: {config_toml:#?}"),);
	};
	Ok(target,)
}

fn executable_location(crate_root: &Path, target: &String, crate_name: &String,) -> Rslt<PathBuf,> {
	let out = if target.contains(".json",) {
		let file = fs::File::open(&crate_root.join(target,),)?;
		let reader = BufReader::new(file,);
		let json: serde_json::Value = serde_json::from_reader(reader,)?;
		let serde_json::Value::Array(opts,) = &json["post-link-args"]["ld.lld"] else {
			panic!("your {target}[\"post-link-args\"][\"ld.lld\"] is not array that must be array");
		};
		let out = opts
			.iter()
			.find_map(|v| {
				let opt = v.as_str().unwrap();
				if &opt[..2] == "-o" { Some(&opt[2..],) } else { None }
			},)
			.expect(&format!(
				"you need to specify name of build artifact explicitly in \
				 {target}[\"post-link-args\"][\"ld.lld\"]",
			),);
		out.to_string()
	} else if target.contains("uefi",) {
		format!("target/{target}/debug/{crate_name}.efi")
	} else {
		format!("target/{target}/debug/{crate_name}")
	};

	Ok(PathBuf::from_str(&out,)?,)
}

fn main() -> Rslt<(),> {
	let xtask_root = env::var("CARGO_MANIFEST_DIR",).unwrap_or_else(|e| {
		eprintln!("error of getting `CARGO_MANIFEST_DIR`: {e}");
		env!("CARGO_MANIFEST_DIR").to_string()
	},);
	let xtask_root = std::path::Path::new(&xtask_root,);

	let mut crate_pathes = vec![];
	// カーネルとブートローダーをビルド
	for crate_name in [KERNEL, LOADER,] {
		let path = xtask_root.parent().unwrap().join(crate_name,);
		env::set_current_dir(&path,)?;
		Command::new("cargo",).arg("b",).run()?;
		crate_pathes.push(path,);
	}

	let oso = OsoWorkSpace::new(
		xtask_root.to_path_buf(),
		crate_pathes.pop().unwrap(),
		crate_pathes.pop().unwrap(),
	)?;

	// qemunのコマンド自体とオプションを決定
	let qemu = oso.qemu();
	let qemu_args = oso.qemu_args();

	// 実行
	oso.post_process()?;
	let _qemu = Command::new(qemu,)
		.args(qemu_args,)
		.stdout(Stdio::piped(),)
		.stderr(Stdio::piped(),)
		.stdin(Stdio::piped(),)
		.output()?;

	Ok((),)
}
