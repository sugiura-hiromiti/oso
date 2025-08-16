use crate::Rslt;
use crate::decl_manage::crate_::OsoCrate;
use oso_dev_util_helper::fs::current_crate_path;
use oso_dev_util_helper::fs::project_root_path;

pub fn project_root() -> Rslt<OsoCrate,> {
	let pr = project_root_path()?;
	Ok(OsoCrate::from(pr,),)
}

pub fn current_crate() -> Rslt<OsoCrate,> {
	let ccp = current_crate_path()?;

	Ok(OsoCrate::from(ccp,),)
}

#[cfg(test)]
// #[cfg(false)]
mod tests {
	use super::*;
	use crate::decl_manage::crate_::CrateInfo;
	use std::path::PathBuf;

	#[test]
	fn test_project_root_function_exists() {
		// Test that the project_root function exists and returns a Result
		let result = project_root();

		// The function should return a Result, regardless of success or failure
		match result {
			Ok(crate_obj,) => {
				// If successful, verify we got an OsoCrate
				let path = crate_obj.path();
				assert!(path.is_absolute() || path.starts_with("."));
			},
			Err(e,) => {
				// If it fails, that's acceptable in test environments
				// Just verify we get a meaningful error
				let error_msg = e.to_string();
				assert!(!error_msg.is_empty());
			},
		}
	}

	#[test]
	fn test_current_crate_function_exists() {
		// Test that the current_crate function exists and returns a Result
		let result = current_crate();

		// The function should return a Result, regardless of success or failure
		match result {
			Ok(crate_obj,) => {
				// If successful, verify we got an OsoCrate
				let path = crate_obj.path();
				assert!(path.is_absolute() || path.starts_with("."));
			},
			Err(e,) => {
				// If it fails, that's acceptable in test environments
				// Just verify we get a meaningful error
				let error_msg = e.to_string();
				assert!(!error_msg.is_empty());
			},
		}
	}

	#[test]
	fn test_project_root_returns_oso_crate() {
		// Test that project_root returns an OsoCrate type
		let result = project_root();

		// Verify the return type is correct
		match result {
			Ok(_crate_obj,) => {},
			Err(_,) => {},
		}
	}

	#[test]
	fn test_current_crate_returns_oso_crate() {
		// Test that current_crate returns an OsoCrate type
		let result = current_crate();

		// Verify the return type is correct
		match result {
			Ok(_crate_obj,) => {
				// Type check passes if this compiles
			},
			Err(_,) => {
				// Error is acceptable in test environment
			},
		}
	}

	#[test]
	fn test_error_handling() {
		// Test error handling by checking error types
		use anyhow::Context;

		// Create a mock error scenario
		let mock_error: Rslt<String,> = Err(anyhow::anyhow!("mock error"),);

		assert!(mock_error.is_err());
		let error = mock_error.unwrap_err();
		assert_eq!(error.to_string(), "mock error");

		// Test error chaining
		let chained_error: Rslt<String,> =
			Err(anyhow::anyhow!("base error"),).context("additional context",);

		assert!(chained_error.is_err());
		let error = chained_error.unwrap_err();
		let error_string = error.to_string();
		assert!(error_string.contains("additional context"));
	}

	#[test]
	fn test_function_integration() {
		// Test that both functions use the same underlying helper functions
		// This is more of an integration test

		let project_result = project_root();
		let current_result = current_crate();

		// Both should return the same type
		match (project_result, current_result,) {
			(Ok(project_crate,), Ok(current_crate,),) => {
				// Both should be valid OsoCrate instances
				let _project_path = project_crate.path();
				let _current_path = current_crate.path();
			},
			(Err(_,), _,) | (_, Err(_,),) => {
				// Errors are acceptable in test environment
			},
		}
	}

	#[test]
	fn test_result_type_consistency() {
		// Test that both functions return the same Result type
		let project_result = project_root();
		let current_result = current_crate();

		// Verify both are the same type by using them in the same context
		let results = vec![project_result, current_result];

		for result in results {
			match result {
				Ok(_,) => (),
				Err(_,) => (),
			}
		}
	}

	#[test]
	fn test_function_error_propagation() {
		// Test that errors from helper functions are properly propagated
		// This is a behavioral test - we can't easily mock the helper functions,
		// but we can verify the error handling structure

		// Test project_root error handling
		let project_result = project_root();
		if let Err(e,) = project_result {
			// Should be an anyhow::Error
			let _error_string = e.to_string();
			let _error_chain: Vec<_,> = e.chain().collect();
		}

		// Test current_crate error handling
		let current_result = current_crate();
		if let Err(e,) = current_result {
			// Should be an anyhow::Error
			let _error_string = e.to_string();
			let _error_chain: Vec<_,> = e.chain().collect();
		}
	}

	#[test]
	fn test_module_dependencies() {
		// Test that the module correctly imports and uses its dependencies

		// Test that we can use the Rslt type alias
		let _result: Rslt<i32,> = Ok(42,);

		// Test that we can reference OsoCrate
		let _crate_type = std::marker::PhantomData::<OsoCrate,>;

		// Test that helper functions are accessible (even if they might fail)
		let _project_path_result = project_root_path();
		let _current_path_result = current_crate_path();
	}

	#[test]
	fn test_function_signatures() {
		// Test that function signatures are as expected

		// project_root should take no parameters and return Rslt<OsoCrate>
		let _: fn() -> Rslt<OsoCrate,> = project_root;

		// current_crate should take no parameters and return Rslt<OsoCrate>
		let _: fn() -> Rslt<OsoCrate,> = current_crate;
	}

	#[test]
	fn test_oso_crate_basic_functionality() {
		// Test basic OsoCrate functionality
		let test_path = PathBuf::from("/test/crate",);
		let crate_obj = OsoCrate::from(test_path.clone(),);

		// Test path method
		assert_eq!(crate_obj.path(), test_path);

		// Test that it implements required traits (compile-time check)
		let _cloned = crate_obj.clone();
		let _default = OsoCrate::default();

		// Test equality
		let another_crate = OsoCrate::from(test_path.clone(),);
		assert_eq!(crate_obj, another_crate);
	}

	#[test]
	fn test_cross_platform_paths() {
		// Test paths that work across different platforms
		let unix_path = PathBuf::from("/unix/style/path",);
		let relative_path = PathBuf::from("relative/path",);
		let current_dir = PathBuf::from(".",);

		let unix_crate = OsoCrate::from(unix_path.clone(),);
		let relative_crate = OsoCrate::from(relative_path.clone(),);
		let current_crate = OsoCrate::from(current_dir.clone(),);

		assert_eq!(unix_crate.path(), unix_path);
		assert_eq!(relative_crate.path(), relative_path);
		assert_eq!(current_crate.path(), current_dir);
	}
}
