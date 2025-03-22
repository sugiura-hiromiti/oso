#![no_std]
#![no_main]

use core::arch::asm;
use oso_bridge::graphic::FrameBufConf;
use oso_kernel::base::graphic::Draw;
use oso_kernel::error::KernelError;
use oso_kernel::base::graphic::FrameBuffer;
use oso_kernel::gui::font::Text;
use oso_kernel::gui::font::TextBuf;

#[unsafe(no_mangle)]
/// TODO:
/// `extern "sysv64"` を除く事はできるのか?
/// カーネルを呼び出すのはうまく行っているようだが、bootloaderとkernel側で sysv64 abi
/// に則った関数として扱わないと引数の受け渡しがうまくいかない
/// elf形式でコンパイルするので、恐らくその時に (x86環境では)sysv64 abi
/// が強制されているのではないか？
pub extern "sysv64" fn kernel_main(frame_buf_conf: FrameBufConf,) {
	let mut fb = FrameBuffer::new(frame_buf_conf,);
	if let Err(_ke,) = app(&mut fb,) {
		todo!()
	}

	loop {
		unsafe {
			asm!("hlt");
		}
	}
}

fn app(fb: &mut FrameBuffer,) -> Result<(), KernelError,> {
	fb.fill_rectangle(&(0, 0,), &(fb.width - 1, fb.height - 1,), &"#ffffff",)
		.expect("failed fill rectangle",);

	fb.fill_rectangle(&(100, 100,), &(200, 200,), &"#fedcba",).expect("failed fill rectangle",);

	let text_buf = &mut TextBuf::new((0, 0,), 8, 16,);

	let width = {
		let mut digits = [0, 0, 0,];
		let mut width = fb.width;

		let mut idx = 0;
		while width > 0 {
			let modulo = width % 10;
			digits[idx] = modulo;
			width /= 10;
			idx += 1;
		}

		digits
	};
	let height = {
		let mut digits = [0, 0, 0,];
		let mut height = fb.height;

		let mut idx = 0;
		while height > 0 {
			let modulo = height % 10;
			digits[idx] = modulo;
			height /= 10;
			idx += 1;
		}

		digits
	};

	fb.write_str("\nwidth: ", text_buf,);
	for i in (0..3).rev() {
		fb.write_char(b'0' + width[i] as u8, text_buf,).unwrap();
	}
	fb.write_str("\nheight: ", text_buf,);
	for i in (0..3).rev() {
		fb.write_char(b'0' + height[i] as u8, text_buf,).unwrap();
	}
	fb.write_char(b'\n', text_buf,).unwrap();

	for y in 0..16 {
		for x in 0..16 {
			let idx = x + y * 16;
			fb.write_char(idx, text_buf,).unwrap();
		}
		fb.write_char(b'\n', text_buf,).unwrap();
	}

	fb.fill_rectangle(&(0, 0,), &(fb.width - 1, fb.height - 1,), &"#000000",).unwrap();

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
