use crate::error::OsoLoaderError;
use crate::string_to_cstr16;
use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use log::debug;
use log::error;
use uefi::proto::media::file;
use uefi::proto::media::file::File;
use uefi::proto::media::file::RegularFile;

pub fn open_file(
	file_name: impl AsRef<str,>,
	open_mode: file::FileMode,
	attributes: file::FileAttribute,
) -> Result<RegularFile, OsoLoaderError,> {
	// イメージのファイルシステムにおけるルートディレクトリを取得
	let mut root_dir = img_root_dir()?;
	assert!(root_dir.is_directory()?);

	string_to_cstr16!(file_name, filename);
	let file_handler = root_dir.open(filename, open_mode, attributes,)?;
	if file_handler.is_regular_file()? {
		Ok(file_handler
			.into_regular_file()
			.expect("failed to convert file handler as a regular file",),)
	} else {
		error!("file name is recognized as directory");
		Err(OsoLoaderError::Uefi("file name is recognized as directory".to_string(),),)
	}
}

pub fn read_file(file: &mut file::RegularFile,) -> Result<alloc::string::String, OsoLoaderError,> {
	//string_to_cstr16!(path, path);
	let content = read_file_bytes(file,)?;
	Ok(alloc::string::String::from_utf8_lossy(content.as_slice(),).into_owned(),)
}

pub fn read_file_bytes(
	file: &mut file::RegularFile,
) -> Result<alloc::vec::Vec<u8,>, OsoLoaderError,> {
	// ファイルのサイズを取得
	//let file_info = file.get_boxed_info::<file::FileInfo>()?;
	let file_info = file.get_boxed_info::<file::FileInfo>()?;
	let file_size = file_info.file_size() as usize;
	debug!("file_size: {file_size}");

	// 内容を書き込む為のバッファの確保
	let mut content = vec![0; file_size];
	let read_size = file.read(&mut content,)?;
	debug!("read_size: {read_size}");
	Ok(content,)
}

pub fn write_file(
	file: &mut file::RegularFile,
	content: alloc::string::String,
) -> Result<(), OsoLoaderError,> {
	// let open_mode = file::FileMode::CreateReadWrite;
	// let attributes = file::FileAttribute::empty();
	// let mut file = open_file(path, open_mode, attributes,)?
	// 	.into_regular_file()
	// 	.expect("path is recognized as directory",);

	let content = content.as_bytes();
	debug!("content len: {}", content.len());
	match file.write(content,) {
		Ok(_,) => Ok((),),
		Err(e,) => {
			let status = e.status();
			let data = e.data();
			assert_eq!(status.0, *data);
			Err(OsoLoaderError::Uefi(format!("failed to write to file: status code is {data}"),),)
		},
	}
}

/// イメージのルートディレクトリを取得
pub fn img_root_dir() -> Result<file::Directory, OsoLoaderError,> {
	// 読み込まれているイメージのファイルシステムを取得する
	let mut img_loaded_fs = super::img_fs()?;

	// イメージのルートディレクトリを取得
	let root_dir = img_loaded_fs.open_volume()?;
	Ok(root_dir,)
}
