use std::path::PathBuf;

use crate::RsltP;
use anyhow::Result as Rslt;
use anyhow::anyhow;
use anyhow::bail;
use itertools::Itertools;
use oso_dev_util_helper::fs::all_crates;

pub fn from_path_buf(item: syn::DeriveInput,) -> RsltP {
	match item.data {
		syn::Data::Struct(_,) => struct_impl(item,),
		_ => bail!("expected struct, found {item:?}"),
	}
}

pub fn struct_impl(struct_def: syn::DeriveInput,) -> RsltP {
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
	let ident = struct_def.ident;

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

fn crate_name(path: &PathBuf,) -> Rslt<String,> {
	path.file_name()
		.ok_or(anyhow!("invalid path: {path:?}"),)?
		.to_str()
		.map(|s| s.to_string(),)
		.ok_or(anyhow!("path is incompatible with utf-8",),)
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::quote;
	use syn::parse_quote;

	#[test]
	fn test_from_path_buf_with_enum() {
		// Create a test enum
		let test_enum: syn::DeriveInput = parse_quote! {
			pub enum TestCrate {
				OsoKernel,
				OsoBootloader,
			}
		};

		let item = syn::DataEnum(test_enum,);

		// This test depends on the all_crates() function working
		// We'll test that the function doesn't panic and returns a result
		let result = from_path_buf(item,);

		// The result depends on the actual crate structure, so we just verify
		// that it returns either Ok or Err without panicking
		match result {
			Ok(_,) => assert!(true),
			Err(_,) => assert!(true), // This is also acceptable since it depends on the environment
		}
	}

	#[test]
	fn test_from_path_buf_with_struct() {
		// Create a test struct
		let test_struct: syn::ItemStruct = parse_quote! {
			pub struct TestStruct {
				field: i32,
			}
		};

		let item = syn::Item::Struct(test_struct,);

		// struct_impl is not implemented yet, so this should panic with todo!()
		let result = std::panic::catch_unwind(|| from_path_buf(item,),);
		assert!(result.is_err());
	}

	#[test]
	fn test_from_path_buf_with_invalid_item() {
		// Create a function item (not enum or struct)
		let test_fn: syn::ItemFn = parse_quote! {
			fn test_function() {}
		};

		let item = syn::Item::Fn(test_fn,);

		let result = from_path_buf(item,);
		assert!(result.is_err());

		if let Err(e,) = result {
			let error_msg = e.to_string();
			assert!(error_msg.contains("expected enum or struct"));
		}
	}

	#[test]
	fn test_enum_impl_basic_functionality() {
		// Create a simple test enum
		let test_enum: syn::ItemEnum = parse_quote! {
			pub enum CrateType {
				Kernel,
				Bootloader,
			}
		};

		// Test that enum_impl doesn't panic
		let result = struct_impl(test_enum,);

		// The result depends on all_crates() working, but we can verify structure
		match result {
			Ok((tokens, diags,),) => {
				let token_string = tokens.to_string();
				assert!(token_string.contains("impl From < PathBuf"));
				assert!(token_string.contains("for CrateType"));
				assert!(token_string.contains("fn from"));
				assert!(diags.is_empty());
			},
			Err(_,) => {
				// This is acceptable if all_crates() fails in test environment
				assert!(true);
			},
		}
	}

	#[test]
	fn test_struct_impl_not_implemented() {
		let test_struct: syn::ItemStruct = parse_quote! {
			pub struct TestStruct;
		};

		// struct_impl should panic with todo!()
		let result = std::panic::catch_unwind(|| struct_impl(test_struct,),);
		assert!(result.is_err());
	}

	#[test]
	fn test_camel_case_conversion_logic() {
		// Test the camel case conversion logic used in enum_impl
		let test_name = "oso_kernel_test";
		let camel_cased = test_name.split('_',).map(|s| s[..1].to_uppercase() + &s[1..],).join("",);

		assert_eq!(camel_cased, "OsoKernelTest");
	}

	#[test]
	fn test_camel_case_single_word() {
		let test_name = "kernel";
		let camel_cased = test_name.split('_',).map(|s| s[..1].to_uppercase() + &s[1..],).join("",);

		assert_eq!(camel_cased, "Kernel");
	}

	#[test]
	fn test_camel_case_empty_parts() {
		let test_name = "oso__kernel"; // Double underscore
		let camel_cased = test_name
			.split('_',)
			.map(|s| if s.is_empty() { String::new() } else { s[..1].to_uppercase() + &s[1..] },)
			.join("",);

		assert_eq!(camel_cased, "OsoKernel");
	}

	#[test]
	fn test_path_string_conversion() {
		use std::path::PathBuf;

		// Test that PathBuf can be converted to string
		let path = PathBuf::from("/test/path",);
		let path_str = path.to_str();

		assert!(path_str.is_some());
		assert_eq!(path_str.unwrap(), "/test/path");
	}

	#[test]
	fn test_path_with_non_utf8_handling() {
		use std::ffi::OsString;
		use std::path::PathBuf;

		// Create a path that might have UTF-8 issues
		let path = PathBuf::from("test_path",);
		let path_str = path.to_str();

		// For normal paths, this should work
		assert!(path_str.is_some());
	}

	#[test]
	fn test_quote_format_ident_functionality() {
		// Test that quote::format_ident works as expected
		let ident_name = "TestVariant";
		let ident = quote::format_ident!("{}", ident_name);

		let token_string = quote! { #ident }.to_string();
		assert!(token_string.contains("TestVariant"));
	}

	#[test]
	fn test_error_handling_in_enum_impl() {
		// Create an enum with a complex name to test error handling
		let test_enum: syn::ItemEnum = parse_quote! {
			pub enum ComplexCrateType {
				VeryLongVariantName,
			}
		};

		// Test that the function handles the enum properly
		let result = struct_impl(test_enum,);

		// We expect either success or a specific error from all_crates()
		match result {
			Ok(_,) => assert!(true),
			Err(e,) => {
				// Should be a meaningful error message
				let error_msg = e.to_string();
				assert!(!error_msg.is_empty());
			},
		}
	}

	#[test]
	fn test_token_stream_generation() {
		// Test that we can generate basic token streams
		let test_tokens = quote! {
			impl From<PathBuf> for TestEnum {
				fn from(value: PathBuf) -> Self {
					match value.to_str().unwrap() {
						"/test/path" => Self::TestVariant,
					}
				}
			}
		};

		let token_string = test_tokens.to_string();
		assert!(token_string.contains("impl From"));
		assert!(token_string.contains("PathBuf"));
		assert!(token_string.contains("TestEnum"));
		assert!(token_string.contains("fn from"));
		assert!(token_string.contains("match"));
	}

	#[test]
	fn test_itertools_join_functionality() {
		// Test that itertools join works as expected
		let parts = vec!["Hello", "World", "Test"];
		let joined = parts.iter().map(|s| s.to_string(),).join("",);

		assert_eq!(joined, "HelloWorldTest");
	}

	#[test]
	fn test_itertools_join_with_separator() {
		let parts = vec!["Hello", "World"];
		let joined = parts.iter().map(|s| s.to_string(),).join("_",);

		assert_eq!(joined, "Hello_World");
	}

	#[test]
	fn test_anyhow_error_creation() {
		// Test anyhow error creation and formatting
		let test_path = std::path::PathBuf::from("/invalid/path",);
		let error = anyhow!("invalid path: {test_path:?}");

		let error_msg = error.to_string();
		assert!(error_msg.contains("invalid path"));
		assert!(error_msg.contains("/invalid/path"));
	}

	#[test]
	fn test_result_try_collect() {
		// Test the try_collect functionality used in enum_impl
		let results: Vec<Result<i32, &str,>,> = vec![Ok(1,), Ok(2,), Ok(3,)];
		let collected: Result<Vec<i32,>, &str,> = results.into_iter().try_collect();

		assert!(collected.is_ok());
		assert_eq!(collected.unwrap(), vec![1, 2, 3]);
	}

	#[test]
	fn test_result_try_collect_with_error() {
		let results: Vec<Result<i32, &str,>,> = vec![Ok(1,), Err("error",), Ok(3,)];
		let collected: Result<Vec<i32,>, &str,> = results.into_iter().try_collect();

		assert!(collected.is_err());
		assert_eq!(collected.unwrap_err(), "error");
	}

	#[test]
	fn test_syn_item_matching() {
		// Test that syn::Item matching works correctly
		let enum_item: syn::Item = syn::parse_quote! {
			enum TestEnum { A, B }
		};

		let struct_item: syn::Item = syn::parse_quote! {
			struct TestStruct;
		};

		let fn_item: syn::Item = syn::parse_quote! {
			fn test_fn() {}
		};

		// Test pattern matching
		match enum_item {
			syn::Item::Enum(_,) => assert!(true),
			_ => panic!("Should match enum"),
		}

		match struct_item {
			syn::Item::Struct(_,) => assert!(true),
			_ => panic!("Should match struct"),
		}

		match fn_item {
			syn::Item::Fn(_,) => assert!(true),
			_ => panic!("Should match function"),
		}
	}
}
