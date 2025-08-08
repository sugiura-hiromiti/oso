#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::format_ident;

enum MacroDef {
	FnStyle { name: ParamChunk, param_item: ParamChunk, },
	Derive { name: ParamChunk, param_item: ParamChunk, param_attr: Option<Vec<ParamChunk,>,>, },
	Attr { name: ParamChunk, param_attr: ParamChunk, param_item: ParamChunk, },
}

struct ParamChunk {
	param: syn::Ident,
	colon: syn::Token![,],
}

impl syn::parse::Parse for ParamChunk {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		Ok(Self { param: input.parse()?, colon: input.parse()?, },)
	}
}

impl syn::parse::Parse for MacroDef {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		let macro_kind: ParamChunk = input.parse()?;

		let macro_kind_raw = macro_kind.param.to_string();
		let macro_kind_raw = macro_kind_raw.as_str();
		match macro_kind_raw {
			"fn_style" => Self::FnStyle { name: input.parse()?, param_item: input.parse()?, },
			"derive" => Self::Derive {
				name:       input.parse()?,
				param_item: input.parse()?,
				param_attr: input.parse_terminated()?,
			},
			"attr" => Self::Attr {
				name:       input.parse()?,
				param_attr: input.parse()?,
				param_item: input.parse()?,
			},
			_ => {
				return Err(input.error(format!(
					"expected one of fn_style, derive, attr. found {macro_kind_raw}"
				),),);
			},
		};

		// input.step(|c| match c.token_tree() {
		// 	Some((TokenTree::Ident(target,), next,),) => {
		// 		let XXXXXXXXX = match target.to_string().as_str() {
		// 			"fn_style" | "derive" | "attr" => {
		// 				ParamChunk { param: target, colon: c.t	oken_stream, }
		// 			},
		// 			a => Err(c.error(format!(
		// 				r#"expected one of "fn_style", "derive", "attr". found {a} "#
		// 			),),),
		// 		};
		// 		todo!()
		// 	},
		// 	_ => unimplemented!(),
		// },);
		todo!()
	}
}

#[proc_macro]
pub fn def(def: TokenStream,) -> TokenStream {
	todo!()
}
