//! same role as oso_proc_macro
//! just for avoiding cyclic dependencies(oso_dev_util -> oso_proc_macro -> oso_proc_macro_logic ->
//! oso_dev_util)

#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod helper;

use proc_macro::Diagnostic;
use proc_macro::Level;
use proc_macro::TokenStream;

#[proc_macro_derive(FromPathBuf)]
pub fn derive_from_pathbuf_for_crate_xxx(item: TokenStream,) -> TokenStream {
	let item = syn::parse_macro_input!(item as syn::Item);
	let rslt = helper::derive_from_pathbuf_for_crate_xxx_helper(item,);
	Diagnostic::new(Level::Error, rslt.to_string(),).emit();
	rslt.into()
}
