use proc_macro2::TokenTree;
use syn::TypePath;
use syn::parse::Parse;
use syn::spanned::Spanned;

pub struct Types {
	type_list: Vec<syn::Type,>,
}

impl Types {
	pub fn iter(&self,) -> std::slice::Iter<'_, syn::Type,> {
		self.type_list.iter()
	}
}

impl Parse for Types {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		let parsed = input.step(|c| {
			let mut rest = *c;
			let mut type_list = vec![];

			while let Some((tt, next,),) = rest.token_tree() {
				match tt {
					TokenTree::Ident(idnt,) => {
						let ty: syn::Type = syn::parse_quote! { #idnt };
						type_list.push(ty,);
						rest = next;
					},
					TokenTree::Punct(_,) => rest = next,
					_ => {
						return Err(syn::Error::new(
							tt.span(),
							format!("parse failed\ntoken tree is: {tt:#?}"),
						),);
					},
				};
			}
			Ok((Types { type_list, }, rest,),)
		},)?;
		Ok(parsed,)
	}
}

pub fn implement(ty: &syn::Type,) -> proc_macro2::TokenStream {
	let idnt = unwrap_primitive(ty,).unwrap();
	let digit_count = digit_count_impl();
	let nth_digit = nth_digit_impl();
	let shift_right = shift_right_impl(&idnt,);
	quote::quote! {
		impl Integer for #idnt {
			#digit_count
			#nth_digit
			#shift_right
		}
	}
}

fn unwrap_primitive(ty: &syn::Type,) -> syn::Result<syn::Ident,> {
	// extract segment as `seg` from `ty`
	let syn::Type::Path(TypePath {
		qself: None,
		path: syn::Path { leading_colon: None, segments: seg, },
	},) = ty
	else {
		return Err(syn::Error::new(ty.span(), format!("unable to unwrap type: {ty:#?}"),),);
	};

	// extract ident of type from `seg`
	let syn::PathSegment { ident: idnt, arguments: syn::PathArguments::None, } =
		seg.first().unwrap()
	else {
		return Err(syn::Error::new(
			seg.span(),
			format!("unable to unwrap path segment: {seg:#?}"),
		),);
	};

	Ok(idnt.clone(),)
}

fn digit_count_impl() -> proc_macro2::TokenStream {
	quote::quote! {
		fn digit_count(&self) -> usize {
			let mut n = self.clone();
			let mut digits = 0;
			while n != 0 {
				n = n / 10;
				digits += 1;
			}

			digits
		}
	}
}

fn nth_digit_impl() -> proc_macro2::TokenStream {
	quote::quote! {
		/// # Panic
		///
		/// when argument `n` is 0, this function will panic
		fn nth_digit(&self, n: usize) -> u8 {
			assert_ne!(n, 0);
			let mut origin = self.clone();
			for _i in 1..n {
				origin.shift_right();
			}
			(origin % 10) as u8
		}
	}
}

fn shift_right_impl(idnt: &syn::Ident,) -> proc_macro2::TokenStream {
	let return_value = if idnt.to_string().contains("u",) {
		quote::quote! {
			first_digit as u8
		}
	} else {
		quote::quote! {
			if first_digit < 0 {
				-first_digit as u8
			} else {
				first_digit as u8
			}
		}
	};

	quote::quote! {
		fn shift_right(&mut self) -> u8 {
			let first_digit = *self % 10;
			*self = *self / 10;
			#return_value
		}
	}
}
