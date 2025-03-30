use crate::error::KernelError;

pub struct MenuBar {
	size:      usize,
	direction: Direction,
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub trait MenuBarDraw {
	fn draw_menu_bar(&mut self,) -> Result<(), KernelError,>;
}

impl MenuBarDraw for MenuBar {
	fn draw_menu_bar(&mut self,) -> Result<(), KernelError,> {
		todo!()
	}
}
