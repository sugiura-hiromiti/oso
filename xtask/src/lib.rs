pub mod builder;
pub mod qemu;
pub mod shell;
pub mod workspace;

pub struct XtaskInfo {
	opts:   Opts,
	ws:     OsoCrate,
	assets: Assets,
	host:   Target,
}
