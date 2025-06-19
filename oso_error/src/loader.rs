#[derive(Debug,)]
pub enum EfiParseError {
	EndOfBinary {
		parser_pos: &'static str,
		stage:      EfiParseStage,
	},
	SizeOverflow {
		stage:    EfiParseStage,
		name:     u64,
		expected: u64,
		base:     u64,
		size:     u64,
	},
}

#[derive(Debug,)]
pub enum EfiParseStage {
	Header,
	ProgramHeader,
	SectionHeader,
}
