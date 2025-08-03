// NOTE:  this file must be copied to oso_proc_macro_logic_2/src/lib.rs on every build
use crate::Rslt;
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
/// use oso_dev_util::Run;
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
	/// use oso_dev_util::Run;
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
	/// use oso_dev_util::Run;
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
