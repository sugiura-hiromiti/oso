#![no_std]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]

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
