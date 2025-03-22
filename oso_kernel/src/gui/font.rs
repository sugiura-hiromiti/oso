//! font data for basic characters

use crate::base::graphic::Coordinal;
use crate::base::graphic::Draw;
use crate::base::graphic::FrameBuffer;
use oso_proc_macro::fonts_data;

// const SINONOME: &[u8; 256] = {
// 	let sinonome_font_txt = include_str!("../resource/sinonome_font.txt");
// 	let characters = &[0; 0x100];
//
// 	characters
// };

const SINONOME: &[u128; 256] = fonts_data!("resource/sinonome_font.txt");

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
}

pub trait Text {
	fn write_char<C: Coordinal,>(
		&mut self,
		char: u8,
		text_buf: &mut TextBuf<C,>,
	) -> Result<(), (),>;
	fn write_str<C: Coordinal,>(
		&mut self,
		text: &str,
		text_buf: &mut TextBuf<C,>,
	) -> Result<(), (),> {
		for c in text.as_bytes() {
			self.write_char(*c, text_buf,)?;
		}
		Ok((),)
	}
}

impl<'a,> Text for FrameBuffer<'a,> {
	/// PERF: フォントデータを画面上の描写対象のエリアに表示する際、現状では1ピクセルずつ書いている
	/// フォントデータを2次元配列として用意してフレームバッファにまとめて反映するとパフォーマンス改善できるのでは
	fn write_char<C: Coordinal,>(
		&mut self,
		char: u8,
		text_buf: &mut TextBuf<C,>,
	) -> Result<(), (),> {
		if char == b'\n' {
			text_buf.row += 1;
			text_buf.col = 0;
			return Ok((),);
		}

		let font_data = SINONOME[char as usize];
		let col_pos = text_buf.col_pixel();
		let row_pos = text_buf.row_pixel();

		for i in 0..text_buf.font_width {
			for j in 0..text_buf.font_height {
				let flag = i + j * text_buf.font_width;
				// determine whether pixel with position (i, j) in the character box should be
				// drawed or not
				let bit = font_data & (0b1 << flag);
				if bit != 0 {
					let coord = (col_pos + i, row_pos + j,);
					self.put_pixel(&coord, &"#000000",)?;
				}
			}
		}

		text_buf.col += 1;
		Ok((),)
	}
}
