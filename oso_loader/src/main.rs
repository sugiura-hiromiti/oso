#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::usize;
use goblin::elf;
use log::debug;
use oso_bridge::graphic::FrameBufConf;
use oso_bridge::graphic::PixelFormatConf;
use oso_loader::elf::copy_load_segment;
use oso_loader::error::OsoLoaderError;
use oso_loader::fs::via_simple_filesystem as sfs;
use oso_loader::memory::required_pages;
use uefi::Status;
use uefi::boot;
use uefi::boot::MemoryType;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file;

#[uefi::entry]
fn efi_main() -> Status {
	uefi::helpers::init().unwrap();

	let (kernel_addr, frame_buf_conf,) = match app() {
		Ok(rslt,) => rslt,
		Err(e,) => {
			oso_loader::on_error!(e, "while executing app()");
			uefi::boot::stall(10_000_000,);
			return Status::ABORTED;
		},
	};
	debug!("exit boot services");
	exit_boot_services();
	exec_kernel(frame_buf_conf, kernel_addr,);
	Status::SUCCESS
}

/// `efi_main`でのエラー処理を楽にする為に、処理中に投げられたResult::Errをここで一度吸収する
fn app() -> Result<(u64, FrameBufConf,), OsoLoaderError,> {
	debug!("load kernel");
	let kernel_addr = load_kernel()?;

	debug!("load graphic configuration");
	let frame_buf_conf = load_graphic_config()?;

	Ok((kernel_addr, frame_buf_conf,),)
}

const KERNEL_NAME: &str = "oso_kernel.elf\0";

/// # Return
///
/// カーネルファイルの内容が置かれているメモリ領域の先頭アドレスを返します
fn load_kernel() -> Result<u64, OsoLoaderError,> {
	let open_mode = file::FileMode::Read;
	let attributes = file::FileAttribute::empty();

	debug!("obtain kernel file handler");
	let mut kernel_file = sfs::open_file(KERNEL_NAME, open_mode, attributes,)?;

	debug!("read kernel file");
	let kernel_bytes = sfs::read_file_bytes(&mut kernel_file,)?;

	debug!("get kernel head address");
	let kernel_head = parse_elf(&kernel_bytes,)?;

	debug!("kernel address: 0x{kernel_head:x}");
	Ok(kernel_head,)
}

/// elf形式のカーネルファイルを読み込み、elfヘッダを解析してどう読み込めば良いかを決定する
/// その後、実行可能バイナリを所定のアドレスに配置する
fn parse_elf(kernel_bytes: &Vec<u8,>,) -> Result<u64, OsoLoaderError,> {
	debug!("parse kernel file");
	let elf_kernel = elf::Elf::parse(kernel_bytes,)?;

	// 何ページ分確保すれば良いか計算
	let (kernel_head, kernel_tail,) = oso_loader::elf::calc_elf_addr_range(&elf_kernel,);
	let page_count = required_pages(kernel_tail - kernel_head,);

	debug!("allocate for kernel program");
	let _alloc_head = boot::allocate_pages(
		boot::AllocateType::Address(kernel_head as u64,),
		MemoryType::LOADER_DATA,
		page_count,
	)?;

	copy_load_segment(&elf_kernel, kernel_bytes,);

	// entryフィールドはプログラムのエントリーポイント(asmで言う_start、
	// Cで言うmain)の仮想アドレスを指す
	Ok(elf_kernel.entry,)
}

fn load_graphic_config() -> Result<FrameBufConf, OsoLoaderError,> {
	debug!("obtain graphics output protocol");
	let mut gout = oso_loader::open_protocol_with::<GraphicsOutput,>()?;

	let mode_info = gout.current_mode_info();
	let (width, height,) = mode_info.resolution();
	let stride = mode_info.stride();
	let pixel_format = match mode_info.pixel_format() {
		uefi::proto::console::gop::PixelFormat::Rgb => PixelFormatConf::Rgb,
		uefi::proto::console::gop::PixelFormat::Bgr => PixelFormatConf::Bgr,
		uefi::proto::console::gop::PixelFormat::Bitmask => PixelFormatConf::Bitmask,
		uefi::proto::console::gop::PixelFormat::BltOnly => PixelFormatConf::BltOnly,
	};
	let mut fb = gout.frame_buffer();
	let base = fb.as_mut_ptr();
	let size = fb.size();

	let fbc = FrameBufConf::new(pixel_format, base, size, width, height, stride,);
	debug!("fbc: {fbc:?}");
	Ok(fbc,)
}

fn exit_boot_services() {
	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
	core::mem::forget(mem_map,);
}

fn exec_kernel(fbc: FrameBufConf, kernel_addr: u64,) {
	let entry_point: extern "sysv64" fn(FrameBufConf,) =
		unsafe { core::mem::transmute(kernel_addr as usize,) };
	entry_point(fbc,);
}
