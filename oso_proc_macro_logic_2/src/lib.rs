#![feature(exit_status_error)]

// pub mod cli; // Will be added in feat/add-cli-module branch

use anyhow::Result as Rslt;

pub fn derive_for_enum(item: syn::ItemEnum,) -> proc_macro2::TokenStream {
	todo!()
}
pub fn derive_for_struct(item: syn::ItemStruct,) -> proc_macro2::TokenStream {
	todo!()
}
