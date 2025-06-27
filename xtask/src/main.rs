#![feature(string_from_utf8_lossy_owned)]
#![feature(exit_status_error)]

//! # OSO xtask
//!
//! A build and run utility for the OSO project that automates the process of building,
//! packaging, and running the OSO kernel and loader in QEMU.
//!
//! This crate provides a convenient way to:
//! - Build the OSO loader (UEFI application) and kernel
//! - Create and format a disk image
//! - Mount the disk image and copy the built artifacts
//! - Configure and run QEMU with the appropriate firmware and disk image
//!
//! ## Usage
//!
//! Run from the OSO project root:
//!
//! ```bash
//! cargo run -p xtask [OPTIONS]
//! ```
//!
//! ### Options
//!
//! - `-r`, `--release`: Build in release mode (default is debug mode)
//! - `-86`, `-x86_64`: Build for x86_64 architecture (default is aarch64)
//! - `--debug`: Enable debug mode with GDB support (listens on port 12345)

use anyhow::Result as Rslt;
use builder::Builder;
use colored::Colorize;
use std::process::Command;
use util_common_code::Run;

pub mod builder;
pub mod qemu;
pub mod shell;
pub mod workspace;

/// Entry point for the xtask utility.
///
/// Creates a new Builder instance, builds the OSO loader and kernel,
/// and runs QEMU with the appropriate configuration.
fn main() -> Rslt<(),> {
	let builder = Builder::new()?;

	let app = || {
		builder.build()?;
		builder.run()
	};

	match app() {
		Ok(_,) => println!("\n\nprogram run successfully\nexit"),
		Err(e,) => {
			eprintln!("{} error msg:\n```rust\n{e:#?}\n```", "program panicked".red().bold())
		},
	}

	print_workspace()?;
	Ok((),)
}

fn print_workspace() -> Rslt<(),> {
	Command::new("eza",)
		.args(
			"-ahlF --icons --group-directories-first --sort=extension --time-style=iso --git \
			 --no-user --no-time -T target/xtask"
				.split_whitespace(),
		)
		.run()
}
