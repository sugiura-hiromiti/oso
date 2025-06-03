use crate::error::OsoLoaderError;
use crate::fs::via_simple_filesystem as sfs;
use alloc::format;
use log::debug;
use uefi::boot;
use uefi::mem::memory_map::MemoryMap;
use uefi::mem::memory_map::MemoryMapOwned;
use uefi::proto::media::file;
use uefi::proto::media::file::File;

/// メモリマップを取得
/// 実体は`uefi::boot::memory_map`のシンプルなラッパー
///
/// # Return
///
/// この関数はuefi::Result型の返り値を持ちます
/// メモリマップの取得に成功した場合はOk(MemoryMapOwned型の変数)を返します
pub fn get_memory_map(mem_type: &boot::MemoryType,) -> Result<MemoryMapOwned, OsoLoaderError,> {
	let mem_map = boot::memory_map(*mem_type,)?;
	Ok(mem_map,)
}

/// 受け取った`mem_map`の内容を`path`で指定されたファイルにcsv形式で保存
///
/// # Return
pub fn save_mamory_map(
	mem_map: &MemoryMapOwned,
	path: impl AsRef<str,>,
) -> Result<(), OsoLoaderError,> {
	debug!("write memory map to file");
	let header = format!("Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n");
	debug!("{header}");

	let open_mode = file::FileMode::CreateReadWrite;
	let attributes = file::FileAttribute::empty();
	let mut file = sfs::open_file(path, open_mode, attributes,)?;

	sfs::write_file(&mut file, header,)?;
	for index in 0..mem_map.len() {
		let desc = mem_map[index];

		let mem_type = format!("{:#x}", desc.ty.0);
		let type_name = format!("{:?}", desc.ty);
		let physical_start = format!("{:#08x}", desc.phys_start);
		let number_of_pages = format!("{:#x}", desc.page_count);
		let attribute = format!("{:#x}", desc.att.bits() & 0xfffff);
		let buf = format!(
			"{index}, {mem_type}, {type_name}, {physical_start}, {number_of_pages}, {attribute}\n"
		);
		//trace!("{buf}");
		sfs::write_file(&mut file, buf,)?;
	}

	file.close();
	Ok((),)
}

/// 指定されたサイズのメモリを確保するためにメモリを何ページアロケートすれば良いのかを算出
pub fn required_pages(size: usize,) -> usize {
	size / uefi::boot::PAGE_SIZE + 1
}
