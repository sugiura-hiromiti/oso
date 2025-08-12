use crate::decl_manage::crate_::OsoCrate;

//  TODO: refactor to use clap
pub struct XtaskInfo {
	opts:   Opts,
	ws:     OsoCrate,
	assets: Assets,
	host:   Host,
}
