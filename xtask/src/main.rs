#![feature(string_from_utf8_lossy_owned)]
#![feature(exit_status_error)]

use crate::shell::Run;
use crate::workspace::KERNEL;
use crate::workspace::LOADER;
use crate::workspace::OsoWorkSpace;
use anyhow::Result as Rslt;
use anyhow::anyhow;
use colored::Colorize;
use std::env;
use std::process::Command;
use std::process::Stdio;

pub mod builder;
pub mod qemu;
pub mod shell;
pub mod workspace;

fn main() -> Rslt<(),> {
	let cli_args = std::env::args();
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

	let oso = OsoWorkSpace::new()?;

	// qemunのコマンド自体とオプションを決定
	let qemu = oso.qemu();
	let qemu_args = oso.qemu_args();

	// 実行
	oso.post_process()?;
	let qemu = Command::new(qemu,)
		.args(qemu_args,)
		.stdout(Stdio::inherit(),)
		.stderr(Stdio::inherit(),)
		.stdin(Stdio::inherit(),)
		.status()?;

	match qemu.exit_ok() {
		Ok(_,) => Ok((),),
		Err(e,) => Err(anyhow!("{}:\n{e}", "qemu exited unsuccessfully".red()),),
	}
}
