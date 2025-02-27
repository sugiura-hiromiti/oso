#![no_std]
#![no_main]

extern crate alloc;

use log::debug;
use log::info;
use oso_loader::fs::via_simple_filesystem as sfs;
use uefi::Status;
use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::media::file;
use uefi::proto::media::file::File;

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
	oso_loader::clear_stdout();

	//oso::draw_sierpinski()?;

	oso_loader::clear_stdout();
	inspector()?;

	oso_loader::clear_stdout();
	//oso::string_to_cstr16!("\\mem_map", filename);
	let file_name = "\\mem_map\0";
	let open_mode = file::FileMode::CreateReadWrite;
	let attributes = file::FileAttribute::empty();
	let mut file = sfs::open_file(file_name, open_mode, attributes,)?
		.into_regular_file()
		.expect("given path is recognized as directory",);
	let mem_map = oso_loader::get_memory_map(&MemoryType(0,),)?;
	let _ = oso_loader::save_mamory_map(&mem_map, file_name,);
	let content = sfs::read_file(&mut file,)?;
	debug!("content len is {}", content.len());
	debug!("content is {}", content);

	file.close();
	loop {}
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

	Ok((),)
}

fn inspect_memory_map(mem_type: &MemoryType,) -> uefi::Result {
	let mem_type = oso_loader::get_memory_map(mem_type,)?;

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
