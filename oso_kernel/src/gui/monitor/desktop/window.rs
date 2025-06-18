use super::Move;
use crate::base::graphic::position::Coordinal;
use crate::gui::monitor::desktop::DesktopObject;
use oso_error::Rslt;

pub trait WindowDraw {
	fn draw_window(&mut self, win: &impl Window,);
}

pub trait Window: DesktopObject + Resize {
	//fn draw_window<C: Coordinal,>(&mut self, win_buf: &WindowBuf<C,>,) -> Result<(),
	// KernelError,>;
}

pub trait Resize {
	fn narrower(&mut self, offset: usize,);
	fn wider(&mut self, offset: usize,);
	fn higher(&mut self, offset: usize,);
	fn lower(&mut self, offset: usize,);
}

pub struct WindowBuf<C: Coordinal,> {
	left_top: C,
	width:    usize,
	height:   usize,
	z:        usize,
	id:       usize,
}

impl<C: Coordinal,> Window for WindowBuf<C,> {}

impl<C: Coordinal,> Resize for WindowBuf<C,> {
	fn narrower(&mut self, offset: usize,) {
		self.width -= offset;
	}

	fn wider(&mut self, offset: usize,) {
		self.width += offset;
	}

	fn higher(&mut self, offset: usize,) {
		self.height += offset;
	}

	fn lower(&mut self, offset: usize,) {
		self.height -= offset;
	}
}

impl<C: Coordinal,> DesktopObject for WindowBuf<C,> {
	fn height(&self,) -> usize {
		self.height
	}

	fn width(&self,) -> usize {
		self.width
	}
}

impl<C: Coordinal,> Coordinal for WindowBuf<C,> {
	fn x(&self,) -> usize {
		self.left_top.x()
	}

	fn y(&self,) -> usize {
		self.left_top.y()
	}

	fn x_mut(&mut self,) -> &mut usize {
		self.left_top.x_mut()
	}

	fn y_mut(&mut self,) -> &mut usize {
		self.left_top.y_mut()
	}
}

impl<C: Coordinal,> Move for WindowBuf<C,> {
	fn move_to_x(&mut self, dest: usize,) -> Rslt<(),> {
		*self.left_top.x_mut() = dest;
		Ok((),)
	}

	fn move_to_y(&mut self, dest: usize,) -> Rslt<(),> {
		*self.left_top.y_mut() = dest;
		Ok((),)
	}

	/// # Panic
	///
	/// make sure that `self.width - offset >= 0`
	fn move_left(&mut self, offset: usize,) -> Rslt<(),> {
		*self.left_top.x_mut() -= offset;
		Ok((),)
	}

	/// # Panic
	///
	/// make sure that `self.width + offset > "max width of display"`
	fn move_right(&mut self, offset: usize,) -> Rslt<(),> {
		*self.left_top.x_mut() += offset;
		Ok((),)
	}

	/// # Panic
	///
	/// make sure that `self.height + offset > "max height of display"`
	fn move_up(&mut self, offset: usize,) -> Rslt<(),> {
		*self.left_top.y_mut() -= offset;
		Ok((),)
	}

	/// # Panic
	///
	/// make sure that `self.width + offset > 800`
	fn move_down(&mut self, offset: usize,) -> Rslt<(),> {
		*self.left_top.y_mut() += offset;
		Ok((),)
	}
}
