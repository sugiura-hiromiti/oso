use crate::base::graphic::Coord;
use crate::base::graphic::Coordinal;
use crate::base::graphic::FrameBuffer;
use crate::gui::font::TextBuf;

const COLS: usize = 100;
const ROWS: usize = 37;

pub struct Console {
	text_buf: TextBuf<Coord,>,
	history:  [[u8; COLS]; ROWS],
}

impl Console {
	fn new(font_width: usize, font_height: usize,) -> Self {
		let text_buf = TextBuf::new(Coord { x: 0, y: 0, }, font_width, font_height,);
		let history = [[0; COLS]; ROWS];
		Self { text_buf, history, }
	}

	fn log(&mut self, s:impl AsRef<str>) {

	}
}

pub trait Print {
	fn print(&mut self, s: impl AsRef<str,>,);
	fn println(&mut self, s: impl AsRef<str,>,);
}
