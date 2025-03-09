#![no_std]
#![no_main]

use core::arch::asm;
use oso_util::graphic::FrameBufConf;

#[unsafe(no_mangle)]
/// TODO:
/// `extern "sysv64"` を除く事はできるのか?
/// カーネルを呼び出すのはうまく行っているようだが、bootloaderとkernel側で sysv64 abi
/// に則った関数として扱わないと引数の受け渡しがうまくいかない
/// elf形式でコンパイルするので、恐らくその時に (x86環境では)sysv64 abi
/// が強制されているのではないか？
pub extern "sysv64" fn kernel_main(frame_buf_conf: FrameBufConf,) {
	let f_buf =
		unsafe { core::slice::from_raw_parts_mut(frame_buf_conf.base, frame_buf_conf.size,) };

	f_buf.iter_mut().enumerate().for_each(|(i, frame,)| {
		*frame = ((i * 100) % 0x100) as u8;
	},);

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
