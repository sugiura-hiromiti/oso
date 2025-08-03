//! # OSO Development Utilities
//!
//! A collection of development utilities and helper functions for the OSO operating system project.
//! This crate provides tools for workspace management, command execution, and development workflow
//! automation.
//!
//! ## Features
//!
//! - **Workspace Management**: Tools for managing multi-crate workspaces
//! - **Command Execution**: Enhanced command execution with better error handling and output
//!   formatting
//! - **Development Workflow**: Utilities to streamline the development process
//! - **Cross-platform Support**: Works across different operating systems
//!
//! ## Key Components
//!
//! ### Command Execution
//!
//! The [`Run`] trait provides enhanced command execution capabilities with:
//! - Colored output formatting
//! - Automatic error handling
//! - Inherited stdio streams
//! - Command display with arguments
//!
//! ### Workspace Management
//!
//! The workspace management system provides:
//! - Root directory detection
//! - Crate enumeration and management
//! - Workspace-wide operations
//!
//! ## Usage
//!
//! ### Basic Command Execution
//!
//! ```rust,no_run
//! use oso_dev_util::Run;
//! use std::process::Command;
//!
//! // Execute a command with enhanced output
//! let mut cmd = Command::new("cargo",);
//! cmd.args(&["build", "--release",],);
//! cmd.run().expect("Build failed",);
//! ```
//!
//! ### Workspace Operations
//!
//! ```rust,ignore
//! use oso_dev_util::{OsoWorkspace, OsoWorkspaceManager};
//!
//! let workspace = OsoWorkspaceManager::new();
//! let root = workspace.root();
//! let crates = workspace.crates();
//!
//! println!("Workspace root: {}", root.display());
//! for crate_path in crates {
//!     println!("Crate: {}", crate_path.display());
//! }
//! ```
//!
//! ## Dependencies
//!
//! - [`anyhow`]: Error handling and context
//! - [`colored`]: Terminal color output
//! - [`toml`]: TOML configuration file parsing

#![feature(exit_status_error)]

use anyhow::Result as Rslt;

// pub mod cli; // Will be added in feat/add-cli-module branch
#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// flowchart TD
/// A[Crate] --> B[Workspace]
/// A --> C[Package]
/// B --> D[CrateBase]
/// C --> D
/// ```
pub mod decl_manage;
// pub mod fs; // Will be added in feat/add-cli-module branch

/// The path to the oso_dev_util crate manifest, set at compile time
const OSO_DEV_UTIL_PATH: &'static str = std::env!("CARGO_MANIFEST_PATH");
