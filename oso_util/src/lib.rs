#![no_std]

/// pixel_format value represents as below
///
/// Rgb => 0,
/// Bgr => 1,
/// Bitmask => 2,
/// BltOnly => 3,
pub struct FrameBufConf {
	pixel_format: u8,
	base:         usize,
	width:        usize,
	height:       usize,
	stride:       usize,
}

impl FrameBufConf {
	pub fn new(pixel_format: u8, base: usize, width: usize, height: usize, stride: usize,) -> Self {
		Self { pixel_format, base, width, height, stride, }
	}
}
