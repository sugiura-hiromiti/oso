use alloc::string::String;

use crate::raw::protocol::TextOutputProtocol;

impl core::fmt::Write for TextOutputProtocol {
	fn write_str(&mut self, s: &str,) -> core::fmt::Result {
		self.output(s,)?;
		Ok((),)
	}
}
