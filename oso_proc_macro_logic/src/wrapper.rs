//! # Function Wrapper Generation Utilities
//!
//! This module provides utilities for generating wrapper functions and extracting
//! method arguments from function signatures. It's primarily used in procedural
//! macros that need to analyze and transform function definitions.

use crate::RsltP;
use syn::Signature;

pub fn wrapper(static_frame_buffer: syn::Ident, trait_def: syn::ItemTrait,) -> RsltP {
	// Generate wrapper functions for each trait method
	let wrapper_fns = trait_def.items.clone().into_iter().filter_map(|i| {
		if let syn::TraitItem::Fn(method,) = i {
			let sig = method.sig;

			// Extract function signature components
			let constness = sig.constness;
			let asyncness = sig.asyncness;
			let unsafety = sig.unsafety;
			let abi = &sig.abi;
			let fn_name = &sig.ident;
			let generics = &sig.generics;

			// Filter out 'self' parameters for the wrapper function
			let fn_params = sig.inputs.iter().filter(|a| matches!(a, &&syn::FnArg::Typed(_)),);

			// Generate method arguments for the delegation call
			let method_args = method_args(&sig);
			let variadic = &sig.variadic;
			let output = &sig.output;

			// Generate the wrapper function declaration
			let decl = quote::quote! {
				pub #unsafety #asyncness #constness #abi fn #fn_name #generics(#(#fn_params),* #variadic) #output {
					#static_frame_buffer.#fn_name(#(#method_args),*)
				}
			};
			Some(decl,)
		} else {
			// Skip non-function trait items
			None
		}
	},);

	// Combine wrapper functions with the original trait definition
	let wrapper_fns = quote::quote! {
		#(#wrapper_fns)*
		#trait_def
	};
	Ok((wrapper_fns, vec![],),)
}

/// Extracts method arguments from a function signature, excluding the receiver (`self`)
///
/// This function analyzes a function signature and returns an iterator over all
/// the argument patterns, filtering out any receiver arguments (like `self`, `&self`,
/// `&mut self`, etc.). This is useful when generating wrapper functions or when you
/// need to forward arguments to another function.
///
/// # Arguments
///
/// * `sig` - A reference to a `syn::Signature` representing the function signature to analyze
///
/// # Returns
///
/// An iterator that yields `Box<syn::Pat>` for each non-receiver argument in the signature.
/// The patterns represent the argument names and destructuring patterns.
///
/// # Examples
///
/// ```ignore
/// use syn::{parse_quote, Signature};
///
/// let sig: Signature = parse_quote! {
///     fn example(&self, arg1: i32, arg2: String) -> bool
/// };
///
/// let args: Vec<_> = method_args(&sig).collect();
/// assert_eq!(args.len(), 2); // Only arg1 and arg2, self is filtered out
/// ```
///
/// # Use Cases
///
/// - Generating wrapper functions that need to forward arguments
/// - Creating proxy methods that delegate to other implementations
/// - Analyzing function signatures in procedural macros
/// - Building function call expressions with the same arguments
pub fn method_args(sig: &Signature,) -> impl Iterator<Item = std::boxed::Box<syn::Pat,>,> {
	sig.inputs.iter().filter_map(|a| match a {
		// Skip receiver arguments (self, &self, &mut self, etc.)
		syn::FnArg::Receiver(_,) => None,

		// Extract the pattern from typed arguments
		syn::FnArg::Typed(pty,) => Some(pty.pat.clone(),),
	},)
}
#[cfg(test)]
mod tests {
	use super::*;
	use syn::Signature;
	use syn::parse_quote;

	#[test]
	fn test_method_args_no_receiver() {
		let sig: Signature = parse_quote! {
			fn test_function(arg1: i32, arg2: String, arg3: bool) -> i32
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		assert_eq!(args.len(), 3);
	}

	#[test]
	fn test_method_args_with_self_receiver() {
		let sig: Signature = parse_quote! {
			fn test_method(&self, arg1: i32, arg2: String) -> bool
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &self, only return the 2 typed arguments
		assert_eq!(args.len(), 2);
	}

	#[test]
	fn test_method_args_with_mut_self_receiver() {
		let sig: Signature = parse_quote! {
			fn test_method(&mut self, arg1: i32) -> ()
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &mut self, only return the 1 typed argument
		assert_eq!(args.len(), 1);
	}

	#[test]
	fn test_method_args_with_owned_self_receiver() {
		let sig: Signature = parse_quote! {
			fn test_method(self, arg1: String, arg2: Vec<i32>) -> String
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude self, only return the 2 typed arguments
		assert_eq!(args.len(), 2);
	}

	#[test]
	fn test_method_args_no_arguments() {
		let sig: Signature = parse_quote! {
			fn test_function() -> ()
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		assert_eq!(args.len(), 0);
	}

	#[test]
	fn test_method_args_only_receiver() {
		let sig: Signature = parse_quote! {
			fn test_method(&self) -> i32
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &self, return empty
		assert_eq!(args.len(), 0);
	}

	#[test]
	fn test_method_args_complex_types() {
		let sig: Signature = parse_quote! {
			fn complex_method(
				&self,
				arg1: Vec<String>,
				arg2: HashMap<String, i32>,
				arg3: Option<Box<dyn Trait>>,
				arg4: &mut [u8]
			) -> Result<(), Error>
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &self, return 4 typed arguments
		assert_eq!(args.len(), 4);
	}

	#[test]
	fn test_method_args_pattern_matching() {
		let sig: Signature = parse_quote! {
			fn pattern_method(&self, (x, y): (i32, i32), [a, b, c]: [u8; 3]) -> ()
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &self, return 2 pattern arguments
		assert_eq!(args.len(), 2);
	}

	#[test]
	fn test_method_args_with_lifetimes() {
		let sig: Signature = parse_quote! {
			fn lifetime_method<'a>(&self, arg1: &'a str, arg2: &'a mut Vec<i32>) -> &'a str
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &self, return 2 typed arguments with lifetimes
		assert_eq!(args.len(), 2);
	}

	#[test]
	fn test_method_args_with_generics() {
		let sig: Signature = parse_quote! {
			fn generic_method<T, U>(&mut self, arg1: T, arg2: Vec<U>) -> Option<T>
			where
				T: Clone,
				U: Debug
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		// Should exclude &mut self, return 2 generic typed arguments
		assert_eq!(args.len(), 2);
	}

	#[test]
	fn test_method_args_preserves_pattern_structure() {
		let sig: Signature = parse_quote! {
			fn destructure_method(&self, Point { x, y }: Point, mut z: i32) -> ()
		};

		let args: Vec<_,> = method_args(&sig,).collect();
		assert_eq!(args.len(), 2);

		// The patterns should be preserved as-is for code generation
		// This test ensures we don't lose the destructuring information
	}
}
