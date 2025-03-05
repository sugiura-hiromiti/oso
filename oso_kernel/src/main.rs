#![no_std]
#![no_main]

use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main() {
	loop {
		unsafe {
			asm!("hlt");
		}
	}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo,) -> ! {
	loop {
		unsafe {
			asm!("hlt");
		}
	}
}
