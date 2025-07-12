//! # Graphics Module
//!
//! This module provides types and utilities for working with framebuffers and
//! graphics output in a bare-metal environment.
//!
//! The graphics module is designed to be used in no_std environments, typically
//! in kernel or bootloader code, to configure and interact with display hardware.
//!
//! ## Key Components
//!
//! - `PixelFormatConf`: Enum representing different pixel formats
//! - `FrameBufConf`: Structure containing framebuffer configuration parameters

/// Represents the pixel format configuration for a framebuffer.
///
/// This enum defines the different pixel formats that can be used when configuring
/// a framebuffer for graphics output.
///
/// # Variants
///
/// - `Rgb` - Red, Green, Blue color format (typically R8G8B8 or R8G8B8A8)
/// - `Bgr` - Blue, Green, Red color format (typically B8G8R8 or B8G8R8A8)
/// - `Bitmask` - Custom color format defined by bit masks
/// - `BltOnly` - Block transfer only, no direct pixel access
///
/// # Examples
///
/// ```rust
/// use oso_bridge::graphic::PixelFormatConf;
///
/// // Configure a framebuffer with RGB pixel format
/// let pixel_format = PixelFormatConf::Rgb;
/// ```
#[repr(C)]
#[derive(Debug, PartialEq, Eq,)]
pub enum PixelFormatConf {
	/// Red, Green, Blue color format
	Rgb,
	/// Blue, Green, Red color format
	Bgr,
	/// Custom color format defined by bit masks
	Bitmask,
	/// Block transfer only, no direct pixel access
	BltOnly,
}

/// NOTE: not useful until implemnt allocator
// impl<D: Draw,> PixelFormatConf {
// 	fn convert(self,) -> impl Draw {
// 		match self {
// 			PixelFormatConf::Rgb => Rgb,
// 			PixelFormatConf::Bgr => Bgr,
// 			PixelFormatConf::Bitmask => Bitmask,
// 			PixelFormatConf::BltOnly => BltOnly,
// 		}
// 	}
// }

/// Configuration structure for a framebuffer.
///
/// This structure contains all the necessary information to configure and use
/// a framebuffer for graphics output. It is designed to be passed between the
/// bootloader and kernel to provide information about the display hardware.
///
/// Since Rust doesn't have a stabilized ABI, this structure uses the `#[repr(C)]`
/// attribute to ensure a consistent memory layout when passed between components
/// compiled with different versions of Rust or different compilers.
///
/// # Fields
///
/// * `pixel_format` - The pixel format used by the framebuffer
/// * `base` - Pointer to the start of the framebuffer memory
/// * `size` - Total size of the framebuffer in bytes
/// * `width` - Width of the display in *pixels*
/// * `height` - Height of the display in pixels
/// * `stride` - Number of bytes per row (may include padding)
///
/// # Examples
///
/// ```rust,no_run
/// use oso_bridge::graphic::FrameBufConf;
/// use oso_bridge::graphic::PixelFormatConf;
///
/// // Create a new framebuffer configuration for a 1024x768 display with RGB format
/// let framebuf = FrameBufConf::new(
/// 	PixelFormatConf::Rgb,
/// 	0x1000_0000 as *mut u8, // Base address
/// 	1024 * 768 * 4,         // Size (bytes)
/// 	1024,                   // Width (pixels)
/// 	768,                    // Height (pixels)
/// 	1024 * 4,               // Stride (bytes per row)
/// );
///
/// // Access framebuffer properties
/// assert_eq!(framebuf.width, 1024);
/// assert_eq!(framebuf.height, 768);
/// ```
///
/// # Safety
///
/// This structure contains a raw pointer to the framebuffer memory.
/// The caller must ensure that this pointer is valid and points to
/// memory that can be safely written to.
#[derive(Debug,)]
#[repr(C)]
pub struct FrameBufConf {
	/// The pixel format used by the framebuffer
	pub pixel_format: PixelFormatConf,
	/// Pointer to the start of the framebuffer memory
	pub base:         *mut u8,
	/// Total size of the framebuffer in bytes
	pub size:         usize,
	/// Width of the display in pixels
	pub width:        usize,
	/// Height of the display in pixels
	pub height:       usize,
	/// Number of bytes per row (may include padding)
	pub stride:       usize,
}

impl FrameBufConf {
	/// Creates a new framebuffer configuration with the specified parameters.
	///
	/// # Parameters
	///
	/// - `pixel_format` - The pixel format used by the framebuffer
	/// - `base` - Pointer to the start of the framebuffer memory
	/// - `size` - Total size of the framebuffer in bytes
	/// - `width` - Width of the display in **pixels**
	/// - `height` - Height of the display in **pixels**
	/// - `stride` - Number of bytes per row (may include padding)
	///
	/// # Returns
	///
	/// A new `FrameBufConf` instance with the specified parameters.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use oso_bridge::graphic::FrameBufConf;
	/// use oso_bridge::graphic::PixelFormatConf;
	///
	/// // Create a new framebuffer configuration for a 1920x1080 display with RGB format
	/// let framebuf = FrameBufConf::new(
	/// 	PixelFormatConf::Rgb,
	/// 	0x1000_0000 as *mut u8, // Base address
	/// 	1920 * 1080 * 4,        // Size (bytes)
	/// 	1920,                   // Width (pixels)
	/// 	1080,                   // Height (pixels)
	/// 	1920 * 4,               // Stride (bytes per row)
	/// );
	/// ```
	pub fn new(
		pixel_format: PixelFormatConf,
		base: *mut u8,
		size: usize,
		width: usize,
		height: usize,
		stride: usize,
	) -> Self {
		Self { pixel_format, size, base, width, height, stride, }
	}
}
