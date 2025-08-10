use anyhow::Result as Rslt;
use anyhow::anyhow;
use anyhow::bail;
use itertools::Itertools;
use oso_dev_util_helper::fs::all_crates;

use crate::RsltP;

pub fn from_path_buf(item: syn::Item,) -> RsltP {
	match item {
		syn::Item::Enum(item_enum,) => enum_impl(item_enum,),
		syn::Item::Struct(item_struct,) => struct_impl(item_struct,),
		_ => bail!("expected enum or struct, found {item:?}"),
	}
}

pub fn enum_impl(item: syn::ItemEnum,) -> RsltP {
	let crate_list = all_crates()?;
	let crate_list: Vec<_,> = crate_list
		.iter()
		.map(|pb| -> Rslt<proc_macro2::TokenStream,> {
			let crate_name = pb
				.file_name()
				.ok_or(anyhow!("invalid path: {pb:?}"),)?
				.to_str()
				.ok_or(anyhow!("path is incompatible with utf-8",),)?;
			let camel_cased =
				crate_name.split('_',).map(|s| s[..1].to_uppercase() + &s[1..],).join("",);
			let path_str = pb.to_str().ok_or(anyhow!("can not convert pathbuf to str"),)?;
			let variant = quote::format_ident!("Self::{camel_cased}");
			Ok(quote::quote! {
				#path_str => #variant
			},)
		},)
		.try_collect()?;
	let ident = item.ident;

	Ok((
		quote::quote! {
		impl From<PathBuf,> for #ident {
			fn from(value: PathBuf,) -> Self {
				let value = value.to_str().unwrap();
				match value {
					#(#crate_list)*,
				}
			}
		}
		},
		vec![],
	),)
}

pub fn struct_impl(item: syn::ItemStruct,) -> RsltP {
	todo!()
}
