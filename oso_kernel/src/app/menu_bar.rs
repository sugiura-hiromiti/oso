use oso_error::Rslt;

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
	fn draw_menu_bar(&mut self,) -> Rslt<(),>;
}

impl MenuBarDraw for MenuBar {
	fn draw_menu_bar(&mut self,) -> Rslt<(),> {
		todo!()
	}
}
