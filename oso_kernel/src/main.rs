#![no_std]
#![no_main]

use core::arch::asm;
#[allow(unused_imports)]
use oso_bridge::graphic::FrameBufConf;
use oso_kernel::app::cursor::CursorBuf;
use oso_kernel::app::cursor::MouseCursorDraw;
use oso_kernel::base::graphic::FRAME_BUFFER;
use oso_kernel::base::graphic::FrameBuffer;
#[cfg(feature = "bgr")]
use oso_kernel::base::graphic::color::Bgr;
#[cfg(feature = "bitmask")]
use oso_kernel::base::graphic::color::Bitmask;
#[cfg(feature = "bltonly")]
use oso_kernel::base::graphic::color::BltOnly;
#[cfg(feature = "rgb")]
use oso_kernel::base::graphic::color::Rgb;
use oso_kernel::base::graphic::fill_rectangle;
use oso_kernel::base::graphic::outline_rectangle;
use oso_kernel::error::KernelError;
use oso_kernel::println;

#[unsafe(no_mangle)]
#[cfg(target_arch = "x86_64")]
/// TODO:
/// `extern "sysv64"` を除く事はできるのか?
/// カーネルを呼び出すのはうまく行っているようだが、bootloaderとkernel側で sysv64 abi
/// に則った関数として扱わないと引数の受け渡しがうまくいかない
/// elf形式でコンパイルするので、恐らくその時に (x86環境では)sysv64 abi
/// が強制されているのではないか？
pub extern "sysv64" fn kernel_main(frame_buf_conf: FrameBufConf,) {
	macro_rules! enter_app {
		($pixel_format:expr) => {
			let fb = FrameBuffer::new(frame_buf_conf, $pixel_format,);
			unsafe {
				FrameBuffer::init(
					&FRAME_BUFFER as *const FrameBuffer<_,>,
					fb.buf,
					fb.size,
					fb.width,
					fb.height,
					fb.stride,
				);
				//FRAME_BUFFER = FrameBuffer::new(frame_buf_conf, $pixel_format,);
			}
			if let Err(_ke,) = app() {
				todo!()
			}
		};
	}

	#[cfg(feature = "rgb")]
	enter_app!(Rgb);
	#[cfg(feature = "bgr")]
	enter_app!(Bgr);
	#[cfg(feature = "bitmask")]
	enter_app!(Bitmask);
	#[cfg(feature = "bltonly")]
	enter_app!(BltOnly);

	loop {
		unsafe {
			asm!("hlt");
		}
	}
}

// #[unsafe(no_mangle)]
// #[cfg(target_arch = "aarch64")]
// pub extern "C" fn kernel_main(frame_buf_conf: FrameBufConf,) {
// 	todo!()
// }

fn app() -> Result<(), KernelError,> {
	fill_rectangle(&(100, 100,), &(700, 500,), &"#abcdef",)?;
	fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#012345",)?;

	fill_rectangle(&(100, 100,), &(200, 200,), &"#fedcba",)?;

	println!("width: {} height: {}", FRAME_BUFFER.width, FRAME_BUFFER.height);
	// to_txt!(let width = 3u8);
	// write_str("\nwidth: ", text_buf,)?;
	// write_str(width, text_buf,)?;
	// write_char(b'\n', text_buf,)?;

	// for y in 0..16 {
	// 	for x in 0..16 {
	// 		let idx = x + y * 16;
	// 		write_char(idx, text_buf,)?;
	// 	}
	// 	write_char(b'\n', text_buf,)?;
	// }

	fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#ffffff",)?;
	fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#abcdef",)?;
	outline_rectangle(&(100, 100,), &(300, 300,), &"#fedcba",)?;

	let mut cursor_buf = CursorBuf::new();
	cursor_buf.draw_mouse_cursor()?;

	Ok((),)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo,) -> ! {
	println!("{}", info);
	loop {}
}
