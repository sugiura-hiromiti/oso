#![no_std]
#![no_main]
// #![feature(stdarch_arm_hints)]

use core::arch::asm;
#[cfg(target_arch = "aarch64")]
use oso_bridge::device_tree::DeviceTreeAddress;
use oso_bridge::wfi;
use oso_error::Rslt;
// use oso_kernel::base::graphic::FrameBuffer;
// #[cfg(feature = "bgr")]
// use oso_kernel::base::graphic::color::Bgr;
// #[cfg(feature = "bitmask")]
// use oso_kernel::base::graphic::color::Bitmask;
// #[cfg(feature = "bltonly")]
// use oso_kernel::base::graphic::color::BltOnly;
// #[cfg(feature = "rgb")]
// use oso_kernel::base::graphic::color::Rgb;
// use oso_kernel::base::graphic::fill_rectangle;
// use oso_kernel::base::graphic::outline_rectangle;
use oso_kernel::init;

#[unsafe(no_mangle)]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main(_device_tree_ptr: DeviceTreeAddress,) {
	// NOTE: Disable IRQ(interrupt request)

	unsafe {
		asm!("msr daifset, #2");
	}

	init();

	let _ = app();

	// NOTE: stops program for debugging purpose
	wfi();
}

#[unsafe(no_mangle)]
#[cfg(target_arch = "x86_64")]
pub extern "sysv64" fn kernel_main() {
	loop {
		unsafe {
			asm!("hlt");
		}
	}

	#[cfg(feature = "rgb")]
	enter_app!(Rgb, frame_buf_conf);
	#[cfg(feature = "bgr")]
	enter_app!(Bgr, frame_buf_conf);
	#[cfg(feature = "bitmask")]
	enter_app!(Bitmask, frame_buf_conf);
	#[cfg(feature = "bltonly")]
	// enter_app!(BltOnly, frame_buf_conf);
	loop {
		unsafe {
			asm!("hlt");
		}
	}
}

fn app() -> Rslt<(),> {
	// fill_rectangle(&(100, 100,), &(700, 500,), &"#abcdef",)?;
	// fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#012345",)?;
	//
	// fill_rectangle(&(100, 100,), &(200, 200,), &"#fedcba",)?;
	//
	// fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#ffffff",)?;
	// fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#abcdef",)?;
	// outline_rectangle(&(100, 100,), &(300, 300,), &"#fedcba",)?;
	// outline_rectangle(&(101, 101,), &(299, 299,), &"#fedcba",)?;
	// outline_rectangle(&(102, 102,), &(298, 298,), &"#fedcba",)?;

	// println!("width: {} height: {}", FRAME_BUFFER.width, FRAME_BUFFER.height);
	// println!("size: {} stride: {}", FRAME_BUFFER.size, FRAME_BUFFER.stride);
	// println!("buf address: {}", FRAME_BUFFER.buf);
	//
	// let mut cursor_buf = CursorBuf::new();
	// cursor_buf.draw_mouse_cursor()?;
	//
	Ok((),)
}
