#[repr(C)]
#[derive(Debug, PartialEq, Eq,)]
pub enum PixelFormatConf {
	Rgb,
	Bgr,
	Bitmask,
	BltOnly,
}

/// NOTE: not useful until implemnt allocator
// impl<D: Draw,> PixelFormatConf {
// 	fn convert(self,) -> impl Draw {
// 		match self {
// 			PixelFormatConf::Rgb => Rgb,
// 			PixelFormatConf::Bgr => Bgr,
// 			PixelFormatConf::Bitmask => Bitmask,
// 			PixelFormatConf::BltOnly => BltOnly,
// 		}
// 	}
// }

/// Rustには安定化されたABIが存在しないため、カーネルのエントリーポイントは `sysv64`
/// abiでコンパイルされる
/// `FrameBufConf`はカーネルのエントリーポイントが受けとる引数の型として使われる為
/// `#[repr(C)]`属性が必要
#[derive(Debug,)]
#[repr(C)]
pub struct FrameBufConf {
	pub pixel_format: PixelFormatConf,
	pub base:         *mut u8,
	pub size:         usize,
	pub width:        usize,
	pub height:       usize,
	pub stride:       usize,
}

impl FrameBufConf {
	pub fn new(
		pixel_format: PixelFormatConf,
		base: *mut u8,
		size: usize,
		width: usize,
		height: usize,
		stride: usize,
	) -> Self {
		Self { pixel_format, size, base, width, height, stride, }
	}
}
