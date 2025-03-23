//! font data for basic characters

use oso_proc_macro::fonts_data;

// const SINONOME: &[u8; 256] = {
// 	let sinonome_font_txt = include_str!("../resource/sinonome_font.txt");
// 	let characters = &[0; 0x100];
//
// 	characters
// };

pub const SINONOME: &[u128; 256] = fonts_data!("resource/sinonome_font.txt");
