use oso_dev_util_helper::fs::all_crates;

const CWD: &str = std::env!("CARGO_MANIFEST_DIR");

pub fn derive_for_enum(item: syn::ItemEnum,) -> proc_macro2::TokenStream {
	let crate_list = all_crates();
	let ident = item.ident;
	todo!()
}
pub fn derive_for_struct(item: syn::ItemStruct,) -> proc_macro2::TokenStream {
	todo!()
}
