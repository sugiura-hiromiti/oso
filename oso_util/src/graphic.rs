use core::usize;
use helper::DrawMarker;
use helper::FrameBufferBuilder;

mod helper;

#[repr(C)]
#[derive(Debug, PartialEq, Eq,)]
pub enum PixelFormat {
	Rgb,
	Bgr,
	Bitmask,
	BltOnly,
}

/// Rustには安定化されたABIが存在しないため、カーネルのエントリーポイントは `sysv64`
/// abiでコンパイルされる
/// `FrameBufConf`はカーネルのエントリーポイントが受けとる引数の型として使われる為
/// `#[repr(C)]`属性が必要
#[derive(Debug,)]
#[repr(C)]
pub struct FrameBufConf {
	pub pixel_format: PixelFormat,
	pub base:         *mut u8,
	pub size:         usize,
	pub width:        usize,
	pub height:       usize,
	pub stride:       usize,
}

impl FrameBufConf {
	pub fn new(
		pixel_format: PixelFormat,
		base: *mut u8,
		size: usize,
		width: usize,
		height: usize,
		stride: usize,
	) -> Self {
		Self { pixel_format, size, base, width, height, stride, }
	}
}

pub struct Rgb;
impl Draw for Rgb {
	fn draw_type(&self,) -> &str {
		"Rgb"
	}
}

pub struct Bgr;
impl Draw for Bgr {
	fn draw_type(&self,) -> &str {
		"Bgr"
	}

	fn put_pixel(&self, coord: &Coord, color: &Color,) {
		todo!()
	}
}

pub struct Bitmask;
impl Draw for Bitmask {
	fn draw_type(&self,) -> &str {
		"Bitmask"
	}
}

pub struct BltOnly;
impl Draw for BltOnly {
	fn draw_type(&self,) -> &str {
		"BltOnly"
	}
}

pub struct Coord {
	x: usize,
	y: usize,
}

pub struct Color {
	red:   u8,
	green: u8,
	blue:  u8,
}

pub trait Draw {
	fn draw_type(&self,) -> &str;
	fn put_pixel(&self, coord: &Coord, color: &Color,) {
		let (..,) = (coord, color,);
		todo!("put_pixel is not implemented for `{}` pixel format", self.draw_type());
	}
}

pub struct FrameBuffer<'a, T: DrawMarker,> {
	drawer: T::Drawer,
	buf:    &'a mut [u8],
	width:  usize,
	height: usize,
	stride: usize,
}

impl<'a, T: DrawMarker,> FrameBuffer<'a, T,> {
	pub fn new(conf: FrameBufConf,) -> Self {
		let buf = unsafe { core::slice::from_raw_parts_mut(conf.base, conf.size,) };
		Self { drawer: todo!(), buf, width: conf.width, height: conf.height, stride: conf.stride, }
	}

	pub fn put_pixel(coord: &Coord, color: &Color,) {}
}
