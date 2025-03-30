use crate::base::util::LinkedList;
use crate::error::KernelError;
use crate::gui::desktop::Desktop;
use monitor::Monitor;

pub mod console;
pub mod desktop;
pub mod monitor;
pub mod text;

pub trait Gui {
	fn add_monitor(&mut self, m: impl Monitor,);
	fn remove_monitor(&mut self, idx: usize,);
	fn count_monitor(&self,);
	fn next_monitor(&mut self,);
	fn prev_monitor(&mut self,);
	fn go_monitor(&mut self, idx: usize,);
}

pub struct GuiBuf<'a, M: Monitor,> {
	monitors:    LinkedList<'a, M,>,
	cur_monitor: usize,
}

impl<'a, M: Monitor,> GuiBuf<'a, M,> {
	fn new() {}
}
