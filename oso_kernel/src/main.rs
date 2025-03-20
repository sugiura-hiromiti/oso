#![no_std]
#![no_main]

use core::arch::asm;
use oso_bridge::graphic::FrameBufConf;
use oso_kernel::graphic::Draw;
use oso_kernel::graphic::FrameBuffer;

#[unsafe(no_mangle)]
/// TODO:
/// `extern "sysv64"` を除く事はできるのか?
/// カーネルを呼び出すのはうまく行っているようだが、bootloaderとkernel側で sysv64 abi
/// に則った関数として扱わないと引数の受け渡しがうまくいかない
/// elf形式でコンパイルするので、恐らくその時に (x86環境では)sysv64 abi
/// が強制されているのではないか？
pub extern "sysv64" fn kernel_main(frame_buf_conf: FrameBufConf,) {
	let mut fb = FrameBuffer::new(frame_buf_conf,);
	fb.fill_rectangle(&(0, 0,), &(fb.width - 1, fb.height - 1,), &(0xff, 0xff, 0xff,),)
		.expect("failed fill rectangle",);

	// #123456
	fb.fill_rectangle(&(0, 0,), &(100, 100,), &(0x01, 0x23, 0x45,),)
		.expect("failed fill rectangle",);

	loop {
		unsafe {
			asm!("hlt");
		}
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
