#![no_std]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_index_methods)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

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
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()],) {}
