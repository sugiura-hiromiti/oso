use crate::string_to_cstr16;
use alloc::vec;
use log::debug;
use log::error;
use uefi::Status;
use uefi::proto::media::file;
use uefi::proto::media::file::File;
use uefi::proto::media::file::FileHandle;

pub fn open_file(
	file_name: impl AsRef<str,>,
	open_mode: file::FileMode,
	attributes: file::FileAttribute,
) -> uefi::Result<FileHandle,> {
	// イメージのファイルシステムにおけるルートディレクトリを取得
	let mut root_dir = img_root_dir()?;
	assert!(root_dir.is_directory()?);

	string_to_cstr16!(file_name, filename);
	let file_handler = root_dir.open(filename, open_mode, attributes,)?;
	if file_handler.is_regular_file()? {
		Ok(file_handler,)
	} else {
		error!("file name is recognized as directory");
		Err(uefi::Error::new(Status::ABORTED, (),),)
	}
}

pub fn read_file(file: &mut file::RegularFile,) -> uefi::Result<alloc::string::String,> {
	//string_to_cstr16!(path, path);
	// ファイルのサイズを取得
	//let file_info = file.get_boxed_info::<file::FileInfo>()?;
	let buf = &mut [0; 1024];
	let file_info: &mut file::FileInfo =
		file.get_info(buf,).expect("error happen while obtaining file info",);
	let file_size = file_info.file_size() as usize;
	debug!("file_size: {file_size}");

	// 内容を書き込む為のバッファの確保
	let mut content = vec![0; file_size];
	let content = content.as_mut_slice();
	debug!("content.len(): {}", content.len());
	let read_size = file.read(content,)?;
	debug!("read_size: {read_size}");

	//assert_eq!(file_size, file.read(content)?);

	Ok(alloc::string::String::from_utf8_lossy(content,).into_owned(),)
}

pub fn write_file(file: &mut file::RegularFile, content: alloc::string::String,) -> uefi::Result {
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
			Err(uefi::Error::new(status, (),),)
		},
	}
}

/// イメージのルートディレクトリを取得
pub fn img_root_dir() -> uefi::Result<file::Directory,> {
	// 読み込まれているイメージのファイルシステムを取得する
	let mut img_loaded_fs = super::img_fs()?;

	// イメージのルートディレクトリを取得
	let root_dir = img_loaded_fs.open_volume()?;
	Ok(root_dir,)
}
