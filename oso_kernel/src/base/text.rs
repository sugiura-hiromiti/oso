//! this module provides interface for display text

use crate::base::graphic::color::PixelFormat;
use crate::base::graphic::position::Coordinal;
use crate::base::graphic::put_pixel;
use crate::error::KernelError;
use core::fmt::Write;
use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;
use oso_proc_macro::fonts_data;
use oso_proc_macro::impl_int;

// const SINONOME: &[u8; 256] = {
// 	let sinonome_font_txt = include_str!("../resource/sinonome_font.txt");
// 	let characters = &[0; 0x100];
//
// 	characters
// };

pub const SINONOME: &[u128; 256] = fonts_data!("resource/sinonome_font.txt");

pub const MAX_DIGIT: usize = 39;

pub struct TextBuf<C: Coordinal,> {
	init_pos:        C,
	row:             usize,
	col:             usize,
	pub font_width:  usize,
	pub font_height: usize,
}

impl<C: Coordinal,> TextBuf<C,> {
	pub fn new(init_pos: C, font_width: usize, font_height: usize,) -> Self {
		Self { init_pos, row: 0, col: 0, font_width, font_height, }
	}

	fn row_pixel(&self,) -> usize {
		self.init_pos.y() + self.font_height * self.row
	}

	fn col_pixel(&self,) -> usize {
		self.init_pos.x() + self.font_width * self.col
	}

	pub fn clear(&mut self,) {
		self.row = 0;
		self.col = 0;
	}
}

impl<C: Coordinal,> Write for TextBuf<C,> {
	fn write_str(&mut self, s: &str,) -> core::fmt::Result {
		// for c in s.as_bytes() {
		// 	self.write_char
		// }
	}
}

pub trait Text {
	fn put_char(&mut self, char: u8,) -> Result<(), KernelError,>;
	// fn write_str<C: Coordinal,>(
	// 	&mut self,
	// 	text: &str,
	// 	text_buf: &mut TextBuf<C,>,
	// ) -> Result<(), KernelError,> {
	// 	for c in text.as_bytes() {
	// 		self.write_char(*c, text_buf,)?;
	// 	}
	// 	Ok((),)
	// }
}

impl<C: Coordinal,> Text for TextBuf<C,> {
	fn put_char(&mut self, char: u8,) -> Result<(), KernelError,> {
		if char == b'\n' {
			self.row += 1;
			self.col = 0;
			return Ok((),);
		}

		let font_data = SINONOME[char as usize];
		let col_pos = self.col_pixel();
		let row_pos = self.row_pixel();

		for i in 0..self.font_width {
			for j in 0..self.font_height {
				let flag = i + j * self.font_width;
				// determine whether pixel with position (i, j) in the character box should be
				// drawed or not
				let bit = font_data & (0b1 << flag);
				if bit != 0 {
					let coord = (col_pos + i, row_pos + j,);
					put_pixel(&coord, &"#000000",)?;
				}
			}
		}

		self.col += 1;
		Ok((),)
	}
}

#[macro_export]
macro_rules! to_txt {
	(let $rslt:ident = $exp:expr) => {
		let mut ___original = $exp.clone();
		let mut ___num = [0; oso_kernel::base::text::MAX_DIGIT];
		let mut ___digits = $exp.digit_count();

		/// マイナスだった場合は`-`を先頭にくっつける
		for i in 0..___digits {
			___num[i] = ___original.shift_right() + b'0';
		}

		if $exp < 0 {
			___num[___digits] = b'-';
			___digits += 1;
		}

		let mut rslt = &mut ___num[..___digits];
		rslt.reverse();

		let $rslt = unsafe { core::str::from_utf8_unchecked(rslt,) };
	};
}

pub trait Integer:
	Add<Output = Self,>
	+ Sub<Output = Self,>
	+ Mul<Output = Self,>
	+ Div<Output = Self,>
	+ PartialOrd
	+ Ord
	+ Clone
	+ Sized
{
	fn digit_count(&self,) -> usize;
	fn nth_digit(&self, n: usize,) -> u8;
	fn shift_right(&mut self,) -> u8;
}

impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
