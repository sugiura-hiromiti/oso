use super::raw::types::Status;

pub mod console;
pub mod memory;
pub mod protocol;

//  TODO: impl later
#[macro_export]
macro_rules! println {
	() => {};
}

impl Status {
	pub fn is_success(&self,) -> bool {
		self.clone() == Self::EFI_SUCCESS
	}
}
