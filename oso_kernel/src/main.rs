#![no_std]
#![no_main]

use core::arch::asm;
use oso_bridge::graphic::FrameBufConf;
use oso_kernel::app::cursor::CursorBuf;
use oso_kernel::app::cursor::MouseCursorDraw;
use oso_kernel::base::graphic::DisplayDraw;
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
use oso_kernel::base::text::Integer;
use oso_kernel::base::text::Text;
use oso_kernel::base::text::TextBuf;
use oso_kernel::error::KernelError;
use oso_kernel::to_txt;

#[unsafe(no_mangle)]
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

	// let mut fb = FrameBuffer::new(frame_buf_conf,);
	// if let Err(_ke,) = app(&mut fb,) {
	// 	todo!()
	// }

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
	unsafe {
		FRAME_BUFFER.fill_rectangle(&(100, 100,), &(700, 500,), &"#abcdef",)?;
		FRAME_BUFFER.fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#012345",)?;

		FRAME_BUFFER.fill_rectangle(&(100, 100,), &(200, 200,), &"#fedcba",)?;

		let text_buf = &mut TextBuf::new((0, 0,), 8, 16,);
		to_txt!(let width = 3u8);
		FRAME_BUFFER.write_str("\nwidth: ", text_buf,)?;
		FRAME_BUFFER.write_str(width, text_buf,)?;
		FRAME_BUFFER.write_char(b'\n', text_buf,)?;

		for y in 0..16 {
			for x in 0..16 {
				let idx = x + y * 16;
				FRAME_BUFFER.write_char(idx, text_buf,)?;
			}
			FRAME_BUFFER.write_char(b'\n', text_buf,)?;
		}

		text_buf.clear();
		FRAME_BUFFER.fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#ffffff",)?;

		to_txt!(let width = FRAME_BUFFER.width);
		to_txt!(let height = FRAME_BUFFER.height);
		FRAME_BUFFER.write_str("\nwidth: ", text_buf,);
		FRAME_BUFFER.write_str(width, text_buf,);
		FRAME_BUFFER.write_str("\nheight: ", text_buf,);
		FRAME_BUFFER.write_str(height, text_buf,);
		to_txt!(let minus = -100);
		FRAME_BUFFER.write_str("\nminus: ", text_buf,);
		FRAME_BUFFER.write_str(minus, text_buf,);

		FRAME_BUFFER.fill_rectangle(&(0, 0,), &FRAME_BUFFER.right_bottom(), &"#fedcba",)?;

		let cursor_buf = CursorBuf::new((123, 456,), 15, 24,);
		FRAME_BUFFER.draw_mouse_cursor(&cursor_buf,)?;

		Ok((),)
	}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo,) -> ! {
	loop {
		// unsafe {
		// 	asm!("hlt");
		// }
	}
}
