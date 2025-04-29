use oso_proc_macro_logic::status_from_spec::StatusCode;
use oso_proc_macro_logic::status_from_spec::StatusCodeInfo;
use proc_macro2::Span;

pub fn impl_status(spec_page: &StatusCode,) -> proc_macro2::TokenStream {
	let (success_match, success_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.success.token_parts(false,).into_iter().unzip();
	let (warn_match, warn_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.warn.token_parts(false,).into_iter().unzip();
	let (error_match, error_assoc,): (Vec<_,>, Vec<_,>,) =
		spec_page.error.token_parts(true,).into_iter().unzip();

	quote::quote! {
		impl Status {
			#(#success_assoc)*
			#(#warn_assoc)*
			#(#error_assoc)*

			pub fn ok_or(self) -> Rslt<Self,> {
				use alloc::string::ToString;
				match self {
					#(#success_match)*
					#(#warn_match)*
					#(#error_match)*
					Self(code) => Err(OsoLoaderError::Uefi("vendor custom error code".to_string())),
				}
			}

			pub fn ok_or_with<T>(self, with: impl FnOnce(Self)->T) -> Rslt<T> {
				let status = self.ok_or()?;
				Ok(with(status))
			}
		}
	}
}

trait TokenParts {
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),>;
}

impl TokenParts for Vec<StatusCodeInfo,> {
	fn token_parts(
		&self,
		is_err: bool,
	) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream,),> {
		self.iter()
			.map(|sci| {
				let mnemonic = syn::Ident::new(&sci.mnemonic, Span::call_site(),);
				let value =
					syn::Lit::Int(syn::LitInt::new(&format!("{}", sci.value), Span::call_site(),),);
				let match_arms =
					if is_err { err_match(&mnemonic, &sci.desc,) } else { ok_match(&mnemonic,) };
				let assoc = assoc_const(&mnemonic, &value, &sci.desc,);
				(match_arms, assoc,)
			},)
			.collect()
	}
}

fn ok_match(mnemonic: &syn::Ident,) -> proc_macro2::TokenStream {
	quote::quote! {
		Self::#mnemonic => Ok(Self::#mnemonic,),
	}
}

fn err_match(mnemonic: &syn::Ident, msg: &String,) -> proc_macro2::TokenStream {
	let mnemonic_str = mnemonic.to_string();
	quote::quote! {
	Self::#mnemonic => {
		let mut mnemonic = #mnemonic_str.to_string();
		mnemonic.push_str(": ");
		mnemonic.push_str(#msg);
		Err(OsoLoaderError::Uefi(mnemonic))
	},
	}
}

fn assoc_const(mnemonic: &syn::Ident, value: &syn::Lit, msg: &String,) -> proc_macro2::TokenStream {
	quote::quote! {
		#[doc = #msg]
		pub const #mnemonic: Self = Self(#value);
	}
}
