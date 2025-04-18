use crate::Rslt;
use crate::error::OsoLoaderError;
use crate::raw::types::Guid;
use alloc::format;

#[macro_export]
macro_rules! guid {
	($s:literal) => {{
		const GUID: $crate::raw::types::Guid = Guid::fix_by($s,);
		GUID
	}};
}

impl Guid {
	#[track_caller]
	pub fn from_str(s: impl AsRef<str,>,) -> Rslt<Self,> {
		let mut s = s.as_ref().chars().filter_map(|c| Hex::try_from(c,).ok(),).map(|h| h as u8,);
		let time_low: [u8; 4] = s.next_chunk().unwrap();
		let time_mid: [u8; 2] = s.next_chunk().unwrap();
		let time_high_and_version: [u8; 2] = s.next_chunk().unwrap();
		let clock_seq_high_and_reserved = s.next().unwrap();
		let clock_seq_low = s.next().unwrap();
		let node: [u8; 6] = s.next_chunk().unwrap();

		Ok(Self::new(
			time_low,
			time_mid,
			time_high_and_version,
			clock_seq_high_and_reserved,
			clock_seq_low,
			node,
		),)
	}

	pub const fn fix_by(s: &str,) -> Self {
		let len = s.len();
		let s_ptr = s.as_ptr();
		let mut hex = [const { Hex::Zero }; 32];

		let mut i = 0;
		let mut hex_i = 0;
		while i < len {
			let ith = unsafe { *s_ptr.add(i,) };
			if Hex::is_valid_hex(ith,) {
				hex[hex_i] = Hex::to_hex(ith,);
				hex_i += 1;
			}
			i += 1;
		}

		let time_low = [hex[0], hex[1], hex[2], hex[3],].as_bytes();
		let time_mid = [hex[4], hex[5],].as_bytes();
		let time_high_and_version = [hex[6], hex[7],].as_bytes();
		let clock_seq_high_and_reserved = hex[8] as u8;
		let clock_seq_low = hex[9] as u8;
		let node = [hex[10], hex[11], hex[12], hex[13], hex[14], hex[15],].as_bytes();
		Guid::new(
			time_low,
			time_mid,
			time_high_and_version,
			clock_seq_high_and_reserved,
			clock_seq_low,
			node,
		)
	}
}

#[repr(u8)]
#[derive(Clone, Copy,)]
enum Hex {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Eleven,
	Twelve,
	Thirteen,
	Fourteen,
	Fifteen,
}

impl Hex {
	pub const fn to_hex(byte: u8,) -> Self {
		match byte {
			b'0' => Hex::Zero,
			b'1' => Hex::One,
			b'2' => Hex::Two,
			b'3' => Hex::Three,
			b'4' => Hex::Four,
			b'5' => Hex::Five,
			b'6' => Hex::Six,
			b'7' => Hex::Seven,
			b'8' => Hex::Eight,
			b'9' => Hex::Nine,
			b'a' | b'A' => Hex::Ten,
			b'b' | b'B' => Hex::Eleven,
			b'c' | b'C' => Hex::Twelve,
			b'd' | b'D' => Hex::Thirteen,
			b'e' | b'E' => Hex::Fourteen,
			b'f' | b'F' => Hex::Fifteen,
			_ => {
				panic!("out of hex representation")
			},
		}
	}

	pub const fn is_valid_hex(byte: u8,) -> bool {
		(byte >= b'0' && byte <= b'9')
			|| (byte >= b'a' && byte <= b'f')
			|| (byte >= b'A' && byte <= b'F')
	}
}

impl TryFrom<char,> for Hex {
	type Error = OsoLoaderError;

	fn try_from(value: char,) -> Result<Self, Self::Error,> {
		let value = value as u8;
		let code = match value {
			c if Hex::is_valid_hex(c,) => Hex::to_hex(c,),
			_ => {
				return Err(OsoLoaderError::Uefi(format!("invalid hex char. 0~f are expected"),),);
			},
		};
		Ok(code,)
	}
}

#[const_trait]
pub trait BytesToInt<const N: usize,> {
	fn le_u128(&self,) -> u128;
}

pub trait BytesCondition<const B: bool,> {}
impl<const BYTES: usize,> BytesCondition<{ BYTES <= 32 },> for [Hex; BYTES] {}

impl<const N: usize,> const BytesToInt<N,> for [Hex; N]
where [Hex; N]: BytesCondition<true,>
{
	fn le_u128(&self,) -> u128 {
		let mut i = 0;
		let mut rslt = 0;
		while i < N {
			rslt += (self[i] as u128) << 4 * i;
			i += 1;
		}
		rslt
	}
}

#[const_trait]
pub trait AsBytes<const BYTES: usize, O = Self,> {
	type Output = O;
	fn as_bytes(&self,) -> Self::Output;
}

impl<const BYTES: usize,> const AsBytes<BYTES,> for [Hex; BYTES] {
	type Output = [u8; BYTES];

	fn as_bytes(&self,) -> Self::Output {
		let mut rslt = [0; BYTES];
		let mut i = 0;
		while i < BYTES {
			rslt[i] = self[i] as u8;
			i += 1;
		}
		rslt
	}
}

// fn le_hex_to_primitive<const N: usize,>(hex_array: [Hex; N],) -> u128 {
// 	todo!()
// }
