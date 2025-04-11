#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::Diagnostic;
use proc_macro::Level;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod helper;

#[proc_macro]
/// # Params
///
/// this macro takes relative path to project root as an argument
/// specify path to font data
pub fn fonts_data(path: TokenStream,) -> TokenStream {
	use helper::fonts_data::convert_bitfield;
	use helper::fonts_data::fonts;

	let specified_path = &syn::parse_macro_input!(path as syn::LitStr);
	let fonts = fonts(specified_path,);
	let fonts = convert_bitfield(&fonts,);

	TokenStream::from(quote::quote! {
		&[#(#fonts),*]
	},)
}

#[proc_macro]
pub fn impl_int(types: TokenStream,) -> TokenStream {
	use helper::impl_init::Types;
	use helper::impl_init::implement;

	let types = parse_macro_input!(types as Types);
	let integers = types.iter().map(implement,);
	quote::quote! {
		#(#integers)*
	}
	.into()
}

#[proc_macro_attribute]
pub fn gen_wrapper_fn(attr: TokenStream, item: TokenStream,) -> TokenStream {
	let trait_def = parse_macro_input!(item as syn::ItemTrait);
	let static_frame_buffer = parse_macro_input!(attr as syn::Ident);

	let wrapper_fns = trait_def.items.clone().into_iter().filter_map(|i| {
		if let syn::TraitItem::Fn(method,) = i {
			let sig = method.sig;

			// extract fn signature
			let constness = sig.constness;
			let asyncness = sig.asyncness;
			let unsafety = sig.unsafety;
			let abi = &sig.abi;
			let fn_name = &sig.ident;
			// syn::Ident::new(format!("global_{}", sig.ident).as_str(), Span::call_site(),);
			let generics = &sig.generics;
			let fn_params = sig.inputs.iter().filter(|a| matches!(a, &&syn::FnArg::Typed(_)),);
			let method_args = helper::gen_wrapper_fn::method_args(&sig);
			let variadic = &sig.variadic;
			let output = &sig.output;

			let decl = quote::quote! {
				pub #unsafety #asyncness #constness #abi fn #fn_name #generics(#(#fn_params),* #variadic) #output {
					#static_frame_buffer.#fn_name(#(#method_args),*)
				}
			};
			Some(decl,)
		} else {
			None
		}
	},);

	let wrapper_fns = quote::quote! {
		#(#wrapper_fns)*
		#trait_def
	};

	wrapper_fns.into()
}

/// attr specifies version of uefi
#[proc_macro_attribute]
pub fn status_from_spec(version: TokenStream, item: TokenStream,) -> TokenStream {
	use helper::status_from_spec;
	let syn::Lit::Float(f,) = parse_macro_input!(version as syn::Lit) else {
		panic!("version have to be floating point literal")
	};
	let status_spec_url = format!("https://uefi.org/specs/UEFI/{f}/Apx_D_Status_Codes.html");

	Diagnostic::new(Level::Note, &status_spec_url,);
	let spec_page = status_from_spec::status_spec_page(&status_spec_url,);

	todo!("{f:?}");
}
