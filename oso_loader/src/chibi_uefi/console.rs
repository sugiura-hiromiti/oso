use crate::raw::protocol::TextOutputProtocol;

impl core::fmt::Write for TextOutputProtocol {
	fn write_str(&mut self, s: &str,) -> core::fmt::Result {
		//let a = s.bytes().chain([b'\0',].iter(),).collect();

		todo!()
	}
}
