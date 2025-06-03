//! uefi implementation

use core::ffi::c_void;

use alloc::string::String;

use crate::Rslt;
use crate::error::OsoLoaderError;

pub mod capsule;
pub mod memory;
pub mod misc;
pub mod text;
pub mod time;

#[repr(C)]
pub struct Header {
	signature:   u64,
	revision:    u32,
	header_size: u32,
	crc32:       u32,
	reserved:    u32,
}

/// abi compatible uefi boolean
/// 0 is false,
/// others are true
#[repr(C)]
pub struct Boolean(pub u8,);

impl Boolean {
	const FALSE: Self = Self(0,);
	const TRUE: Self = Self(1,);
}

#[repr(C)]
pub struct Guid {
	time_low:                    u32,
	time_mid:                    [u8; 2],
	time_high_and_version:       [u8; 2],
	clock_seq_high_and_reserved: u8,
	clock_seq_low:               u8,
	node:                        [u8; 6],
}

#[oso_proc_macro::status_from_spec(2.11)]
#[repr(usize)]
#[derive(Eq, PartialEq, Clone,)]
pub enum Status {}
