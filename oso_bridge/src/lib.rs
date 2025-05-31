#![no_std]
#![feature(associated_type_defaults)]

use core::arch::asm;

pub mod graphic;

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
