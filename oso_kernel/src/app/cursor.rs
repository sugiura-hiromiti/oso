use crate::base::graphic::FrameBuffer;
use crate::base::graphic::color::PixelFormat;
use crate::base::graphic::position::Coordinal;
use crate::error::KernelError;
use crate::gui::monitor::desktop::DesktopObject;
use crate::gui::monitor::desktop::Move;

// TODO: modularize project structure to remove pub keyword
const MOUSE_CURSOR_WIDTH: usize = 15;
const MOUSE_CURSOR_HEIGHT: usize = 24;
//const MOUSE_CURSOR: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
pub const MOUSE_CURSOR: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
	['@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ',],
	['@', '.', '.', '.', '.', '.', '.', '@', '@', '@', '@', '@', '@', '@', '@',],
	['@', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '@', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '@', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '@', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '@', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ',],
	['@', '@', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ',],
	['@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ',],
	[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ',],
	[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', ' ', ' ', ' ',],
];

pub trait MouseCursor: DesktopObject {
	//const CURSOR: [[char; Self::WIDTH]; Self::HEIGHT];
}

pub trait MouseCursorDraw {
	fn draw_mouse_cursor(&mut self, cursor_buf: &impl MouseCursor,) -> Result<(), KernelError,>;
}

/// belong to `gui` struct
pub struct CursorBuf<C: Coordinal,> {
	pos:    C,
	width:  usize,
	height: usize,
}

impl<C: Coordinal,> CursorBuf<C,> {
	pub fn new(pos: C, width: usize, height: usize,) -> Self {
		Self { pos, width, height, }
	}
}

impl<C: Coordinal,> MouseCursor for CursorBuf<C,> {}

impl<C: Coordinal,> Coordinal for CursorBuf<C,> {
	fn x(&self,) -> usize {
		self.pos.x()
	}

	fn y(&self,) -> usize {
		self.pos.y()
	}

	fn x_mut(&mut self,) -> &mut usize {
		self.pos.x_mut()
	}

	fn y_mut(&mut self,) -> &mut usize {
		self.pos.y_mut()
	}
}

impl<C: Coordinal,> DesktopObject for CursorBuf<C,> {
	fn width(&self,) -> usize {
		self.width
	}

	fn height(&self,) -> usize {
		self.height
	}
}

impl<C: Coordinal,> Move for CursorBuf<C,> {
	fn move_up(&mut self, offset: usize,) -> Result<(), KernelError,> {
		*self.pos.y_mut() -= offset;
		Ok((),)
	}

	fn move_down(&mut self, offset: usize,) -> Result<(), KernelError,> {
		*self.pos.y_mut() += offset;
		Ok((),)
	}

	fn move_left(&mut self, offset: usize,) -> Result<(), KernelError,> {
		*self.pos.x_mut() -= offset;
		Ok((),)
	}

	fn move_right(&mut self, offset: usize,) -> Result<(), KernelError,> {
		*self.pos.x_mut() += offset;
		Ok((),)
	}

	fn move_to_x(&mut self, dest: usize,) -> Result<(), KernelError,> {
		*self.pos.x_mut() = dest;
		Ok((),)
	}

	fn move_to_y(&mut self, dest: usize,) -> Result<(), KernelError,> {
		*self.pos.y_mut() = dest;
		Ok((),)
	}
}

impl<P: PixelFormat,> MouseCursorDraw for FrameBuffer<P,> {
	fn draw_mouse_cursor(&mut self, cursor_buf: &impl MouseCursor,) -> Result<(), KernelError,> {
		let x = cursor_buf.x();
		let y = cursor_buf.y();

		for i in 0..cursor_buf.width() {
			for j in 0..cursor_buf.height() {
				// match MOUSE_CURSOR[j][i] {
				match MOUSE_CURSOR[j][i] {
					'@' => {
						self.put_pixel(&(x + i, y + j,), &"#ffffff",)?;
					},
					'.' => {
						self.put_pixel(&(x + i, y + j,), &"#000000",)?;
					},
					_ => continue,
				};
			}
		}

		Ok((),)
	}
}
