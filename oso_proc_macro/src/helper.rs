use oso_proc_macro_logic::status_from_spec::StatusCodeInfo;
use proc_macro2::Span;
use syn::ItemEnum;

pub fn impl_ok_or(
	enum_def: &ItemEnum,
	success: &Vec<StatusCodeInfo,>,
	warn: &Vec<StatusCodeInfo,>,
	error: &Vec<StatusCodeInfo,>,
) -> proc_macro2::TokenStream {
	let success: Vec<_,> = success
		.iter()
		.map(|sci| {
			let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);
			quote::quote! {
				Self::#mnemonic => Ok(self,)
			}
		},)
		.collect();
	let warn: Vec<_,> = warn
		.iter()
		.map(|sci| {
			let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);
			quote::quote! {
				Self::#mnemonic => Ok(self,)
			}
		},)
		.collect();
	let error: Vec<_,> = error
		.iter()
		.map(|sci| {
			let msg = &sci.desc;
			let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);
			quote::quote! {
				Self::#mnemonic => Err(OsoLoaderError::Uefi(#msg.to_string()))
			}
		},)
		.collect();

	let ident = &enum_def.ident;
	quote::quote! {
		impl #ident {
			pub fn ok_or(self) -> Rslt<Self,> {
				use alloc::string::ToString;
				match self {
					#(#success,)*
					#(#warn,)*
					#(#error,)*
				}
			}
		}
	}
}
