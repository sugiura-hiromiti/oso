#![no_std]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]

pub mod bridge;
pub mod data;
pub mod parser;

use core::arch::asm;

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
