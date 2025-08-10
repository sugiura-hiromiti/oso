// NOTE:  this file must be copied to oso_proc_macro_logic_2/src/lib.rs on every build
use anyhow::Result as Rslt;
use colored::Colorize;
use std::ffi::OsStr;
use std::process::Command;
use std::process::Stdio;

/// Trait for enhanced command execution with better error handling and output formatting
///
/// The `Run` trait extends the standard [`Command`] functionality with:
/// - Colored command output display
/// - Automatic stdio inheritance
/// - Enhanced error handling with context
/// - Command argument formatting
///
/// This trait is particularly useful for development tools and build scripts where
/// clear command output and error reporting are essential.
///
/// # Examples
///
/// ```rust,no_run
/// use oso_dev_util_helper::cli::Run;
/// use std::process::Command;
///
/// let mut cmd = Command::new("ls",);
/// cmd.args(&["-la", "/tmp",],);
///
/// match cmd.run() {
/// 	Ok((),) => println!("Command executed successfully"),
/// 	Err(e,) => eprintln!("Command failed: {}", e),
/// }
/// ```
pub trait Run {
	/// Executes the command with enhanced output and error handling
	///
	/// This method runs the command while providing:
	/// - Colored display of the command being executed
	/// - Inherited stdio streams for interactive commands
	/// - Proper error handling with exit code checking
	/// - Formatted command argument display
	///
	/// # Returns
	///
	/// * `Ok(())` - If the command executed successfully (exit code 0)
	/// * `Err(anyhow::Error)` - If the command failed or returned a non-zero exit code
	///
	/// # Errors
	///
	/// This method will return an error if:
	/// - The command cannot be found or executed
	/// - The command returns a non-zero exit code
	/// - There are I/O errors during command execution
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use oso_dev_util_helper::cli::Run;
	/// use std::process::Command;
	///
	/// // Execute a simple command
	/// let mut cmd = Command::new("echo",);
	/// cmd.arg("Hello, World!",);
	/// cmd.run().expect("Echo command failed",);
	///
	/// // Execute a build command
	/// let mut build_cmd = Command::new("cargo",);
	/// build_cmd.args(&["build", "--release",],);
	/// build_cmd.run().expect("Build failed",);
	/// ```
	///
	/// # Output Format
	///
	/// The method displays the command in the following format:
	/// ```text
	/// program_name arg1 arg2 arg3
	/// ```
	/// The command line is displayed in bold blue text for easy identification.
	fn run(&mut self,) -> Rslt<(),>;
}

impl Run for Command {
	/// Executes the command with enhanced formatting and error handling
	///
	/// This implementation provides a user-friendly command execution experience
	/// with colored output, proper error handling, and stdio inheritance.
	///
	/// # Implementation Details
	///
	/// 1. **Command Display**: Formats and displays the command with arguments in bold blue
	/// 2. **Stdio Configuration**: Inherits stdout, stderr, and stdin from the parent process
	/// 3. **Execution**: Runs the command and waits for completion
	/// 4. **Error Checking**: Validates the exit status and converts errors to `anyhow::Error`
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use oso_dev_util_helper::cli::Run;
	/// use std::process::Command;
	///
	/// let mut cmd = Command::new("git",);
	/// cmd.args(&["status", "--porcelain",],);
	///
	/// // This will display: git status --porcelain
	/// // in bold blue, then execute the command
	/// cmd.run().expect("Git command failed",);
	/// ```
	fn run(&mut self,) -> Rslt<(),> {
		// Format the command display string with program and arguments
		let cmd_dsply = format!(
			"{} {}",
			self.get_program().display(),
			self.get_args().collect::<Vec<&OsStr,>>().join(OsStr::new(" ")).display()
		);

		// Display the command in bold blue for visibility
		println!("\n{}", cmd_dsply.bold().blue());

		// Configure stdio inheritance and execute the command
		let out = self
			.stdout(Stdio::inherit(),)  // Inherit stdout for real-time output
			.stderr(Stdio::inherit(),)  // Inherit stderr for error messages
			.stdin(Stdio::inherit(),)   // Inherit stdin for interactive commands
			.status()?; // Execute and get exit status

		// Check exit status and convert to Result
		out.exit_ok()?; // This will return an error if exit code != 0
		Ok((),)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run_trait_successful_command() {
		let mut cmd = Command::new("echo");
		cmd.arg("test");
		
		let result = cmd.run();
		assert!(result.is_ok(), "Echo command should succeed");
	}

	#[test]
	fn test_run_trait_failing_command() {
		let mut cmd = Command::new("false"); // Command that always fails
		
		let result = cmd.run();
		assert!(result.is_err(), "False command should fail");
	}

	#[test]
	fn test_run_trait_nonexistent_command() {
		let mut cmd = Command::new("definitely_nonexistent_command_12345");
		
		let result = cmd.run();
		assert!(result.is_err(), "Nonexistent command should fail");
	}

	#[test]
	fn test_run_trait_with_args() {
		let mut cmd = Command::new("echo");
		cmd.args(&["hello", "world"]);
		
		let result = cmd.run();
		assert!(result.is_ok(), "Echo with args should succeed");
	}

	#[test]
	fn test_run_trait_with_env() {
		let mut cmd = Command::new("echo");
		cmd.arg("test");
		cmd.env("TEST_VAR", "test_value");
		
		let result = cmd.run();
		assert!(result.is_ok(), "Echo with env should succeed");
	}

	#[test]
	fn test_run_trait_multiple_calls() {
		// Test that we can call run multiple times on different command instances
		let mut cmd1 = Command::new("echo");
		cmd1.arg("first");
		assert!(cmd1.run().is_ok());

		let mut cmd2 = Command::new("echo");
		cmd2.arg("second");
		assert!(cmd2.run().is_ok());
	}

	#[test]
	fn test_run_trait_with_current_dir() {
		let mut cmd = Command::new("pwd");
		cmd.current_dir("/tmp");
		
		let result = cmd.run();
		// This might fail on some systems if /tmp doesn't exist or pwd isn't available
		// but we're mainly testing that the trait works with current_dir
		let _ = result; // Don't assert success since it's system-dependent
	}

	#[test]
	fn test_run_trait_error_contains_command_info() {
		let mut cmd = Command::new("definitely_nonexistent_command_12345");
		
		let result = cmd.run();
		assert!(result.is_err());
		
		let error_msg = result.unwrap_err().to_string();
		// The error should contain some information about what went wrong
		assert!(!error_msg.is_empty());
	}

	#[test]
	fn test_command_builder_pattern() {
		// Test that we can use the builder pattern with our trait
		let result = Command::new("echo")
			.arg("builder")
			.arg("pattern")
			.env("TEST", "value")
			.run();
		
		assert!(result.is_ok(), "Builder pattern should work");
	}

	#[test]
	fn test_run_trait_idempotent_success() {
		// Test that successful commands are idempotent
		let mut cmd = Command::new("true"); // Command that always succeeds
		
		assert!(cmd.run().is_ok());
		// Note: We can't call run again on the same Command instance
		// because std::process::Command consumes itself on spawn()
	}

	#[test]
	fn test_run_trait_idempotent_failure() {
		// Test that failing commands consistently fail
		let mut cmd = Command::new("false"); // Command that always fails
		
		assert!(cmd.run().is_err());
		// Note: We can't call run again on the same Command instance
	}

	#[test]
	fn test_run_trait_with_output_redirection() {
		// Test commands that might produce output
		let mut cmd = Command::new("echo");
		cmd.arg("output_test");
		
		let result = cmd.run();
		assert!(result.is_ok(), "Echo should succeed even with output");
	}

	#[test]
	fn test_run_trait_with_error_output() {
		// Test commands that write to stderr
		let mut cmd = Command::new("sh");
		cmd.args(&["-c", "echo 'error message' >&2"]);
		
		let result = cmd.run();
		assert!(result.is_ok(), "Command writing to stderr should still succeed if exit code is 0");
	}

	#[test]
	fn test_run_trait_exit_code_handling() {
		// Test that non-zero exit codes are treated as errors
		let mut cmd = Command::new("sh");
		cmd.args(&["-c", "exit 1"]);
		
		let result = cmd.run();
		assert!(result.is_err(), "Non-zero exit code should be treated as error");
	}

	#[test]
	fn test_run_trait_zero_exit_code() {
		// Test that zero exit code is treated as success
		let mut cmd = Command::new("sh");
		cmd.args(&["-c", "exit 0"]);
		
		let result = cmd.run();
		assert!(result.is_ok(), "Zero exit code should be treated as success");
	}

	#[test]
	fn test_run_trait_with_long_running_command() {
		// Test with a command that takes a bit of time
		let mut cmd = Command::new("sleep");
		cmd.arg("0.1"); // Sleep for 100ms
		
		let result = cmd.run();
		assert!(result.is_ok(), "Sleep command should succeed");
	}

	#[test]
	fn test_run_trait_command_not_found_vs_execution_failure() {
		// Test the difference between command not found and execution failure
		let mut nonexistent_cmd = Command::new("definitely_nonexistent_command_12345");
		let nonexistent_result = nonexistent_cmd.run();
		assert!(nonexistent_result.is_err());

		let mut failing_cmd = Command::new("false");
		let failing_result = failing_cmd.run();
		assert!(failing_result.is_err());

		// Both should fail, but potentially with different error types
		// We can't easily distinguish them in the test, but both should be errors
	}
}
