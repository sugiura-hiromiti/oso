use alloc::vec::Vec;
use goblin::elf::Elf;

/// カーネルファイル展開先のメモリ領域を計算する
pub fn calc_elf_addr_range(elf: &Elf,) -> (usize, usize,) {
	let program_headers = &elf.program_headers;

	let (mut head, mut tail,) = (u64::MAX, 0,);
	for ph in program_headers {
		// プログラムがLOADセグメントの場合でない場合はスキップ
		if ph.p_type != goblin::elf::program_header::PT_LOAD {
			continue;
		}

		// セグメントの頭と尻尾を求める
		let load_seg_head = ph.p_vaddr;
		let load_seg_tail = load_seg_head + ph.p_memsz;

		head = head.min(load_seg_head,);
		tail = tail.max(load_seg_tail,);
	}

	(head as usize, tail as usize,)
}

pub fn elf_exec_size(elf: &Elf,) -> usize {
	let (head, tail,) = calc_elf_addr_range(elf,);
	tail - head
}

pub fn copy_load_segment(elf: &Elf, src: &Vec<u8,>,) {
	for ph in &elf.program_headers {
		if ph.p_type != goblin::elf::program_header::PT_LOAD {
			continue;
		}

		// `size_on_mem` maybe larger than `size` due to `.bss` section
		let size_on_mem = ph.p_memsz as usize;
		let size_on_file = ph.p_filesz as usize;
		let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, size_on_mem,) };

		// offset to head of segment
		let ofs = ph.p_offset as usize;
		// copy contents of setment describe by current program header
		dest[..size_on_file].copy_from_slice(&src[ofs..ofs + size_on_file],);
		// fill remaining bytes by 0
		dest[size_on_file..].fill(0,);
	}
}
