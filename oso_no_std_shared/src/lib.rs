//! # OSO Bridge
//!
//! `oso_bridge` is a no_std crate that provides low-level interfaces and utilities for 
//! operating system development, particularly focused on bare-metal environments.
//! 
//! This crate serves as a bridge between hardware and the operating system kernel,
//! providing essential primitives for CPU control, framebuffer management, and device tree access.
//!
//! ## Features
//!
//! - CPU control functions (wait for interrupt, wait for event, no-operation)
//! - Framebuffer configuration for graphics output
//! - Device tree address handling
//!
//! ## Usage
//!
//! This crate is designed to be used in no_std environments, typically in kernel or bootloader code:
//!
//! ```rust,no_run
//! use oso_bridge::{wfi, graphic::FrameBufConf, graphic::PixelFormatConf};
//!
//! // Configure a framebuffer
//! let framebuf = FrameBufConf::new(
//!     PixelFormatConf::Rgb,
//!     0x1000_0000 as *mut u8,  // Base address
//!     1024 * 768 * 4,          // Size (bytes)
//!     1024,                    // Width (pixels)
//!     768,                     // Height (pixels)
//!     1024 * 4,                // Stride (bytes per row)
//! );
//!
//! // Put the CPU into a low-power state until an interrupt occurs
//! wfi(); // This function never returns
//! ```

#![no_std]
#![feature(associated_type_defaults)]

use core::arch::asm;

pub mod device_tree;
pub mod graphic;

/// Puts the CPU into a low-power state until an interrupt occurs.
///
/// This function enters an infinite loop where the CPU is repeatedly put into a 
/// wait-for-interrupt state. This is commonly used in bare-metal environments to 
/// conserve power when there's no work to be done.
///
/// # Platform-specific behavior
///
/// - On AArch64 (ARM): Uses the `wfi` (Wait For Interrupt) instruction
/// - On x86_64: Uses the `hlt` (Halt) instruction
///
/// # Examples
///
/// ```rust,no_run
/// use oso_bridge::wfi;
///
/// // After completing all necessary work:
/// wfi(); // CPU will enter low-power state until an interrupt occurs
/// ```
///
/// # Safety
///
/// This function never returns and contains inline assembly.
#[inline(always)]
pub fn wfi() -> ! {
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfi");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}

/// Puts the CPU into a low-power state until an event occurs.
///
/// This function enters an infinite loop where the CPU is repeatedly put into a 
/// wait-for-event state. This is similar to `wfi()` but responds to events rather 
/// than just interrupts, which can be useful in certain synchronization scenarios.
///
/// # Platform-specific behavior
///
/// - On AArch64 (ARM): Uses the `wfe` (Wait For Event) instruction
/// - On x86_64: Uses the `hlt` (Halt) instruction as a fallback
///
/// # Examples
///
/// ```rust,no_run
/// use oso_bridge::wfe;
///
/// // After setting up event monitoring:
/// wfe(); // CPU will enter low-power state until an event occurs
/// ```
///
/// # Safety
///
/// This function never returns and contains inline assembly.
#[inline(always)]
pub fn wfe() -> ! {
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfe");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}

/// Puts the CPU into an infinite loop of no-operation instructions.
///
/// This function enters an infinite loop where the CPU repeatedly executes
/// no-operation instructions. This can be useful for debugging or in situations
/// where you want to keep the CPU busy without doing meaningful work.
///
/// # Platform-specific behavior
///
/// - On AArch64 (ARM): Uses the `nop` (No Operation) instruction
/// - On x86_64: Uses the `hlt` (Halt) instruction as a fallback
///
/// # Examples
///
/// ```rust,no_run
/// use oso_bridge::nop;
///
/// // When you want to keep the CPU busy without doing work:
/// nop(); // CPU will continuously execute no-operation instructions
/// ```
///
/// # Safety
///
/// This function never returns and contains inline assembly.
#[inline(always)]
pub fn nop() -> ! {
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("nop");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}
