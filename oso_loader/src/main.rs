#![no_std]
#![no_main]

extern crate alloc;

use byteorder::ByteOrder;
use log::debug;
use log::info;
use oso_loader::fs::via_simple_filesystem as sfs;
use oso_loader::graphic;
use uefi::Status;
use uefi::boot;
use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;
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
	//oso_loader::clear_stdout();
	//oso::draw_sierpinski()?;

	// oso_loader::clear_stdout();
	// inspector()?;

	oso_loader::clear_stdout();
	graphic::fill_with(0xff, 0xff, 0xff,)?;
	//graphic::draw_sierpinski()?;

	oso_loader::clear_stdout();
	load_kernel()?;
	exit_boot_services();
	exec_kernel();
	Ok((),)
}

const KERNEL_BASE_ADDR: u64 = 0x100_000;
const KERNEL_NAME: &str = "oso_kernel.elf\0";

/// カーネルファイルをメモリに読み込む
fn load_kernel() -> uefi::Result {
	let open_mode = file::FileMode::Read;
	let attributes = file::FileAttribute::empty();

	let mut file = sfs::open_file(KERNEL_NAME, open_mode, attributes,)?
		.into_regular_file()
		.expect("file name is recognized as directory. abort",);

	let file_info = file.get_boxed_info::<FileInfo>()?;
	let file_size = file_info.file_size() as usize;
	let page_count = 1 + file_size / uefi::boot::PAGE_SIZE;

	// TODO: ハードウェアによって、0x100_000番地が空いてない事がある
	// その場合はメモリマップを確認し、EfiConventionalMemoryとなっている十分な大きさの領域を探す
	let ptr = uefi::boot::allocate_pages(
		uefi::boot::AllocateType::Address(KERNEL_BASE_ADDR,),
		MemoryType::LOADER_DATA,
		page_count,
	)?;
	debug!("ptr is {ptr:#?}");

	// `KERNEL_BASE_ADDR`が指すメモリアドレスに、カーネルファイルの内容を展開
	let read_size = file.read(unsafe {
		core::slice::from_raw_parts_mut(KERNEL_BASE_ADDR as *mut u8, file_size,)
	},)?;
	debug!("read_size is {read_size}");

	file.close();
	Ok((),)
}

fn exit_boot_services() {
	debug!("exit boot services");
	let mem_map = unsafe { boot::exit_boot_services(MemoryType::BOOT_SERVICES_DATA,) };
	core::mem::forget(mem_map,);
	//todo!()
}

fn exec_kernel() {
	let entry_addr = unsafe {
		// NOTE: lenになぜ8を指定しているのか
		// → `extern "sysv64" fn()`のサイズが8だから？
		core::slice::from_raw_parts((KERNEL_BASE_ADDR + 24) as *const u8, 8,)
	};
	let entry_addr = byteorder::LittleEndian::read_u64(entry_addr,);
	let entry_point: extern "sysv64" fn() = unsafe { core::mem::transmute(entry_addr as usize,) };
	entry_point();
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
