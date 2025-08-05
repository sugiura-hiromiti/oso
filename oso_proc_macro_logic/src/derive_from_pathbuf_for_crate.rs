use anyhow::Result as Rslt;
use itertools::Itertools;
use oso_dev_util_helper::fs::all_crates;

pub fn enum_impl(item: syn::ItemEnum,) -> Rslt<proc_macro2::TokenStream,> {
	let crate_list = all_crates()?;
	let crate_list = crate_list.iter().map(|pb| {
		let crate_name = pb
			.file_name()
			.expect(&format!("invalid path: {pb:?}"),)
			.to_str()
			.expect("path is incompatible with utf-8",);
		let camel_cased =
			crate_name.split('_',).map(|s| s[..1].to_uppercase() + &s[1..],).join("",);
		let path_str = pb.to_str().unwrap();
		let variant = quote::format_ident!("Self::{camel_cased}");
		quote::quote! {
			#path_str => #variant
		}
	},);
	let ident = item.ident;

	Ok(quote::quote! {
	impl From<PathBuf,> for #ident {
		fn from(value: PathBuf,) -> Self {
			let value = value.to_str().unwrap();
			match value {
				#(#crate_list)*,
			}
		}
	}
	},)
}

pub fn struct_impl(item: syn::ItemStruct,) -> Rslt<proc_macro2::TokenStream,> {
	todo!()
}
