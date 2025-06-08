#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use colored::Colorize;
use oso_proc_macro_logic as macro_logic;
use proc_macro::Diagnostic;
use proc_macro::Level;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::LitFloat;
use syn::parse_macro_input;

mod helper;

#[proc_macro]
/// # Params
///
/// this macro takes relative path to project root as an argument
/// specify path to font data
pub fn fonts_data(path: TokenStream,) -> TokenStream {
	use macro_logic::fonts_data::convert_bitfield;
	use macro_logic::fonts_data::fonts;

	let specified_path = &syn::parse_macro_input!(path as syn::LitStr);
	let fonts = fonts(specified_path,);
	let fonts = convert_bitfield(&fonts,);

	TokenStream::from(quote::quote! {
		&[#(#fonts),*]
	},)
}

#[proc_macro]
pub fn impl_int(types: TokenStream,) -> TokenStream {
	use macro_logic::impl_init::Types;
	use macro_logic::impl_init::implement;

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
			let method_args = macro_logic::gen_wrapper_fn::method_args(&sig);
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
#[proc_macro]
pub fn status_from_spec(version: TokenStream,) -> TokenStream {
	let syn::Lit::Float(f,) = parse_macro_input!(version as syn::Lit) else {
		panic!("version have to be floating point literal")
	};
	let status_spec_url = format!("https://uefi.org/specs/UEFI/{f}/Apx_D_Status_Codes.html");

	let spec_page = match macro_logic::status_from_spec::status_spec_page(&status_spec_url,) {
		Ok(sc,) => sc,
		Err(e,) => {
			panic!("{}\n{e}", "failed to get statuscode info from specification web page".red())
		},
	};

	let c_enum_impl = helper::impl_status(&spec_page,);

	let enum_def = quote::quote! {
			#[repr(transparent)]
			#[derive(Eq, PartialEq, Clone, Debug,)]
			pub struct Status(pub usize);

			#c_enum_impl
	};

	enum_def.into()
}

/// test ElfHeader::parse()
#[proc_macro]
pub fn test_elf_header_parse(header: TokenStream,) -> TokenStream {
	let answer = helper::elf_header_info();

	Diagnostic::new(Level::Note, format!("{answer:#?}"),).emit();
	let header = proc_macro2::TokenStream::from(header,);
	let rslt = quote::quote! {#header};

	quote::quote! {
		if cfg!(debug_assertions) {
			assert_eq!(#answer, #rslt);
		}
	}
	.into()
}

#[proc_macro]
pub fn test_program_headers_parse(program_headers: TokenStream,) -> TokenStream {
	let answer = helper::program_headers_info();
	todo!()
}
