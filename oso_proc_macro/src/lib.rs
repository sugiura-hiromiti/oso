extern crate proc_macro;

use colored::Colorize;
use helper::impl_init::Types;
use helper::impl_init::implement;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod helper;

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

#[proc_macro]
pub fn impl_int(types: TokenStream,) -> TokenStream {
	let types = parse_macro_input!(types as Types);
	let integers = types.iter().map(implement,);
	quote::quote! {
		#(#integers)*
	}
	.into()
}

#[proc_macro_attribute]
pub fn gen_wrapper_fn(_attr: TokenStream, item: TokenStream,) -> TokenStream {
	todo!()
}
