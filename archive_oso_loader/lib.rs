#![no_std]
#![feature(alloc_error_handler)]
#![feature(ptr_as_ref_unchecked)]

pub mod chibi_uefi;
pub mod elf;
pub mod error;
pub mod fs;
pub mod graphic;
pub mod memory;
pub mod mmio;
pub mod raw;

extern crate alloc;

use core::fmt::Debug;
use error::OsoLoaderError;
use log::debug;
use log::info;
use uefi::Identify;
use uefi::boot;
use uefi::boot::OpenProtocolParams;
use uefi::proto;
use uefi::proto::loaded_image;

type Rslt<T = (),> = Result<T, OsoLoaderError,>;

#[macro_export]
/// ?演算子で処理できないエラーがあった場合に使う
macro_rules! on_error {
	($e:ident, $situation:expr) => {{
		log::error!("error happen {}", $situation);
		log::error!("error msg:");
		log::error!("{}", $e);
	}};
}

#[macro_export]
/// `AsRef<str>`を実装する型の変数をuefi::CStr16型へ変換する
/// 所有権の問題で関数ではなくマクロになっている
macro_rules! string_to_cstr16 {
	($str:expr, $rslt:ident) => {
		//let $rslt = alloc::string::ToString::to_string($string,);
		let $rslt = $str.as_ref();
		let $rslt: alloc::vec::Vec<u16,> = $rslt.chars().map(|c| c as u16,).collect();
		let $rslt = match uefi::CStr16::from_u16_with_nul(&$rslt[..],) {
			Ok(cstr16,) => cstr16,
			Err(e,) => {
				log::error!("{:?}", e);
				panic!(
					"failed to convert &[u16] to CStr16\ninvalid code may included or not null \
					 terminated",
				);
			},
		};
	};
}

/// 画面をクリア
pub fn clear_stdout() {
	uefi::system::with_stdout(|o| {
		if let Err(e,) = o.clear() {
			info!("display clearing failed\nError is: {e}");
		}
	},);
}

/// uefiで読み込まれているイメージを探りイメージファイルへのパスを表示する
pub fn print_image_path() -> Result<(), OsoLoaderError,> {
	// イメージがどこにあるかを探るアプリケーション
	let loaded_image =
		boot::open_protocol_exclusive::<loaded_image::LoadedImage,>(boot::image_handle(),)?;

	// device_path型をテキストに変換するアプリケーションのハンドラ
	let device_path_to_text_handle = *boot::locate_handle_buffer(boot::SearchType::ByProtocol(
		&proto::device_path::text::DevicePathToText::GUID,
	),)?
	.first()
	.expect("DevicePathToText is missing",);

	// device_pathをテキストに変換するアプリケーション
	let device_path_to_text = boot::open_protocol_exclusive::<
		proto::device_path::text::DevicePathToText,
	>(device_path_to_text_handle,)?;

	let image_device_path = loaded_image.file_path().expect("file path is not set",);
	let image_device_path_text = device_path_to_text
		.convert_device_path_to_text(
			image_device_path,
			proto::device_path::text::DisplayOnly(true,),
			proto::device_path::text::AllowShortcuts(false,),
		)
		.expect("convert_device_path_to_text failed",);

	debug!("Image path: {}", &*image_device_path_text);

	uefi::boot::stall(2_000_000,);
	Ok((),)
}

pub fn open_protocol_with<P: uefi::proto::ProtocolPointer + ?Sized + Debug,>()
-> Result<boot::ScopedProtocol<P,>, OsoLoaderError,> {
	debug!("open handler");
	let handle = boot::get_handle_for_protocol::<P,>()?;
	let img_hndl = boot::image_handle();
	let params = OpenProtocolParams { handle, agent: img_hndl, controller: None, };
	let attributes = boot::OpenProtocolAttributes::GetProtocol;

	debug!("opened handler");
	let proto = unsafe { boot::open_protocol::<P,>(params, attributes,) }?;
	debug!("opened proto");

	// let proto = boot::open_protocol_exclusive::<P,>(handle,);
	// debug!("opened proto");
	Ok(proto,)
}
