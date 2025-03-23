use crate::base::graphic::Coord;
use crate::gui::text::TextBuf;

const COLS: usize = 100;
const ROWS: usize = 37;

pub struct Console<const C: usize, const R: usize,> {
	text_buf: TextBuf<Coord,>,
	history:  [[u8; C]; R],
}

impl<const C: usize, const R: usize,> Console<C, R,> {
	fn new(font_width: usize, font_height: usize,) -> Self {
		let text_buf = TextBuf::new(Coord { x: 0, y: 0, }, font_width, font_height,);
		let history = [[0; C]; R];
		Self { text_buf, history, }
	}

	fn log(&mut self, s: impl AsRef<str,>,) {}
}

pub trait Print {
	fn print(&mut self, s: impl AsRef<str,>,);
	fn println(&mut self, s: impl AsRef<str,>,);
}
