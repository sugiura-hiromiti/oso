#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use oso_proc_macro_logic::oso_proc_macro_helper::MacroDef;
use proc_macro::TokenStream;

#[proc_macro]
pub fn def(def: TokenStream,) -> TokenStream {
	use oso_proc_macro_logic::oso_proc_macro_helper::MacroDef;

	let macro_def = syn::parse_macro_input!(def as MacroDef);
	oso_proc_macro_logic::oso_proc_macro_helper::def(macro_def,).into()
}
