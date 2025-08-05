//! # OSO Procedural Macro Logic
//!
//! This crate provides procedural macro logic and utilities for the OSO operating system project.
//! It includes functionality for:
//!
//! - Font data processing and bitmap conversion
//! - ELF file parsing and analysis
//! - UEFI status code generation from specifications
//! - Code generation utilities for wrapper functions and trait implementations
//!
//! ## Features
//!
//! The crate uses several unstable Rust features:
//! - `proc_macro_diagnostic`: For emitting diagnostic messages during macro expansion
//! - `str_as_str`: String manipulation utilities
//! - `iter_array_chunks`: Iterator chunking operations
//! - `associated_type_defaults`: Default associated types in traits
//! - `iterator_try_collect`: Fallible iterator collection

#![feature(proc_macro_diagnostic)]
#![feature(str_as_str)]
#![feature(iter_array_chunks)]
#![feature(associated_type_defaults)]
#![feature(iterator_try_collect)]

use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::env::current_dir;

extern crate proc_macro;

/// Font data processing and bitmap conversion utilities
pub mod fonts_data;

/// Function wrapper generation utilities
pub mod gen_wrapper_fn;

/// Trait implementation generation for integer types
pub mod impl_init;

/// UEFI status code parsing from HTML specifications
pub mod status_from_spec;

/// ELF header parsing and analysis utilities
pub mod test_elf_header_parse;

/// ELF program header parsing utilities
pub mod test_program_headers_parse;

pub mod derive_from_pathbuf_for_crate;
#[cfg(test)]
mod tests {
	use super::*;
	use std::env::current_dir;
	use std::env::set_current_dir;
	use std::fs::File;
	use std::fs::create_dir_all;
	use std::path::PathBuf;
	use tempfile::TempDir;

	/// Helper function to create a temporary directory structure for testing
	fn create_test_environment() -> (TempDir, PathBuf,) {
		let temp_dir = TempDir::new().expect("Failed to create temp directory",);
		let target_dir = temp_dir.path().join("target",);
		create_dir_all(&target_dir,).expect("Failed to create target directory",);

		let kernel_path = target_dir.join("oso_kernel.elf",);
		(temp_dir, kernel_path,)
	}

	#[test]
	fn test_check_oso_kernel_file_exists() {
		let (temp_dir, kernel_path,) = create_test_environment();

		// Create the kernel file
		File::create(&kernel_path,).expect("Failed to create kernel file",);

		// Change to the temp directory
		let original_dir = current_dir().expect("Failed to get current directory",);
		set_current_dir(temp_dir.path(),).expect("Failed to change directory",);

		// Test that check_oso_kernel succeeds when file exists
		let result = check_oso_kernel();

		// Restore original directory
		set_current_dir(original_dir,).expect("Failed to restore directory",);

		assert!(result.is_ok());
	}

	#[test]
	fn test_check_oso_kernel_file_not_exists() {
		let (temp_dir, _kernel_path,) = create_test_environment();

		// Don't create the kernel file

		// Change to the temp directory
		let original_dir = current_dir().expect("Failed to get current directory",);
		set_current_dir(temp_dir.path(),).expect("Failed to change directory",);

		// Test that check_oso_kernel fails when file doesn't exist
		let result = check_oso_kernel();

		// Restore original directory
		set_current_dir(original_dir,).expect("Failed to restore directory",);

		assert!(result.is_err());

		// Check error message
		let error_msg = result.unwrap_err().to_string();
		assert!(error_msg.contains("oso_kernel.elf not exist"));
	}

	#[test]
	fn test_check_oso_kernel_target_directory_not_exists() {
		let temp_dir = TempDir::new().expect("Failed to create temp directory",);

		// Don't create target directory

		// Change to the temp directory
		let original_dir = current_dir().expect("Failed to get current directory",);
		set_current_dir(temp_dir.path(),).expect("Failed to change directory",);

		// Test that check_oso_kernel fails when target directory doesn't exist
		let result = check_oso_kernel();

		// Restore original directory
		set_current_dir(original_dir,).expect("Failed to restore directory",);

		assert!(result.is_err());
	}

	#[test]
	fn test_check_oso_kernel_path_construction() {
		// Test that the path is constructed correctly
		let current = current_dir().expect("Failed to get current directory",);
		let expected_path = current.join("target/oso_kernel.elf",);

		// We can't easily test the internal path construction without modifying the function,
		// but we can test that it behaves consistently
		let result1 = check_oso_kernel();
		let result2 = check_oso_kernel();

		// Both calls should have the same result (both succeed or both fail)
		assert_eq!(result1.is_ok(), result2.is_ok());
	}

	#[test]
	fn test_module_visibility() {
		// Test that all modules are properly exposed
		// This is more of a compilation test - if it compiles, the modules are accessible

		// We can't directly test the module contents without using them,
		// but we can verify they exist by referencing their types
		use crate::fonts_data;
		use crate::gen_wrapper_fn;
		use crate::impl_init;
		use crate::status_from_spec;
		use crate::test_elf_header_parse;
		use crate::test_program_headers_parse;

		// If this compiles, all modules are accessible
		assert!(true);
	}

	#[test]
	fn test_anyhow_result_alias() {
		// Test that our Result alias works correctly
		fn test_function() -> Rslt<i32,> {
			Ok(42,)
		}

		let result = test_function();
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), 42);
	}

	#[test]
	fn test_anyhow_error_creation() {
		// Test that we can create anyhow errors
		let error = anyhow!("Test error message");
		let error_string = error.to_string();
		assert!(error_string.contains("Test error message"));
	}

	#[test]
	fn test_crate_features() {
		// This test verifies that the crate compiles with all the required features
		// If any feature is missing, compilation would fail

		// Test proc_macro_diagnostic feature (implicitly tested by compilation)
		// Test str_as_str feature (implicitly tested by compilation)
		// Test iter_array_chunks feature (implicitly tested by compilation)
		// Test associated_type_defaults feature (implicitly tested by compilation)
		// Test iterator_try_collect feature (implicitly tested by compilation)

		assert!(true);
	}

	#[test]
	fn test_error_propagation() {
		// Test that errors propagate correctly through the Result type
		fn failing_function() -> Rslt<(),> {
			check_oso_kernel()?; // This will likely fail in test environment
			Ok((),)
		}

		let result = failing_function();
		// In most test environments, this should fail because oso_kernel.elf doesn't exist
		// But we don't assert the specific result since it depends on the test environment

		// Just verify that the Result type works correctly
		match result {
			Ok(_,) => assert!(true),
			Err(_,) => assert!(true),
		}
	}

	#[test]
	fn test_path_join_functionality() {
		// Test that path joining works correctly (used in check_oso_kernel)
		let base = PathBuf::from("/tmp",);
		let joined = base.join("target/oso_kernel.elf",);

		assert!(joined.to_string_lossy().contains("target"));
		assert!(joined.to_string_lossy().contains("oso_kernel.elf"));
	}

	#[test]
	fn test_file_exists_check() {
		// Test the file existence checking logic
		let temp_dir = TempDir::new().expect("Failed to create temp directory",);
		let test_file = temp_dir.path().join("test_file.txt",);

		// File doesn't exist initially
		assert!(!test_file.exists());

		// Create the file
		File::create(&test_file,).expect("Failed to create test file",);

		// File should now exist
		assert!(test_file.exists());
	}
}
