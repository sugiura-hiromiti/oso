#![no_std]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_index_methods)]
#![feature(new_range_api)]

use oso_bridge::wfi;

pub mod app;
pub mod base;
pub mod gui;

pub mod error {
	#[derive(Debug,)]
	pub enum KernelError {
		Graphics(GraphicError,),
	}

	#[derive(Debug,)]
	pub enum GraphicError {
		InvalidCoordinate,
	}
	impl From<KernelError,> for core::fmt::Error {
		fn from(_value: KernelError,) -> Self {
			core::fmt::Error
		}
	}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo,) -> ! {
	// println!("{}", info);
	wfi()
}

// pub mod test {
// 	use crate::print;
// 	use crate::println;
//
// 	#[cfg(test)]
// 	pub fn test_runner(tests: &[&dyn Testable],) {
// 		println!("running {} tests", tests.len());
// 		for test in tests {
// 			test.run_test()
// 		}
// 		loop {}
// 	}
//
// 	pub trait Testable {
// 		fn run_test(&self,);
// 	}
//
// 	impl<T: Fn(),> Testable for T {
// 		fn run_test(&self,) {
// 			print!("{}   ---------------\n", core::any::type_name::<T,>());
// 			self();
// 			println!("\t\t\t\t...[ok]");
// 		}
// 	}
//
// 	#[test_case]
// 	fn exmpl() {
// 		let a = 1 + 1;
// 		assert_eq!(2, a);
// 	}
// }
