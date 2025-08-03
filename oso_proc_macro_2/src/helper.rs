use oso_proc_macro_logic_2::derive_for_enum;
use oso_proc_macro_logic_2::derive_for_struct;

pub fn derive_from_pathbuf_for_crate_xxx_helper(item: syn::Item,) -> proc_macro2::TokenStream {
	match item {
		syn::Item::Enum(item_enum,) => derive_for_enum(item_enum,),
		syn::Item::Struct(item_struct,) => derive_for_struct(item_struct,),
		_ => unreachable!(),
	}
}
