#![no_std]
#![feature(associated_type_defaults)]

pub mod base;
pub mod gui;

pub mod error {
	pub enum KernelError {
		Graphics,
	}
}
