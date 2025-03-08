use goblin::elf::Elf;
use goblin::elf::header::Header;

/// カーネルファイル展開先のメモリ領域を計算する
pub fn calc_kernel_address(elf: &Elf,) -> (u64, u64,) {
	let program_headers = &elf.program_headers;

	let (mut head, mut tail,) = (u64::MAX, 0,);
	for program_header in program_headers {
		// プログラムがLOADセグメントの場合でない場合はスキップ
		if program_header.p_type != goblin::elf::program_header::PT_LOAD {
			continue;
		}

		// セグメントの頭と尻尾を求める
		let load_seg_head = program_header.p_vaddr;
		let load_seg_tail = load_seg_head + program_header.p_memsz;

		head = head.min(load_seg_head,);
		tail = tail.max(load_seg_tail,);
	}

	(head, tail,)
}
