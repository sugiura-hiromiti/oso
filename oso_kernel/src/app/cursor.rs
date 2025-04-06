use crate::base::graphic::FRAME_BUFFER;
use crate::base::graphic::color::Color;
use crate::base::graphic::color::ColorRpr;
use crate::base::graphic::position::Coord;
use crate::base::graphic::position::Coordinal;
use crate::base::graphic::put_pixel;
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
	['@', '.', '.', '.', '.', '.', '.', '.', '@', '@', '@', '@', '@', '@', '@',],
	['@', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '.', '@', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '.', '@', '@', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ',],
	['@', '.', '.', '@', ' ', ' ', '@', '.', '.', '.', '@', ' ', ' ', ' ', ' ',],
	['@', '.', '@', ' ', ' ', ' ', '@', '.', '.', '.', '@', ' ', ' ', ' ', ' ',],
	['@', '@', ' ', ' ', ' ', ' ', ' ', '@', '.', '.', '.', '@', ' ', ' ', ' ',],
	['@', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '.', '.', '@', ' ', ' ', ' ',],
	[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', '@', ' ', ' ', ' ',],
	[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', ' ', ' ', ' ', ' ', ' ',],
];

pub trait MouseCursor: DesktopObject {
	//const CURSOR: [[char; Self::WIDTH]; Self::HEIGHT];
}

pub trait MouseCursorDraw {
	fn draw_mouse_cursor(&mut self,) -> Result<(), KernelError,>;
}

/// belong to `gui` struct
pub struct CursorBuf {
	pos:           Coord,
	width:         usize,
	height:        usize,
	body_color:    Color,
	outline_color: Color,
}

impl CursorBuf {
	pub fn new() -> Self {
		let mut pos = FRAME_BUFFER.right_bottom();
		*pos.x_mut() = pos.x() / 2;
		*pos.y_mut() = pos.y() / 2;
		Self {
			pos,
			width: MOUSE_CURSOR_WIDTH,
			height: MOUSE_CURSOR_HEIGHT,
			body_color: "#000000".to_color(),
			outline_color: "#ffffff".to_color(),
		}
	}
}

impl MouseCursor for CursorBuf {}
impl MouseCursorDraw for CursorBuf {
	fn draw_mouse_cursor(&mut self,) -> Result<(), KernelError,> {
		let mut coord = self.pos.clone();
		for y in 0..self.height {
			for x in 0..self.width {
				match MOUSE_CURSOR[y][x] {
					'@' => put_pixel(&coord, &self.outline_color,)?,
					'.' => put_pixel(&coord, &self.body_color,)?,
					_ => (),
				}
				*coord.x_mut() += 1;
			}
			*coord.x_mut() = self.pos.x();
			*coord.y_mut() += 1;
		}

		Ok((),)
	}
}

//
impl Coordinal for CursorBuf {
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

impl DesktopObject for CursorBuf {
	fn width(&self,) -> usize {
		self.width
	}

	fn height(&self,) -> usize {
		self.height
	}
}

impl Move for CursorBuf {
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

// impl<P: PixelFormat,> MouseCursorDraw for FrameBuffer<P,> {
// 	fn draw_mouse_cursor(&mut self, cursor_buf: &impl MouseCursor,) -> Result<(), KernelError,> {
// 		let x = cursor_buf.x();
// 		let y = cursor_buf.y();
//
// 		for i in 0..cursor_buf.width() {
// 			for j in 0..cursor_buf.height() {
// 				// match MOUSE_CURSOR[j][i] {
// 				match MOUSE_CURSOR[j][i] {
// 					'@' => {
// 						self.put_pixel(&(x + i, y + j,), &"#ffffff",)?;
// 					},
// 					'.' => {
// 						self.put_pixel(&(x + i, y + j,), &"#000000",)?;
// 					},
// 					_ => continue,
// 				};
// 			}
// 		}
//
// 		Ok((),)
// 	}
// }
