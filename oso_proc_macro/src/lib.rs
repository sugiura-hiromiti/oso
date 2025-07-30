//! # OSO Procedural Macros
//!
//! This crate provides procedural macros for the OSO operating system project.
//! It includes macros for font data processing, integer type implementations,
//! wrapper function generation, UEFI status code generation, and ELF parsing utilities.
//!
//! ## Features
//!
//! - **Font Data Processing**: Convert font files to embedded data structures
//! - **Integer Implementation**: Generate implementations for integer types
//! - **Wrapper Functions**: Generate wrapper functions for traits
//! - **UEFI Status Codes**: Generate status code enums from UEFI specifications
//! - **ELF Testing**: Utilities for testing ELF header and program header parsing
//!
//! ## Usage
//!
//! This crate is designed to be used as a procedural macro dependency in OSO kernel
//! and related projects. The macros are compile-time code generators that help
//! reduce boilerplate and ensure consistency across the codebase.

#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use colored::Colorize;
use oso_proc_macro_logic as macro_logic;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod helper;

/// Generates embedded font data from font files at compile time.
///
/// This procedural macro takes a relative path to the project root and processes
/// font files to generate embedded data structures that can be used at runtime.
/// The macro converts font data into bitfield representations for efficient storage.
///
/// # Parameters
///
/// * `path` - A string literal containing the relative path from the project root to the directory
///   containing font data files
///
/// # Returns
///
/// Returns a token stream representing an array slice of processed font data.
/// The generated code will be in the form `&[font_data_1, font_data_2, ...]`.
///
/// # Examples
///
/// ```rust,ignore
/// // Generate font data from files in the "assets/fonts" directory
/// let fonts = fonts_data!("assets/fonts");
/// ```
///
/// # Panics
///
/// This macro will cause a compile-time error if:
/// - The specified path does not exist
/// - Font files in the path cannot be processed
/// - The path parameter is not a valid string literal
#[proc_macro]
pub fn fonts_data(path: TokenStream,) -> TokenStream {
	use macro_logic::fonts_data::convert_bitfield;
	use macro_logic::fonts_data::fonts;

	// Parse the input path as a string literal
	let specified_path = &syn::parse_macro_input!(path as syn::LitStr);

	// Load and process font files from the specified path
	let fonts = fonts(specified_path,);

	// Convert font data to bitfield representation for efficient storage
	let fonts = convert_bitfield(&fonts,);

	// Generate the final token stream as an array slice
	TokenStream::from(quote::quote! {
		&[#(#fonts),*]
	},)
}

/// Generates implementations for integer types.
///
/// This procedural macro takes a list of types and generates implementations
/// for them using the logic defined in the `oso_proc_macro_logic::impl_init` module.
/// It's typically used to reduce boilerplate when implementing common traits
/// or methods for multiple integer types.
///
/// # Parameters
///
/// * `types` - A token stream representing the types to implement. The format should match the
///   `Types` parser in the logic module.
///
/// # Returns
///
/// Returns a token stream containing the generated implementations for all
/// specified types.
///
/// # Examples
///
/// ```rust,ignore
/// // Generate implementations for u8, u16, u32, u64
/// impl_int!(u8, u16, u32, u64);
/// ```
///
/// # Panics
///
/// This macro will cause a compile-time error if:
/// - The input cannot be parsed as valid types
/// - The implementation logic fails for any of the specified types
#[proc_macro]
pub fn impl_int(types: TokenStream,) -> TokenStream {
	use macro_logic::impl_init::Types;
	use macro_logic::impl_init::implement;

	// Parse the input token stream as a list of types
	let types = parse_macro_input!(types as Types);

	// Generate implementations for each type using the logic module
	let integers = types.iter().map(implement,);

	// Combine all implementations into a single token stream
	quote::quote! {
		#(#integers)*
	}
	.into()
}

/// Generates wrapper functions for trait methods.
///
/// This attribute macro takes a trait definition and generates corresponding
/// wrapper functions that delegate to a static instance. This is useful for
/// creating global function interfaces that wrap trait implementations.
///
/// # Parameters
///
/// * `attr` - The identifier of the static frame buffer or instance to delegate to
/// * `item` - The trait definition to generate wrappers for
///
/// # Returns
///
/// Returns the original trait definition along with generated wrapper functions.
/// Each trait method becomes a standalone function that calls the corresponding
/// method on the specified static instance.
///
/// # Generated Code
///
/// For each trait method, generates a function with:
/// - Same signature as the trait method (excluding `self` parameter)
/// - Same visibility, safety, async, const, and ABI attributes
/// - Delegation to the static instance method
///
/// # Examples
///
/// ```rust,ignore
/// #[gen_wrapper_fn(GLOBAL_FRAMEBUFFER)]
/// trait Display {
///     fn write_pixel(&mut self, x: u32, y: u32, color: u32);
///     fn clear(&mut self);
/// }
///
/// // Generates:
/// // pub fn write_pixel(x: u32, y: u32, color: u32) {
/// //     GLOBAL_FRAMEBUFFER.write_pixel(x, y, color)
/// // }
/// // pub fn clear() {
/// //     GLOBAL_FRAMEBUFFER.clear()
/// // }
/// ```
///
/// # Panics
///
/// This macro will cause a compile-time error if:
/// - The attribute is not a valid identifier
/// - The item is not a valid trait definition
/// - Any trait method has an unsupported signature
#[proc_macro_attribute]
pub fn gen_wrapper_fn(attr: TokenStream, item: TokenStream,) -> TokenStream {
	// Parse the attribute as the static instance identifier
	let trait_def = parse_macro_input!(item as syn::ItemTrait);
	let static_frame_buffer = parse_macro_input!(attr as syn::Ident);

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
			let method_args = macro_logic::gen_wrapper_fn::method_args(&sig);
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

	wrapper_fns.into()
}

/// Generates UEFI status code definitions from the official UEFI specification.
///
/// This procedural macro fetches status code information from the UEFI specification
/// website and generates a complete `Status` struct with associated constants and
/// error handling methods. The macro downloads and parses the specification page
/// at compile time to ensure the status codes are up-to-date and accurate.
///
/// # Parameters
///
/// * `version` - A floating-point literal specifying the UEFI specification version (e.g., `2.8`,
///   `2.9`, `2.10`)
///
/// # Returns
///
/// Returns a token stream containing:
/// - A `Status` struct with transparent representation
/// - Associated constants for all status codes (success, warning, error)
/// - Implementation of `ok_or()` method for error handling
/// - Implementation of `ok_or_with()` method for custom error handling
///
/// # Generated Structure
///
/// ```rust,ignore
/// #[repr(transparent)]
/// #[derive(Eq, PartialEq, Clone, Debug)]
/// pub struct Status(pub usize);
///
/// impl Status {
///     // Success status codes
///     pub const SUCCESS: Self = Self(0x0);
///     
///     // Warning status codes  
///     pub const WARN_UNKNOWN_GLYPH: Self = Self(0x1);
///     
///     // Error status codes
///     pub const LOAD_ERROR: Self = Self(0x8000000000000001);
///     
///     // Error handling methods
///     pub fn ok_or(self) -> Result<Self, UefiError> { ... }
///     pub fn ok_or_with<T>(self, with: impl FnOnce(Self) -> T) -> Result<T, UefiError> { ... }
/// }
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// // Generate status codes from UEFI 2.9 specification
/// status_from_spec!(2.9);
/// ```
///
/// # Network Requirements
///
/// This macro requires internet access at compile time to fetch the UEFI specification.
/// The macro will download from: `https://uefi.org/specs/UEFI/{version}/Apx_D_Status_Codes.html`
///
/// # Panics
///
/// This macro will cause a compile-time error if:
/// - The version parameter is not a floating-point literal
/// - The UEFI specification page cannot be accessed
/// - The specification page format has changed and cannot be parsed
/// - Network connectivity issues prevent downloading the specification
#[proc_macro]
pub fn status_from_spec(version: TokenStream,) -> TokenStream {
	// Parse and validate the version parameter
	let syn::Lit::Float(f,) = parse_macro_input!(version as syn::Lit) else {
		panic!("version have to be floating point literal")
	};

	// Construct the URL for the UEFI specification page
	let status_spec_url = format!("https://uefi.org/specs/UEFI/{f}/Apx_D_Status_Codes.html");

	// Fetch and parse the specification page
	let spec_page = match macro_logic::status_from_spec::status_spec_page(&status_spec_url,) {
		Ok(sc,) => sc,
		Err(e,) => {
			panic!("{}\n{e}", "failed to get statuscode info from specification web page".red())
		},
	};

	// Generate the Status struct implementation using the helper
	let c_enum_impl = helper::impl_status(&spec_page,);

	// Generate the complete Status struct with all implementations
	let enum_def = quote::quote! {
			#[repr(transparent)]
			#[derive(Eq, PartialEq, Clone, Debug,)]
			pub struct Status(pub usize);

			#c_enum_impl
	};

	enum_def.into()
}

/// Generates compile-time tests for ELF header parsing.
///
/// This procedural macro creates a compile-time assertion that validates ELF header
/// parsing by comparing the provided header data against the expected structure
/// obtained from running `readelf -h` on the target binary. The test only runs
/// in debug builds to avoid performance overhead in release builds.
///
/// # Parameters
///
/// * `header` - A token stream representing the ELF header structure to validate
///
/// # Returns
///
/// Returns a token stream containing a conditional assertion that compares the
/// provided header against the expected header information. The assertion is
/// only active in debug builds (`cfg!(debug_assertions)`).
///
/// # Generated Code
///
/// ```rust,ignore
/// if cfg!(debug_assertions) {
///     assert_eq!(expected_header_info, provided_header);
/// }
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// // Test that a parsed ELF header matches expectations
/// test_elf_header_parse!(my_elf_header);
/// ```
///
/// # Behavior
///
/// - **Debug builds**: Performs the assertion and will panic if headers don't match
/// - **Release builds**: No-op, generates no code for performance
///
/// # Dependencies
///
/// This macro relies on:
/// - `readelf` command being available in the system PATH
/// - The helper module's `elf_header_info()` function
/// - The target binary being available for analysis
///
/// # Panics
///
/// In debug builds, this macro will cause a runtime panic if:
/// - The provided header doesn't match the expected header structure
/// - The `readelf` command fails or is not available
/// - The ELF header cannot be parsed from the binary
#[proc_macro]
pub fn test_elf_header_parse(header: TokenStream,) -> TokenStream {
	// Get the expected ELF header information from readelf
	let answer = helper::elf_header_info();

	// Convert the input header to a token stream for comparison
	let header = proc_macro2::TokenStream::from(header,);
	let rslt = quote::quote! {#header};

	// Generate conditional assertion for debug builds only
	quote::quote! {
		if cfg!(debug_assertions) {
			assert_eq!(#answer, #rslt);
		}
	}
	.into()
}

/// Generates compile-time tests for ELF program headers parsing.
///
/// This procedural macro creates a compile-time assertion that validates ELF program
/// headers parsing by comparing the provided program headers data against the expected
/// structure obtained from running `readelf -l` on the target binary. Like the ELF
/// header test, this only runs in debug builds for performance reasons.
///
/// # Parameters
///
/// * `program_headers` - A token stream representing the program headers structure to validate
///
/// # Returns
///
/// Returns a token stream containing a conditional assertion that compares the
/// provided program headers against the expected program headers information.
/// The assertion is only active in debug builds (`cfg!(debug_assertions)`).
///
/// # Generated Code
///
/// ```rust,ignore
/// if cfg!(debug_assertions) {
///     assert_eq!(expected_program_headers_info, provided_program_headers);
/// }
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// // Test that parsed program headers match expectations
/// test_program_headers_parse!(my_program_headers);
/// ```
///
/// # Behavior
///
/// - **Debug builds**: Performs the assertion and will panic if headers don't match
/// - **Release builds**: No-op, generates no code for performance
///
/// # Program Header Validation
///
/// The macro validates all aspects of program headers including:
/// - Header type (LOAD, DYNAMIC, INTERP, etc.)
/// - Flags (read, write, execute permissions)
/// - File and memory offsets
/// - Virtual and physical addresses
/// - File and memory sizes
/// - Alignment requirements
///
/// # Dependencies
///
/// This macro relies on:
/// - `readelf` command being available in the system PATH
/// - The helper module's `program_headers_info()` function
/// - The target binary being available for analysis
///
/// # Panics
///
/// In debug builds, this macro will cause a runtime panic if:
/// - The provided program headers don't match the expected structure
/// - The `readelf` command fails or is not available
/// - The program headers cannot be parsed from the binary
/// - Any program header field has an unexpected value
#[proc_macro]
pub fn test_program_headers_parse(program_headers: TokenStream,) -> TokenStream {
	// Get the expected program headers information from readelf
	let answer = helper::program_headers_info();

	// Convert the input program headers to a token stream for comparison
	let rslt = proc_macro2::TokenStream::from(program_headers,);
	let rslt = quote::quote! {#rslt};

	// Generate conditional assertion for debug builds only
	quote::quote! {
		if cfg!(debug_assertions) {
			assert_eq!(#answer, #rslt);
		}
	}
	.into()
}
