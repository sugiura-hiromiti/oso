use super::Context;
use super::read_le_bytes;
use crate::Rslt;
use crate::elf::Container;
use crate::elf::ElfHeader;
use crate::error::OsoLoaderError;
use alloc::format;
use alloc::vec::Vec;

pub fn gnu_hash_len(binary: &Vec<u8,>, mut offset: usize, context: &Context,) -> Rslt<usize,> {
	let buckets_count = read_le_bytes::<u32,>(&mut offset, binary,) as usize;
	let min_chain = read_le_bytes::<u32,>(&mut offset, binary,) as usize;
	let bloom_size = read_le_bytes::<u32,>(&mut offset, binary,) as usize;
	if buckets_count == 0 || min_chain == 0 || bloom_size == 0 {
		return Err(OsoLoaderError::EfiParse(format!(
			"invalid gnu hash: buckets_count={buckets_count} min_chain={min_chain} \
			 bloom_size={bloom_size}"
		),),);
	}

	// find the last bucket
	let buckets_offset =
		offset + 4 + bloom_size * if context.container == Container::Big { 8 } else { 4 };
	let mut max_chain = 0;
	for bucket in 0..buckets_count {
		let chain = read_le_bytes::<u32,>(&mut (buckets_offset + bucket * 4), binary,) as usize;
		if max_chain < chain {
			max_chain = chain;
		}
	}

	if max_chain < min_chain {
		return Ok(0,);
	}

	// find the last chain within the bucket
	let mut chain_offset = buckets_offset + buckets_count * 4 + (max_chain - min_chain) * 4;
	loop {
		let hash = read_le_bytes::<u32,>(&mut chain_offset, binary,) as usize;
		max_chain += 1;
		if hash & 1 != 0 {
			return Ok(max_chain,);
		}
	}
}

pub fn hash_len(
	binary: &Vec<u8,>,
	mut offset: usize,
	machine: u16,
	context: &Context,
) -> Rslt<usize,> {
	offset = offset.saturating_add(4,);
	let nchain = if (machine == ElfHeader::EM_FAKE_ALPHA || machine == ElfHeader::EM_S390)
		&& context.container == Container::Big
	{
		read_le_bytes::<u64,>(&mut offset, binary,) as usize
	} else {
		read_le_bytes::<u32,>(&mut offset, binary,) as usize
	};
	Ok(nchain,)
}
