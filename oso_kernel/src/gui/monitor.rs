pub trait Monitor {
	fn add_desktop(&mut self,);
	fn count_desktop(&self,);
	fn next_desktop(&mut self,);
	fn prev_desktop(&mut self,);
	fn go_desktop(&mut self, idx: usize,);
}
