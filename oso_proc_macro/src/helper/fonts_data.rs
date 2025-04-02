use colored::Colorize;
use proc_macro::Diagnostic;
use proc_macro::Level;
use syn::LitStr;

const CHARACTER_COUNT: usize = 256;

/// get font data of ascii character
pub fn fonts(specified_path: &LitStr,) -> Vec<String,> {
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
	let path = format!("{project_root}/{}", specified_path.value());
	Diagnostic::new(Level::Help, format!("path is {path}"),).emit();
	let font_data = std::fs::read_to_string(&path,).expect(&format!(
		"{}: {}\n",
		"failed to open font file".bold().red(),
		path
	),);

	let fonts_data_lines: Vec<&str,> = font_data
		.split("\n",)
		.collect::<Vec<&str,>>()
		.into_iter()
		.filter(|s| !(*s == "" || s.contains("0x",)),)
		.collect();

	let mut fonts = vec!["".to_string(); CHARACTER_COUNT];
	for idx in 0..CHARACTER_COUNT {
		fonts[idx] = fonts_data_lines[idx * 16..(idx + 1) * 16].join("",);
	}

	fonts.iter().for_each(|s| assert_eq!(s.len(), 128),);
	fonts
}

pub fn convert_bitfield(fonts: &Vec<String,>,) -> Vec<u128,> {
	let fonts: Vec<u128,> = fonts
		.into_iter()
		.map(|s| {
			let lines = s.split("\n",).collect::<Vec<&str,>>();
			let a: u128 = lines
				.into_iter()
				.enumerate()
				.map(|(i, s,)| {
					let s = s.replace(".", "0",).replace("@", "1",);
					let s: String = s.chars().rev().collect();
					let line = u128::from_str_radix(&s, 2,).unwrap();
					line << i
				},)
				.sum();
			a
		},)
		.collect();
	fonts
}
