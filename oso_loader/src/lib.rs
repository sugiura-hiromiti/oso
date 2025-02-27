#![no_std]
#![no_main]

pub mod fs;

extern crate alloc;

use alloc::format;
use fs::via_simple_filesystem as sfs;
use log::info;
use log::trace;
use uefi::Identify;
use uefi::boot;
use uefi::mem::memory_map::MemoryMap;
use uefi::mem::memory_map::MemoryMapOwned;
use uefi::proto;
use uefi::proto::console::gop::BltOp;
use uefi::proto::console::gop::BltPixel;
use uefi::proto::console::gop::BltRegion;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::loaded_image;
use uefi::proto::media::file;
use uefi::proto::media::file::File;
use uefi::proto::rng::Rng;

/// bytes(not bit🫠) of volume of file system
const VOLUME_SIZE: usize = 16 * 1024; //1024 * 1024;

#[macro_export]
macro_rules! on_error {
	($e:ident, $situation:expr) => {{
		log::error!("error happen {}", $situation);
		log::error!("error msg:");
		log::error!("{}", $e);
	}};
}

#[macro_export]
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

/// メモリマップを取得
/// 実体は`uefi::boot::memory_map`のシンプルなラッパー
///
/// # Return
///
/// この関数はuefi::Result型の返り値を持ちます
/// メモリマップの取得に成功した場合はOk(MemoryMapOwned型の変数)を返します
pub fn get_memory_map(mem_type: &boot::MemoryType,) -> uefi::Result<MemoryMapOwned,> {
	let mem_map = boot::memory_map(*mem_type,)?;
	Ok(mem_map,)
}

/// 受け取った`mem_map`の内容を`path`で指定されたファイルに保存
///
/// # Return
pub fn save_mamory_map(mem_map: &MemoryMapOwned, path: impl AsRef<str,>,) -> uefi::Result {
	trace!("write memory map to file");
	let header = format!("Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n");
	trace!("{header}");

	let open_mode = file::FileMode::CreateReadWrite;
	let attributes = file::FileAttribute::empty();
	let mut file = sfs::open_file(path, open_mode, attributes,)?
		.into_regular_file()
		.expect("path seems directory",);

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

// pub fn save_memory_map(mem_map: &MemoryMapRpr, file: file::RegularFile,) {
// 	let header = cstr8!("Index,, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n");
// 	info!("write to file");
// 	info!("{header}");
//
// 	let mut i = 0;
//
// 	todo!()
// }

pub fn print_image_path() -> uefi::Result {
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

	trace!("Image path: {}", &*image_device_path_text);

	uefi::boot::stall(2_000_000,);
	Ok((),)
}

#[derive(Clone,)]
struct Point {
	x: f32,
	y: f32,
}

impl Point {
	fn new(x: f32, y: f32,) -> Self {
		Self { x, y, }
	}
}

struct Buffer {
	width:  usize,
	height: usize,
	pixels: alloc::vec::Vec<BltPixel,>,
}

impl Buffer {
	fn new(width: usize, height: usize,) -> Self {
		Self { width, height, pixels: alloc::vec![BltPixel::new(0, 0, 0); height * width], }
	}

	/// 特定の1点を取得する
	fn pixel(&mut self, x: usize, y: usize,) -> Option<&mut BltPixel,> {
		self.pixels.get_mut(y * self.width + x,)
	}

	/// メモリをバイト毎にコピーする
	/// bltはblitとも呼ばれる
	/// この呼びはDEC PDP-10コンピュータのBLT命令に由来する
	/// (block transferの意味)
	///
	/// bufferをframebufferに移動させる
	fn blit(&self, gout: &mut GraphicsOutput,) -> uefi::Result {
		gout.blt(BltOp::BufferToVideo {
			buffer: &self.pixels,
			src:    BltRegion::Full,
			dest:   (0, 0,),
			dims:   (self.width, self.height,),
		},)
	}

	/// 特定の1点のみをframebufferに転送する
	fn blit_pixel(&self, gout: &mut GraphicsOutput, coord: (usize, usize,),) -> uefi::Result {
		gout.blt(BltOp::BufferToVideo {
			buffer: &self.pixels,
			src:    BltRegion::SubRectangle { coords: coord, px_stride: self.width, },
			dest:   coord,
			dims:   (1, 1,),
		},)
	}
}

fn open_protocol_with<P: uefi::proto::ProtocolPointer + ?Sized,>()
-> uefi::Result<boot::ScopedProtocol<P,>,> {
	let hndlr = boot::get_handle_for_protocol::<P,>()?;
	boot::open_protocol_exclusive::<P,>(hndlr,)
}

/// Get a random `usize` value
fn get_randowm_usize(rng: &mut Rng,) -> usize {
	let mut buf = [0; size_of::<usize,>()];
	rng.get_rng(None, &mut buf,).expect("get_rng failed",);
	usize::from_le_bytes(buf,)
}

/// sierpinski図形を描く
pub fn draw_sierpinski() -> uefi::Result {
	trace!("entered draw_sierpinski");
	// graphics output protocolを開く
	let mut gout = open_protocol_with::<GraphicsOutput,>()?;
	trace!("open graphics output protocol");
	// random number generator protocolを開く
	let mut rng = open_protocol_with::<Rng,>()?;
	trace!("open random number generator protocol");

	// 描画のためのバッファを作成する
	let (width, height,) = gout.current_mode_info().resolution();
	let mut buf = Buffer::new(width, height,);
	trace!("cretae buffor for render");

	// シンプルなグラデーション背景で初期化
	for y in 0..height {
		let r = ((y as f32) / ((height - 1) as f32)) * 255.0;
		for x in 0..width {
			let g = ((x as f32) / ((width - 1) as f32)) * 255.0;
			let pixel = buf.pixel(x, y,).unwrap();
			pixel.red = r as u8;
			pixel.green = g as u8;
			pixel.blue = 255;
		}
	}

	// 背景をセット
	buf.blit(&mut gout,)?;

	let size = Point::new(width as f32, height as f32,);

	let border = 20.0;
	let triangle = [
		Point::new(size.x / 2.0, border,),
		Point::new(border, size.y - border,),
		Point::new(size.x - border, size.y - border,),
	];

	let mut p = Point::new(size.x / 2.0, size.y / 2.0,);

	loop {
		// 三角形の頂点をランダムに選ぶ
		let v = triangle[get_randowm_usize(&mut rng,) % 3].clone();
		p.x = (p.x + v.x) * 0.5;
		p.y = (p.y + v.y) * 0.5;

		let pixel = buf.pixel(p.x as usize, p.y as usize,).unwrap();
		pixel.red = 0;
		pixel.green = 0;
		pixel.blue = 0;

		buf.blit_pixel(&mut gout, (p.x as usize, p.y as usize,),)?;
	}
}
