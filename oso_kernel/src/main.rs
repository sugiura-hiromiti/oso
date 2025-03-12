#![no_std]
#![no_main]

use core::arch::asm;
use oso_util::graphic::Draw;
use oso_util::graphic::FrameBufConf;
use oso_util::graphic::FrameBuffer;

#[unsafe(no_mangle)]
/// TODO:
/// `extern "sysv64"` を除く事はできるのか?
/// カーネルを呼び出すのはうまく行っているようだが、bootloaderとkernel側で sysv64 abi
/// に則った関数として扱わないと引数の受け渡しがうまくいかない
/// elf形式でコンパイルするので、恐らくその時に (x86環境では)sysv64 abi
/// が強制されているのではないか？
pub extern "sysv64" fn kernel_main(frame_buf_conf: FrameBufConf,) {
	// let f_buf =
	// 	unsafe { core::slice::from_raw_parts_mut(frame_buf_conf.base, frame_buf_conf.size,) };
	//
	// f_buf.iter_mut().enumerate().for_each(|(i, frame,)| {
	// 	*frame = ((i * 100) % 0x100) as u8;
	// },);

	let mut fb = FrameBuffer::new(frame_buf_conf,);
	// let half_w = fb.width / 2;
	// let half_h = fb.height / 2;
	// fb.fill_rectangle(&(0, 0,).into(), &(half_w, half_h,).into(), &(0xff, 0xff, 0xff,).into(),)
	// 	.expect("failed to fill rectangle",);
	fb.fill_rectangle(
		&(0, 0,).into(),
		&(fb.width - 1, fb.height - 1,).into(),
		&(0xff, 0xff, 0xff,).into(),
	)
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
