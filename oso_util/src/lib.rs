#![no_std]

/// pixel_format value represents as below
///
/// Rgb => 0,
/// Bgr => 1,
/// Bitmask => 2,
/// BltOnly => 3,
#[derive(Debug,)]
pub struct FrameBufConf {
	pub pixel_format: u8,
	pub base:         usize,
	pub width:        usize,
	pub height:       usize,
	pub stride:       usize,
}

impl FrameBufConf {
	pub fn new(pixel_format: u8, base: usize, width: usize, height: usize, stride: usize,) -> Self {
		Self { pixel_format, base, width, height, stride, }
	}
}
