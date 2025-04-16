use crate::Rslt;

use super::raw::types::Status;

pub mod console;
pub mod guid;
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

	pub fn ok_or_with<T,>(self, with: impl FnOnce() -> T,) -> Rslt<T,> {
		self.ok_or()?;
		Ok(with(),)
	}
}
