#![no_std]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_index_methods)]
#![feature(new_range_api)]
#![feature(generic_const_exprs)]

use oso_bridge::wfe;

pub mod app;
pub mod base;
pub mod driver;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo,) -> ! {
	println!("{}", info);
	wfe()
}

/// this function takes responsibility of hardware initialization, kernel setup and utility setup
/// TODO:
pub fn init() {}

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
