#![no_std]
#![no_main]

use core::arch::asm;
use oso_bridge::graphic::FrameBufConf;
use oso_kernel::app::cursor::CursorBuf;
use oso_kernel::app::cursor::MouseCursorDraw;
use oso_kernel::base::graphic::DisplayDraw;
use oso_kernel::base::graphic::FrameBuffer;
use oso_kernel::base::graphic::color::Bgr;
use oso_kernel::base::graphic::color::Bitmask;
use oso_kernel::base::graphic::color::BltOnly;
use oso_kernel::base::graphic::color::PixelFormat;
use oso_kernel::base::graphic::color::Rgb;
use oso_kernel::error::KernelError;
use oso_kernel::gui::text::Integer;
use oso_kernel::gui::text::Text;
use oso_kernel::gui::text::TextBuf;
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
		($pixel_format:expr) => {{
			let mut fb = FrameBuffer::new(frame_buf_conf, $pixel_format,);
			if let Err(_ke,) = app(&mut fb,) {
				todo!()
			}
		}};
	}
	match frame_buf_conf.pixel_format {
		oso_bridge::graphic::PixelFormatConf::Rgb => {
			enter_app!(Rgb)
		},
		oso_bridge::graphic::PixelFormatConf::Bgr => enter_app!(Bgr),
		oso_bridge::graphic::PixelFormatConf::Bitmask => enter_app!(Bitmask),
		oso_bridge::graphic::PixelFormatConf::BltOnly => enter_app!(BltOnly),
	}

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

fn app<P: PixelFormat,>(fb: &mut FrameBuffer<P,>,) -> Result<(), KernelError,> {
	fb.fill_rectangle(&(100, 100,), &(700, 500,), &"#abcdef",)?;
	fb.fill_rectangle(&(0, 0,), &fb.right_bottom(), &"#012345",)?;

	fb.fill_rectangle(&(100, 100,), &(200, 200,), &"#fedcba",)?;

	let text_buf = &mut TextBuf::new((0, 0,), 8, 16,);
	to_txt!(let width = 3u8);
	fb.write_str("\nwidth: ", text_buf,);
	fb.write_str(width, text_buf,)?;
	fb.write_char(b'\n', text_buf,)?;

	for y in 0..16 {
		for x in 0..16 {
			let idx = x + y * 16;
			fb.write_char(idx, text_buf,)?;
		}
		fb.write_char(b'\n', text_buf,)?;
	}

	text_buf.clear();
	fb.fill_rectangle(&(0, 0,), &fb.right_bottom(), &"#ffffff",)?;

	to_txt!(let width = fb.width);
	to_txt!(let height = fb.height);
	fb.write_str("\nwidth: ", text_buf,);
	fb.write_str(width, text_buf,);
	fb.write_str("\nheight: ", text_buf,);
	fb.write_str(height, text_buf,);
	to_txt!(let minus = -100);
	fb.write_str("\nminus: ", text_buf,);
	fb.write_str(minus, text_buf,);

	fb.fill_rectangle(&(0, 0,), &fb.right_bottom(), &"#abcdef",)?;

	let cursor_buf = CursorBuf::new((123, 456,), 15, 24,);
	fb.draw_mouse_cursor(&cursor_buf,)?;

	Ok((),)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo,) -> ! {
	loop {
		// unsafe {
		// 	asm!("hlt");
		// }
	}
}
