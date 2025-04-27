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
		let mut time_low: [u8; 4] = s.next_chunk().unwrap();
		time_low.reverse();
		let mut time_mid: [u8; 2] = s.next_chunk().unwrap();
		time_mid.reverse();
		let mut time_high_and_version: [u8; 2] = s.next_chunk().unwrap();
		time_high_and_version.reverse();

		let clock_seq_high_and_reserved = s.next().unwrap();
		let clock_seq_low = s.next().unwrap();
		let mut node: [u8; 6] = s.next_chunk().unwrap();
		node.reverse();

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

		let time_low: [u8; 4] =
			[hex[0], hex[1], hex[2], hex[3], hex[4], hex[5], hex[6], hex[7],].as_le_bytes();
		let time_mid: [u8; 2] = [hex[8], hex[9], hex[10], hex[11],].as_le_bytes();
		let time_high_and_version: [u8; 2] = [hex[12], hex[13], hex[14], hex[15],].as_le_bytes();
		let clock_seq_high_and_reserved = [hex[16], hex[17],].le_u8();
		let clock_seq_low = [hex[18], hex[19],].le_u8();
		let node: [u8; 6] = AsBytes::<12, [u8; 6],>::as_bytes(&[
			hex[20], hex[21], hex[22], hex[23], hex[24], hex[25], hex[26], hex[27], hex[28],
			hex[29], hex[30], hex[31],
		],);
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
	fn le_u64(&self,) -> u64 {
		self.le_u128() as u64
	}
	fn le_u32(&self,) -> u32 {
		self.le_u128() as u32
	}
	fn le_u16(&self,) -> u16 {
		self.le_u128() as u16
	}
	fn le_u8(&self,) -> u8 {
		self.le_u128() as u8
	}
}

pub trait BytesNotTooLong<const B: bool,> {}
impl<const BYTES: usize,> BytesNotTooLong<{ bytes_not_too_long::<BYTES,>() },> for [Hex; BYTES] {}
const fn bytes_not_too_long<const BYTES: usize,>() -> bool {
	BYTES <= 32
}

pub trait BytesIsEven<const B: bool, const N: usize,> {}
impl<const BYTES: usize,> BytesIsEven<{ bytes_is_even::<BYTES,>() }, BYTES,> for [Hex; BYTES] {}
const fn bytes_is_even<const BYTES: usize,>() -> bool {
	BYTES % 2 == 0
}

impl<const N: usize,> const BytesToInt<N,> for [Hex; N]
where [Hex; N]: BytesNotTooLong<true,>
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
	//const LIMIT: usize = BYTES / 2;
	fn as_bytes(&self,) -> Self::Output;
}

impl<const BYTES: usize,> const AsBytes<BYTES, [u8; BYTES / 2],> for [Hex; BYTES]
where [Hex; BYTES]: BytesNotTooLong<true,> + BytesIsEven<true, BYTES,>
{
	fn as_bytes(&self,) -> Self::Output {
		let mut rslt = [0; BYTES / 2];
		let mut i = 0;
		while i < BYTES / 2 {
			let left = (self[i * 2 + 1] as u8) << 4;
			let right = self[i * 2] as u8;
			rslt[i] = left + right;

			i += 1;
		}
		rslt
	}
}

impl<const BYTES: usize,> const AsBytes<BYTES, [u8; BYTES],> for [Hex; BYTES] {
	fn as_bytes(&self,) -> Self::Output {
		let mut rslt = [0; BYTES];
		let mut i = 0;
		while i < BYTES {
			rslt[i] = self[i] as u8;
			i += 1
		}
		rslt
	}
}

#[const_trait]
trait AsLeBytes<const BYTES: usize, O = Self,>:
	AsBytes<BYTES, O,> + BytesNotTooLong<true,> + BytesIsEven<true, BYTES,>
{
	fn as_le_bytes(&self,) -> Self::Output;
	fn idx(i: usize,) -> usize {
		let mut idx = BYTES / 2 - i - 1;
		if idx % 2 == 0 {
			idx += 1;
		} else {
			idx -= 1;
		}
		idx
	}
}

impl<const BYTES: usize,> const AsLeBytes<BYTES, [u8; BYTES / 2],> for [Hex; BYTES]
where [Hex; BYTES]: BytesNotTooLong<true,> + BytesIsEven<true, BYTES,>
{
	fn as_le_bytes(&self,) -> Self::Output {
		let mut le_ordered_hexes = [Hex::Zero; BYTES / 2];
		let mut i = 0;
		while i < BYTES / 2 {
			le_ordered_hexes[i] = self[Self::idx(i,)];
			i += 1;
		}
		le_ordered_hexes.as_bytes()
	}
}
