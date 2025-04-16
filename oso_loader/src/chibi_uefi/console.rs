use crate::raw::protocol::TextOutputProtocol;
use core::mem::MaybeUninit;

pub static COSOLE: MaybeUninit<TextOutputProtocol,> = MaybeUninit::uninit();

pub fn init() {}

impl core::fmt::Write for TextOutputProtocol {
	fn write_str(&mut self, s: &str,) -> core::fmt::Result {
		self.output(s,)?;
		Ok((),)
	}
}
