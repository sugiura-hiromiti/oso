//! # Font Data Processing Module
//!
//! This module provides functionality for loading and processing ASCII font data
//! for use in the OSO operating system. It handles bitmap font conversion from
//! text-based representations to binary formats suitable for rendering.

use colored::Colorize;
use proc_macro::Diagnostic;
use proc_macro::Level;
use syn::LitStr;

/// Number of ASCII characters supported (0-255)
const CHARACTER_COUNT: usize = 256;

pub fn fonts_data_body(path: &syn::LitStr,) -> proc_macro2::TokenStream {
	let fonts = convert_bitfield(&fonts(path,),);
	quote::quote! {
		&[#(#fonts),*]
	}
}

/// Loads and processes ASCII font data from a specified file path
///
/// This function reads a font data file containing ASCII character bitmaps
/// represented as text patterns using '.' for empty pixels and '@' for filled pixels.
/// Each character is expected to be 16 lines tall with 8 pixels per line.
///
/// # Arguments
///
/// * `specified_path` - A string literal containing the relative path to the font file from the
///   project root directory
///
/// # Returns
///
/// A vector of 256 strings, where each string represents the bitmap data for one
/// ASCII character. Each string contains 128 characters (16 lines × 8 characters per line).
///
/// # Panics
///
/// This function will panic if:
/// - The font file cannot be read
/// - Any character bitmap doesn't have exactly 128 characters
///
/// # Examples
///
/// ```ignore
/// use syn::LitStr;
/// let path = LitStr::new("assets/font.txt", proc_macro2::Span::call_site());
/// let font_data = fonts(&path);
/// assert_eq!(font_data.len(), 256);
/// ```
fn fonts(specified_path: &LitStr,) -> Vec<String,> {
	// Get the project root directory, falling back to compile-time directory if needed
	#[cfg(not(test))]
	let project_root = std::env::var("CARGO_MANIFEST_DIR",).unwrap_or_else(|e| {
		Diagnostic::new(
			Level::Warning,
			format!(
				"failed to get `CARGO_MANIFEST_DIR`:\n{e}\nenvironment variable root dir of \
				 oso_proc_macro is used instead"
			),
		)
		.emit();
		env!("CARGO_MANIFEST_DIR").to_string()
	},);
	#[cfg(test)]
	let project_root = "".to_string();

	// Construct the full path to the font file
	let path = format!("{project_root}/{}", specified_path.value());
	#[cfg(not(test))]
	Diagnostic::new(Level::Help, format!("path is {path}"),).emit();

	// Read the font data file
	let font_data = std::fs::read_to_string(&path,).expect(&format!(
		"{}: {}\n",
		"failed to open font file".bold().red(),
		path
	),);

	// Split the file into lines and filter out empty lines and hex values
	let fonts_data_lines: Vec<&str,> = font_data
		.split("\n",)
		.collect::<Vec<&str,>>()
		.into_iter()
		.filter(|s| !(*s == "" || s.contains("0x",)),) // Remove empty lines and hex values
		.collect();

	// Process each character (16 lines per character)
	let mut fonts = vec!["".to_string(); CHARACTER_COUNT];
	for idx in 0..CHARACTER_COUNT {
		// Each character consists of 16 consecutive lines
		fonts[idx] = fonts_data_lines[idx * 16..(idx + 1) * 16].join("",);
	}

	// Verify that each character has exactly 128 characters (16 lines × 8 chars)
	fonts.iter().for_each(|s| assert_eq!(s.len(), 128),);
	fonts
}

/// Converts text-based font bitmaps to binary bitfield representation
///
/// This function takes the string-based font data (with '.' and '@' characters)
/// and converts it to a more compact binary representation using u128 integers.
/// Each character's bitmap is encoded as a single u128 value where each bit
/// represents a pixel.
///
/// # Arguments
///
/// * `fonts` - A vector of strings containing the text-based bitmap data, where '.' represents an
///   empty pixel and '@' represents a filled pixel
///
/// # Returns
///
/// A vector of u128 values, where each value represents the bitmap for one character.
/// The bits are arranged with the first line at the least significant bits.
///
/// # Bitmap Encoding
///
/// - '.' characters are converted to '0' bits (empty pixels)
/// - '@' characters are converted to '1' bits (filled pixels)
/// - Each line is bit-reversed before encoding
/// - Lines are stacked with line 0 at the LSB and line 15 at the MSB
///
/// # Examples
///
/// ```ignore
/// let fonts = vec!["........@@......".to_string(); 256];
/// let bitfields = convert_bitfield(&fonts);
/// assert_eq!(bitfields.len(), 256);
/// ```
fn convert_bitfield(fonts: &Vec<String,>,) -> Vec<u128,> {
	let fonts: Vec<u128,> = fonts
		.into_iter()
		.map(|s| {
			// Split each character's bitmap into 16 lines
			let lines = s.split("\n",).collect::<Vec<&str,>>();

			// Process each line and combine into a single u128
			let a: u128 = lines
				.into_iter()
				.enumerate()
				.map(|(i, s,)| {
					// Convert '.' to '0' and '@' to '1'
					let s = s.replace(".", "0",).replace("@", "1",);

					// Reverse the bit order for proper display orientation
					let s: String = s.chars().rev().collect();

					// Parse the binary string to get the line value
					let line = u128::from_str_radix(&s, 2,).unwrap();

					// Shift the line to its proper position (line i goes to bit position i*8)
					line << i
				},)
				.sum(); // Combine all lines using bitwise OR (via sum)
			a
		},)
		.collect();
	fonts
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::path::Path;
	use tempfile::NamedTempFile;

	/// Creates a temporary font file for testing
	fn create_test_font_file() -> NamedTempFile {
		let mut temp_file = NamedTempFile::new().expect("Failed to create temp file",);

		// Create sample font data for character 'A' (ASCII 65)
		// 16 lines of 8 characters each, representing a simple 'A' pattern
		let font_data = r#"
........
...@@...
..@..@..
..@..@..
..@..@..
..@@@@..
..@..@..
..@..@..
..@..@..
..@..@..
........
........
........
........
........
........"#;

		// Repeat this pattern 256 times for all ASCII characters
		let mut full_font_data = String::new();
		for _ in 0..256 {
			full_font_data.push_str(font_data,);
			full_font_data.push('\n',);
		}

		fs::write(temp_file.path(), full_font_data,).expect("Failed to write test font data",);
		temp_file
	}

	#[test]
	fn test_fonts_loads_correct_number_of_characters() {
		let temp_file = create_test_font_file();
		let path_str = temp_file.path().to_str().unwrap();
		let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site(),);

		let fonts = fonts(&lit_str,);

		// Should load exactly 256 characters
		assert_eq!(fonts.len(), 256);
	}

	#[test]
	fn test_fonts_each_character_has_correct_length() {
		let temp_file = create_test_font_file();
		let path_str = temp_file.path().to_str().unwrap();
		let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site(),);

		let fonts = fonts(&lit_str,);

		// Each character should have exactly 128 characters (16 lines × 8 chars)
		for (i, font_char,) in fonts.iter().enumerate() {
			assert_eq!(
				font_char.len(),
				128,
				"Character {} has incorrect length: {}",
				i,
				font_char.len()
			);
		}
	}

	#[test]
	fn test_convert_bitfield_returns_correct_count() {
		let test_fonts = vec!["........".repeat(16); 256];
		let bitfields = convert_bitfield(&test_fonts,);

		assert_eq!(bitfields.len(), 256);
	}

	#[test]
	fn test_convert_bitfield_empty_pattern() {
		// Test with all empty pixels (all dots)
		let empty_pattern = "........".repeat(16,);
		let test_fonts = vec![empty_pattern; 1];
		let bitfields = convert_bitfield(&test_fonts,);

		// All dots should result in 0
		assert_eq!(bitfields[0], 0);
	}

	#[test]
	fn test_convert_bitfield_full_pattern() {
		// Test with all filled pixels (all @)
		let full_pattern = "@@@@@@@@".repeat(16,);
		let test_fonts = vec![full_pattern; 1];
		let bitfields = convert_bitfield(&test_fonts,);

		// All @ should result in a non-zero value
		assert_ne!(bitfields[0], 0);
	}

	#[test]
	fn test_convert_bitfield_specific_pattern() {
		// Test a specific pattern: single pixel in top-left corner
		let mut pattern = String::new();
		pattern.push_str("@.......",); // First line with one pixel
		for _ in 1..16 {
			pattern.push_str("........",); // Remaining 15 lines empty
		}

		let test_fonts = vec![pattern; 1];
		let bitfields = convert_bitfield(&test_fonts,);

		// Should have the rightmost bit set (due to bit reversal)
		assert_eq!(bitfields[0] & 1, 1);
	}

	#[test]
	fn test_convert_bitfield_line_positioning() {
		// Test that different lines result in different bit positions
		let mut patterns = Vec::new();

		// Create patterns with a single pixel on different lines
		for line in 0..16 {
			let mut pattern = String::new();
			for i in 0..16 {
				if i == line {
					pattern.push_str("@.......",); // Pixel on this line
				} else {
					pattern.push_str("........",); // Empty line
				}
			}
			patterns.push(pattern,);
		}

		let bitfields = convert_bitfield(&patterns,);

		// Each pattern should produce a different value
		for i in 0..15 {
			for j in (i + 1)..16 {
				assert_ne!(
					bitfields[i], bitfields[j],
					"Patterns {} and {} produced the same bitfield",
					i, j
				);
			}
		}
	}

	#[test]
	#[should_panic(expected = "failed to open font file")]
	fn test_fonts_nonexistent_file() {
		let lit_str =
			syn::LitStr::new("/nonexistent/path/font.txt", proc_macro2::Span::call_site(),);
		fonts(&lit_str,);
	}

	#[test]
	fn test_fonts_with_hex_values_filtered() {
		let mut temp_file = NamedTempFile::new().expect("Failed to create temp file",);

		// Create font data with hex values that should be filtered out
		let font_data_with_hex = r#"
0x41
........
...@@...
..@..@..
..@..@..
..@..@..
..@@@@..
..@..@..
..@..@..
..@..@..
..@..@..
........
........
........
........
........
........"#;

		// Repeat for all 256 characters
		let mut full_font_data = String::new();
		for _ in 0..256 {
			full_font_data.push_str(font_data_with_hex,);
			full_font_data.push('\n',);
		}

		fs::write(temp_file.path(), full_font_data,).expect("Failed to write test font data",);

		let path_str = temp_file.path().to_str().unwrap();
		let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site(),);

		let fonts = fonts(&lit_str,);

		// Should still load 256 characters, with hex lines filtered out
		assert_eq!(fonts.len(), 256);

		// Each character should still have 128 characters (hex lines filtered)
		for font_char in &fonts {
			assert_eq!(font_char.len(), 128);
		}
	}
}
