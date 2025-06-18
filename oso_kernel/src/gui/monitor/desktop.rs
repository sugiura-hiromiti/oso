use window::Window;

use crate::base::graphic::position::Coordinal;
use crate::base::util::LinkedList;
use oso_error::Rslt;

pub mod window;

pub trait Move {
	fn move_up(&mut self, offset: usize,) -> Rslt<(),>;
	fn move_down(&mut self, offset: usize,) -> Rslt<(),>;
	fn move_left(&mut self, offset: usize,) -> Rslt<(),>;
	fn move_right(&mut self, offset: usize,) -> Rslt<(),>;
	fn move_to(&mut self, dest: impl Coordinal,) -> Rslt<(),> {
		self.move_to_x(dest.x(),)?;
		self.move_to_y(dest.y(),)
	}
	fn move_to_x(&mut self, dest: usize,) -> Rslt<(),>;
	fn move_to_y(&mut self, dest: usize,) -> Rslt<(),>;
}
pub trait Layer {
	fn back(&mut self,);
	fn front(&mut self,);
	fn top(&mut self,);
	fn bottom(&mut self,);
}

pub trait DesktopObject: Coordinal + Move {
	fn width(&self,) -> usize;
	fn height(&self,) -> usize;
}

pub trait DesktopDraw {
	//fn draw_mouse_cursor(&mut self, cursor_buf: &impl MouseCursor,) -> Rslt<()>;
}

pub struct DesktopBuf<'a, W: Window,> {
	id:      usize,
	windows: LinkedList<'a, W,>,
}

pub trait Desktop {
	fn add_window(&mut self,);
}

// impl<P: PixelFormat,> Desktop for FrameBuffer<'_, P,> {
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
