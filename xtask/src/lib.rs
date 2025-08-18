#![feature(string_from_utf8_lossy_owned)]
#![feature(exit_status_error)]

use oso_dev_util::cargo::Assets;
use oso_dev_util::cargo::Opts;
use oso_dev_util::decl_manage::crate_::OsoCrate;

pub mod builder;
pub mod qemu;
pub mod shell;
pub mod workspace;

pub struct XtaskInfo {
	opts:   Opts,
	ws:     OsoCrate,
	assets: Assets,
}
