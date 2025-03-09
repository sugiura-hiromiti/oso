#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use byteorder::ByteOrder;
use core::arch::asm;
use core::usize;
use goblin::elf;
use log::debug;
use log::info;
use oso_loader::elf::copy_load_segment;
use oso_loader::error::OsoLoaderError;
use oso_loader::fs::via_simple_filesystem as sfs;
use oso_loader::memory::required_pages;
use oso_util::FrameBufConf;
use uefi::Status;
use uefi::boot;
use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file;
use uefi::proto::media::file::File;
use uefi::proto::media::file::FileInfo;

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

	// loop {
	// 	unsafe {
	// 		asm!("hlt");
	// 	}
	// }
	Status::SUCCESS
}

/// `efi_main`でのエラー処理を楽にする為に、処理中に投げられたResult::Errをここで一度吸収する
fn app() -> Result<(u64, FrameBufConf,), OsoLoaderError,> {
	// oso_loader::clear_stdout();
	// inspector()?;

	//oso_loader::clear_stdout();
	//graphic::fill_with(0xff, 0xff, 0xff,)?;
	//graphic::draw_sierpinski()?;

	oso_loader::clear_stdout();

	debug!("load kernel");
	let kernel_addr = load_kernel()? /* + 24 */;
	debug!("start address of kernel: 0x{kernel_addr:x}");

	debug!("load graphic configuration");
	let frame_buf_conf = load_graphic_config()?;
	debug!("frame_buf_conf: {frame_buf_conf:?}");

	Ok((kernel_addr, frame_buf_conf,),)
}

const KERNEL_NAME: &str = "oso_kernel.elf\0";

/// カーネルファイルをVec<u8>として読み込む
///
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

	debug!("parse elf header & calculate load address range for kernel");
	let hernel_head = parse_elf(&kernel_bytes,)?;
	Ok(hernel_head,)
}

/// elf形式のカーネルファイルを読み込み、elfヘッダを解析してどう読み込めば良いかを決定する
/// その後、実行可能バイナリを所定のアドレスに配置する
fn parse_elf(kernel_bytes: &Vec<u8,>,) -> Result<u64, OsoLoaderError,> {
	let elf_kernel = elf::Elf::parse(kernel_bytes,)?;

	// 何ページ分確保すれば良いか計算
	let (kernel_head, kernel_tail,) = oso_loader::elf::calc_elf_addr_range(&elf_kernel,);
	let page_count = required_pages(kernel_tail - kernel_head,);

	let _alloc_head = boot::allocate_pages(
		boot::AllocateType::Address(kernel_head as u64,),
		MemoryType::LOADER_DATA,
		page_count,
	)?;

	copy_load_segment(&elf_kernel, kernel_head, kernel_bytes,);

	// カーネルプログラムをコピーし終わったので確保したメモリ領域を開放する
	// unsafe { boot::free_pages(alloc_head, page_count,) }?;

	// entryフィールドはプログラムのエントリーポイント(asmで言う_start、
	// Cで言うmain)の仮想アドレスを指す
	debug!("entry point address of kernel: {:x}", elf_kernel.entry);
	Ok(elf_kernel.entry,)
}

fn load_graphic_config() -> Result<FrameBufConf, OsoLoaderError,> {
	debug!("obtain graphics output protocol");
	let mut gout = oso_loader::open_protocol_with::<GraphicsOutput,>()?;

	let base = gout.frame_buffer().as_mut_ptr() as usize;
	let mode_info = gout.current_mode_info();
	let (width, height,) = mode_info.resolution();
	let stride = mode_info.stride();
	let pixel_format = match mode_info.pixel_format() {
		uefi::proto::console::gop::PixelFormat::Rgb => 0,
		uefi::proto::console::gop::PixelFormat::Bgr => 1,
		uefi::proto::console::gop::PixelFormat::Bitmask => 2,
		uefi::proto::console::gop::PixelFormat::BltOnly => 3,
	};

	let fbc = FrameBufConf::new(pixel_format, base, width, height, stride,);
	Ok(fbc,)
}

fn exit_boot_services() {
	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
	core::mem::forget(mem_map,);
}

fn exec_kernel(fbc: FrameBufConf, kernel_addr: u64,) {
	// let entry_addr = unsafe {
	// 	// NOTE: lenになぜ8を指定しているのか
	// 	// → `extern "sysv64" fn()`のサイズが8だから？
	// 	core::slice::from_raw_parts((kernel_addr) as *const u8, 8,)
	// };
	// let entry_addr = byteorder::LittleEndian::read_u64(entry_addr,);
	//
	// let entry_point: extern "sysv64" fn(FrameBufConf,) =
	// 	unsafe { core::mem::transmute(entry_addr as usize,) };
	let entry_point: extern "sysv64" fn(FrameBufConf,) =
		unsafe { core::mem::transmute(kernel_addr as usize,) };
	entry_point(fbc,);
}

/// 起動時にログをいくつか表示
fn inspector() -> Result<(), OsoLoaderError,> {
	oso_loader::clear_stdout();
	debug!("oso is 0w0");

	// stdoutの情報を表示
	uefi::system::with_stdout(|o| {
		info!("called stdout");
		o.modes().for_each(|mode| info!("output mode: {mode:#?}"),);
	},);
	uefi::system::with_stdout(|o| {
		info!("current mode is: {:#?}", o.current_mode());
	},);
	let revision = uefi::system::uefi_revision();
	info!("uefi revision is: {revision}");
	uefi::boot::stall(2_000_000,);

	oso_loader::clear_stdout();
	oso_loader::print_image_path()?;

	oso_loader::clear_stdout();
	// メモリマップの表示
	let mem_types = &[
		MemoryType::RESERVED,
		MemoryType::LOADER_CODE,
		MemoryType::LOADER_DATA,
		MemoryType(0,),
		MemoryType(1,),
	];
	mem_types.iter().for_each(|mty| {
		if let Err(e,) = inspect_memory_map(mty,) {
			oso_loader::on_error!(e, alloc::format!("failed to get memory_map of type: {mty:?}"));
		}
	},);

	let file_name = "\\mem_map\0";
	let open_mode = file::FileMode::CreateReadWrite;
	let attributes = file::FileAttribute::empty();
	let mut file = sfs::open_file(file_name, open_mode, attributes,)?;
	let mem_map = oso_loader::memory::get_memory_map(&MemoryType(0,),)?;
	let _ = oso_loader::memory::save_mamory_map(&mem_map, file_name,);
	let content = sfs::read_file(&mut file,)?;

	debug!("content len is {}", content.len());
	debug!("content is {}", content);
	let info: alloc::boxed::Box<FileInfo,> = file.get_boxed_info()?;

	oso_loader::clear_stdout();
	debug!("fileinfo: {info:?}");

	file.close();

	Ok((),)
}

fn inspect_memory_map(mem_type: &MemoryType,) -> Result<(), OsoLoaderError,> {
	let mem_type = oso_loader::memory::get_memory_map(mem_type,)?;

	let buf = mem_type.buffer();
	debug!("______________________________");
	debug!("mem type buf address: {:p}", buf);
	debug!("mem type buf_len: {:?}", buf.len());
	debug!("mem type map_size: {:?}", mem_type.meta().map_size);
	debug!("mem type desc_size: {:?}", mem_type.meta().desc_size);
	debug!("mem type map_key: {:?}", mem_type.meta().map_key);
	debug!("mem type desc_version: {:?}", mem_type.meta().desc_version);
	debug!("mem type len: {:?}", mem_type.len());

	Ok((),)
}
