extern crate proc_macro;

use colored::Colorize;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::TypePath;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::spanned::Spanned;

//const FONT_DATA_SIZE: u16 = 256;
const CHARACTER_COUNT: usize = 256;

#[proc_macro]
pub fn fonts_data(path: TokenStream,) -> TokenStream {
	let path = &syn::parse_macro_input!(path as syn::LitStr);
	let path = format!("/Users/a/Downloads/QwQ/oso/oso_proc_macro/{}", path.value());
	let font_data = std::fs::read_to_string(&path,).expect(&format!(
		"{}: {}\n",
		"failed to open font file".bold().red(),
		path
	),);

	let fonts_data_lines: Vec<&str,> = font_data
		.split("\n",)
		.collect::<Vec<&str,>>()
		.into_iter()
		.filter(|s| !(*s == "" || s.contains("0x",)),)
		.collect();

	let mut fonts = vec!["".to_string(); CHARACTER_COUNT];
	for idx in 0..CHARACTER_COUNT {
		fonts[idx] = fonts_data_lines[idx * 16..(idx + 1) * 16].join("",);
	}

	fonts.iter().for_each(|s| assert_eq!(s.len(), 128),);

	let fonts: Vec<u128,> = fonts
		.into_iter()
		.map(|s| {
			let lines = s.split("\n",).collect::<Vec<&str,>>();
			let a: u128 = lines
				.into_iter()
				.enumerate()
				.map(|(i, s,)| {
					let s = s.replace(".", "0",).replace("@", "1",);
					let s: String = s.chars().rev().collect();
					let line = u128::from_str_radix(&s, 2,).unwrap();
					line << i
				},)
				.sum();
			a
		},)
		.collect();

	let a = fonts[b'a' as usize];
	let mut s = String::new();
	for j in 0..16 {
		for i in 0..8 {
			let flag = i + j * 8;
			let bit = a & (0b1 << flag);
			if bit != 0 {
				s.push('W',);
			} else {
				s.push(' ',);
			}
		}
		s.push('\n',);
	}

	TokenStream::from(quote::quote! {
		&[#(#fonts),*]
	},)
}

struct Types {
	type_list: Vec<syn::Type,>,
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

fn implement(ty: &syn::Type,) -> proc_macro2::TokenStream {
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

#[proc_macro]
pub fn impl_int(types: TokenStream,) -> TokenStream {
	let types = parse_macro_input!(types as Types);
	let integers = types.type_list.iter().map(implement,);
	quote::quote! {
		#(#integers)*
	}
	.into()
}
