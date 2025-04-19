#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::arch::asm;
use core::ffi::c_void;
use core::usize;
use goblin::elf;
use oso_bridge::graphic::FrameBufConf;
use oso_bridge::graphic::PixelFormatConf;
use oso_loader::Rslt;
use oso_loader::error::OsoLoaderError;
use oso_loader::init;
use oso_loader::println;
use oso_loader::raw::table::SystemTable;

// #[uefi::entry]
// fn efi_main() -> Status {
// 	uefi::helpers::init().unwrap();
//
// 	oso_loader::mmio::mmio_descriptor().unwrap();
//
// 	let (kernel_addr, frame_buf_conf,) = match app() {
// 		Ok(rslt,) => rslt,
// 		Err(e,) => {
// 			oso_loader::on_error!(e, "while executing app()");
// 			uefi::boot::stall(10_000_000,);
// 			return Status::ABORTED;
// 		},
// 	};
//
// 	debug!("exit boot services");
// 	exit_boot_services();
// 	exec_kernel(frame_buf_conf, kernel_addr,);
// 	Status::SUCCESS
// }

#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_image_entry_point(
	image_handle: *const c_void,
	system_table: *const SystemTable,
) {
	init(unsafe { system_table.as_ref() }.expect("system_table is null",),)
		.expect("failed to initialized application",);
	app().expect("error arise while executing application",);
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfe");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}

#[panic_handler]
fn panic(panic: &core::panic::PanicInfo,) -> ! {
	loop {
		unsafe {
			#[cfg(target_arch = "aarch64")]
			asm!("wfi");
			#[cfg(target_arch = "x86_64")]
			asm!("hlt");
		}
	}
}

fn app() -> Rslt<u64,> {
	// println!("hello");
	// println!("{}", 2525);

	Ok(0,)
}

// /// `efi_main`でのエラー処理を楽にする為に、処理中に投げられたResult::Errをここで一度吸収する
// fn app() -> Result<(u64, FrameBufConf,), OsoLoaderError,> {
// 	debug!("load kernel");
// 	let kernel_addr = load_kernel()?;
//
// 	debug!("load graphic configuration");
// 	let frame_buf_conf = load_graphic_config()?;
//
// 	Ok((kernel_addr, frame_buf_conf,),)
// }
//
// const KERNEL_NAME: &str = "oso_kernel.elf\0";
//
// /// # Return
// ///
// /// カーネルファイルの内容が置かれているメモリ領域の先頭アドレスを返します
// fn load_kernel() -> Result<u64, OsoLoaderError,> {
// 	let open_mode = file::FileMode::Read;
// 	let attributes = file::FileAttribute::empty();
//
// 	debug!("obtain kernel file handler");
// 	let mut kernel_file = sfs::open_file(KERNEL_NAME, open_mode, attributes,)?;
//
// 	debug!("read kernel file");
// 	let kernel_bytes = sfs::read_file_bytes(&mut kernel_file,)?;
//
// 	debug!("get kernel head address");
// 	let kernel_head = parse_elf(&kernel_bytes,)?;
//
// 	debug!("kernel address: 0x{kernel_head:x}");
// 	Ok(kernel_head,)
// }
//
// /// elf形式のカーネルファイルを読み込み、elfヘッダを解析してどう読み込めば良いかを決定する
// /// その後、実行可能バイナリを所定のアドレスに配置する
// fn parse_elf(kernel_bytes: &Vec<u8,>,) -> Result<u64, OsoLoaderError,> {
// 	debug!("parse kernel file");
// 	let elf_kernel = elf::Elf::parse(kernel_bytes,)?;
//
// 	// 何ページ分確保すれば良いか計算
// 	let (kernel_head, kernel_tail,) = oso_loader::elf::calc_elf_addr_range(&elf_kernel,);
// 	let page_count = required_pages(kernel_tail - kernel_head,);
// 	debug!("kernel head: {:#x}", kernel_head);
// 	debug!("kernel tail: {:#x}", kernel_tail);
//
// 	debug!("allocate for kernel program");
// 	let _alloc_head = boot::allocate_pages(
// 		boot::AllocateType::Address(kernel_head as u64,),
// 		MemoryType::LOADER_DATA,
// 		page_count,
// 	)?;
//
// 	copy_load_segment(&elf_kernel, kernel_bytes,);
//
// 	// entryフィールドはプログラムのエントリーポイント(asmで言う_start、
// 	// Cで言うmain)の仮想アドレスを指す
// 	Ok(elf_kernel.entry,)
// }
//
// fn load_graphic_config() -> Result<FrameBufConf, OsoLoaderError,> {
// 	debug!("obtain graphics output protocol");
// 	let mut gout = oso_loader::open_protocol_with::<GraphicsOutput,>()?;
//
// 	let mode_info = gout.current_mode_info();
// 	debug!("current graphic output mode: {mode_info:?}");
// 	let (width, height,) = mode_info.resolution();
// 	let stride = mode_info.stride();
//
// 	let pixel_format = match mode_info.pixel_format() {
// 		uefi::proto::console::gop::PixelFormat::Rgb => PixelFormatConf::Rgb,
// 		uefi::proto::console::gop::PixelFormat::Bgr => PixelFormatConf::Bgr,
// 		uefi::proto::console::gop::PixelFormat::Bitmask => PixelFormatConf::Bitmask,
// 		uefi::proto::console::gop::PixelFormat::BltOnly => PixelFormatConf::BltOnly,
// 	};
// 	debug!("pixel_format: {:?}", pixel_format);
//
// 	let base;
// 	// this code emits error because base is not initialized
// 	// let a = base;
// 	let size;
//
// 	if pixel_format == PixelFormatConf::BltOnly {
// 		let gout = gout.get().unwrap();
// 		let gout = gout as *const GraphicsOutput as *const GraphicsOutputProtocol;
// 		let blt = unsafe { (*gout).blt };
// 		let mode;
// 		unsafe {
// 			mode = *(*gout).mode;
// 		}
// 		base = mode.frame_buffer_base as *mut u8;
// 		size = mode.frame_buffer_size;
// 		// loop {
// 		// 	unsafe {
// 		// 		#[cfg(target_arch = "aarch64")]
// 		// 		asm!("wfi");
// 		// 		#[cfg(target_arch = "x86_64")]
// 		// 		asm!("hlt");
// 		// 	}
// 		// }
// 	} else {
// 		let mut fb = gout.frame_buffer();
// 		base = fb.as_mut_ptr();
// 		size = fb.size();
// 	}
//
// 	// loop {
// 	// 	unsafe {
// 	// 		asm!("wfi");
// 	// 	}
// 	// }
//
// 	let fbc = FrameBufConf::new(pixel_format, base, size, width, height, stride,);
// 	debug!("fbc: {fbc:?}");
// 	uefi::boot::stall(100_000_000,);
//
// 	Ok(fbc,)
// }
//
// fn exit_boot_services() {
// 	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
// 	core::mem::forget(mem_map,);
// }
//
// fn exec_kernel(fbc: FrameBufConf, kernel_addr: u64,) {
// 	#[cfg(target_arch = "aarch64")]
// 	let entry_point: extern "C" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	#[cfg(target_arch = "riscv64")]
// 	let entry_point: extern "C" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	#[cfg(target_arch = "x86_64")]
// 	let entry_point: extern "sysv64" fn(FrameBufConf,) =
// 		unsafe { core::mem::transmute(kernel_addr as usize,) };
// 	entry_point(fbc,);
// }
