#![no_std]
#![no_main]

extern crate alloc;

use core::ptr::NonNull;

use alloc::vec::Vec;
use byteorder::ByteOrder;
use goblin::elf;
use goblin::elf64;
use log::debug;
use log::info;
use oso_loader::fs::via_simple_filesystem as sfs;
use oso_loader::graphic;
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

	if let Err(e,) = app() {
		oso_loader::on_error!(e, "while executing app()");
		uefi::boot::stall(10_000_000,);
	}

	Status::SUCCESS
}

/// `efi_main`でのエラー処理を楽にする為に、処理中に投げられたResult::Errをここで一度吸収する
fn app() -> uefi::Result {
	// oso_loader::clear_stdout();
	// inspector()?;

	//oso_loader::clear_stdout();
	//graphic::fill_with(0xff, 0xff, 0xff,)?;
	//graphic::draw_sierpinski()?;

	//oso_loader::clear_stdout();

	debug!("load kernel");
	let kernel_position = load_kernel()?;

	debug!("load graphic configuration to pass kernel");
	let frame_buf_conf = load_graphic_config()?;

	debug!("exit boot services");
	exit_boot_services();
	exec_kernel(frame_buf_conf,);
	Ok((),)
}

const KERNEL_BASE_ADDR: u64 = 0x100_000;
const KERNEL_NAME: &str = "oso_kernel.elf\0";

/// カーネルファイルをVec<u8>として読み込む
///
/// # Return
///
/// カーネルファイルの内容が置かれているメモリ領域の先頭アドレスを返します
fn load_kernel() -> uefi::Result<NonNull<u8,>,> {
	let open_mode = file::FileMode::Read;
	let attributes = file::FileAttribute::empty();

	debug!("obtain kernel file handler");
	let mut kernel_file = sfs::open_file(KERNEL_NAME, open_mode, attributes,)?;

	debug!("read kernel file");
	let kernel_bytes = sfs::read_file_bytes(&mut kernel_file,)?;

	debug!("parse elf header & calculate load address range for kernel");
	parse_elf(&kernel_bytes,)
}

/// elf形式のカーネルファイルを読み込み、elfヘッダを解析してどう読み込めば良いかを決定する
fn parse_elf(kernel_bytes: &Vec<u8,>,) -> uefi::Result<NonNull<u8,>,> {
	let elf = elf::Elf::parse(kernel_bytes,)?;
	let (kernel_head, kernel_tail,) = oso_loader::elf::calc_kernel_address(&elf,);
	todo!()
}

fn load_graphic_config() -> uefi::Result<FrameBufConf,> {
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
	debug!("exit boot services");
	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
	core::mem::forget(mem_map,);
	//todo!()
}

fn exec_kernel(fbc: FrameBufConf,) {
	let entry_addr = unsafe {
		// NOTE: lenになぜ8を指定しているのか
		// → `extern "sysv64" fn()`のサイズが8だから？
		core::slice::from_raw_parts((KERNEL_BASE_ADDR + 24) as *const u8, 8,)
	};
	let entry_addr = byteorder::LittleEndian::read_u64(entry_addr,);
	let entry_point: extern "sysv64" fn(FrameBufConf,) =
		unsafe { core::mem::transmute(entry_addr as usize,) };
	entry_point(fbc,);
}

/// 起動時にログをいくつか表示
fn inspector() -> uefi::Result {
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
	let mut file = sfs::open_file(file_name, open_mode, attributes,)?
		.into_regular_file()
		.expect("given path is recognized as directory",);
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

fn inspect_memory_map(mem_type: &MemoryType,) -> uefi::Result {
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
