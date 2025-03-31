use crate::base::util::LinkedList;
use monitor::Monitor;

pub mod console;
pub mod monitor;

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
