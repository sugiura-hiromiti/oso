use std::env::set_current_dir;
use std::process::Command;

use crate::shell::Architecture;
use crate::shell::Opts;
use crate::shell::Run;
use crate::workspace::OsoWorkSpace;
use anyhow::Result as Rslt;

pub struct Builder {
	opts:      Opts,
	workspace: OsoWorkSpace,
}

impl Builder {
	pub fn new() -> Rslt<Self,> {
		let opts = Opts::new();
		let workspace = OsoWorkSpace::new()?;
		Ok(Self { opts, workspace, },)
	}

	pub fn build(self,) -> Rslt<(),> {
		set_current_dir(&self.workspace.root,)?;
		let is_release = self.opts.build_mode.is_release();
		self.build_xtask()?;
		self.build_loader(is_release,)?;
		self.build_kernel(is_release,)
	}

	fn build_xtask(&self,) -> Rslt<(),> {
		Command::new("cargo",).args(["b", "-p", "xtask", "-r",],).run()
	}

	fn build_loader(&self, is_release: bool,) -> Rslt<(),> {
		cargo_build(&self.workspace.loader.name, is_release,)
	}

	fn build_kernel(&self, is_release: bool,) -> Rslt<(),> {
		cargo_build(&self.workspace.kernel.name, is_release,)
	}

	pub fn run() {}
}

fn cargo_build(package: &String, is_release: bool, arch: &Architecture,) -> Rslt<(),> {
	let mut cmd = Command::new("cargo",);
	cmd.args(["b", "-p", package,],);
	if is_release {
		cmd.arg("-r",);
	}

	cmd.run()
}
