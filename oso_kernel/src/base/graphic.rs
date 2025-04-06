use crate::base::graphic::color::ColorRpr;
use crate::base::graphic::color::PixelFormat;
use crate::base::graphic::position::Coord;
use crate::base::graphic::position::Coordinal;
use crate::error::GraphicError;
use crate::error::KernelError;
#[cfg(feature = "bgr")] use color::Bgr;
#[cfg(feature = "bitmask")] use color::Bitmask;
#[cfg(feature = "bltonly")] use color::BltOnly;
#[cfg(feature = "rgb")] use color::Rgb;
use oso_bridge::graphic::FrameBufConf;
use oso_proc_macro::gen_wrapper_fn;

pub mod color;
pub mod position;

//  TODO: use `MaybeUninit`
#[cfg(feature = "rgb")]
pub static FRAME_BUFFER: FrameBuffer<Rgb,> =
	FrameBuffer { drawer: Rgb, buf: 0, size: 0, width: 0, height: 0, stride: 0, };

#[cfg(feature = "bgr")]
pub static FRAME_BUFFER: FrameBuffer<Bgr,> =
	FrameBuffer { drawer: Bgr, buf: 0, size: 0, width: 0, height: 0, stride: 0, };

#[cfg(feature = "bitmask")]
pub static FRAME_BUFFER: FrameBuffer<Bitmask,> =
	FrameBuffer { drawer: Bitmask, buf: 0, size: 0, width: 0, height: 0, stride: 0, };

#[cfg(feature = "bltonly")]
pub static FRAME_BUFFER: FrameBuffer<BltOnly,> =
	FrameBuffer { drawer: BltOnly, buf: 0, size: 0, width: 0, height: 0, stride: 0, };

/// draw to display
#[gen_wrapper_fn(FRAME_BUFFER)]
pub trait DisplayDraw {
	fn put_pixel(&self, coord: &impl Coordinal, color: &impl ColorRpr,)
	-> Result<(), KernelError,>;

	/// # Params
	///
	/// letf_topとright_bottomの間には以下の関係が成り立っている必要があります
	///
	/// ```no_run
	/// left_top.x < right_bottom.x && left_top.y < right_bottom.y
	/// ```
	///
	/// また、right_bottomは
	///
	/// ```no_run
	/// right_bottom.x <= frame_buffer.width && right_bottom.y <= frame_buffer.height
	/// ```
	///
	/// である必要があります
	fn fill_rectangle(
		&self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), KernelError,>;

	fn outline_rectangle(
		&self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), KernelError,>;
}

/// contains frame buffer itself & some helper data like display width/height, pixel format ..
pub struct FrameBuffer<P: PixelFormat,> {
	pub drawer: P,
	/// this number represents head address of frame buffer
	pub buf:    usize,
	pub size:   usize,
	pub width:  usize,
	pub height: usize,
	pub stride: usize,
}

impl<P: PixelFormat,> FrameBuffer<P,> {
	pub fn new(conf: FrameBufConf, pxl_fmt: P,) -> Self {
		let buf = conf.base as usize;
		let width = conf.width;
		let height = conf.height;
		let stride = conf.stride;
		let size = conf.size;

		Self { drawer: pxl_fmt, buf, width, height, stride, size, }
	}

	/// this method is required for inner mutability
	/// thus, unsafe
	pub unsafe fn init(
		this: *const Self,
		buf: usize,
		size: usize,
		width: usize,
		height: usize,
		stride: usize,
	) {
		unsafe {
			let this = this as *mut Self;
			(*this).buf = buf;
			(*this).size = size;
			(*this).width = width;
			(*this).height = height;
			(*this).stride = stride;
		}
	}

	/// 指定された座標のポイントに該当するFramebuffer上でのindexの先頭を返します
	fn pos(&self, coord: &impl Coordinal,) -> usize {
		// 一つのピクセルの大きさが４バイトなので4をかけている
		(self.stride * coord.y() + coord.x()) * 4
	}

	/// this function returns
	///
	/// ```no_run
	/// Coord { x: self.width - 1, y: self.height - 1, }
	/// ```
	///
	/// this is useful to get coordination of right bottom corner
	pub fn right_bottom(&self,) -> Coord {
		Coord { x: self.width - 1, y: self.height - 1, }
	}

	/// # Panic
	///
	/// if `pos` is greater than `self.size`, this function panics
	pub fn slice_mut(&self, pos: usize, len: usize,) -> &mut [u8] {
		let pos = pos * size_of::<u8,>();
		assert!(self.size - pos > 0);

		let data_at_pos = self.buf + pos;
		unsafe { core::slice::from_raw_parts_mut(data_at_pos as *mut u8, len,) }
	}
}

impl<P: PixelFormat,> DisplayDraw for FrameBuffer<P,> {
	fn put_pixel(
		&self, coord: &impl Coordinal, color: &impl ColorRpr,
	) -> Result<(), KernelError,> {
		let pos = self.pos(coord,);
		let pxl = self.slice_mut(pos, 3,);
		let color = self.drawer.color_repr(color,);
		pxl[0] = color[0];
		pxl[1] = color[1];
		pxl[2] = color[2];

		Ok((),)
	}

	fn fill_rectangle(
		&self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), KernelError,> {
		if left_top.x() > right_bottom.x()
			|| left_top.y() > right_bottom.y()
			|| right_bottom.x() > self.width
			|| right_bottom.y() > self.height
		{
			return Err(KernelError::Graphics(GraphicError::InvalidCoordinate,),);
		}

		// PERF: convert color into `[u8; 3]` before loop because this reduce determination just
		// once of which  pixel format is used
		let color = self.drawer.color_repr(color,);
		let mut coord = (left_top.x(), left_top.y(),);

		for _ in left_top.y()..=right_bottom.y() {
			for _ in left_top.x()..=right_bottom.x() {
				let pos = self.pos(&coord,);
				// let pos = self.pos(&(x, y,),);
				let pxl = self.slice_mut(pos, 3,);
				pxl[0] = color[0];
				pxl[1] = color[1];
				pxl[2] = color[2];
				coord.0 += 1;
			}
			coord.1 += 1;
			coord.0 = left_top.x();
		}

		Ok((),)
	}

	fn outline_rectangle(
		&self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), KernelError,> {
		if left_top.x() > right_bottom.x()
			|| left_top.y() > right_bottom.y()
			|| right_bottom.x() > self.width
			|| right_bottom.y() > self.height
		{
			return Err(KernelError::Graphics(GraphicError::InvalidCoordinate,),);
		}

		let width = right_bottom.x() - left_top.x() - 1;
		let height = right_bottom.y() - left_top.y() - 1;

		let color = self.drawer.color_repr(color,);
		let mut coord = (left_top.x(), left_top.y(),);

		// draw top line
		for _ in 0..width {
			let pos = self.pos(&coord,);
			let pxl = self.slice_mut(pos, 3,);
			pxl[0] = color[0];
			pxl[1] = color[1];
			pxl[2] = color[2];
			coord.0 += 1;
		}

		// draw right line
		for _ in 0..height {
			let pos = self.pos(&coord,);
			let pxl = self.slice_mut(pos, 3,);
			pxl[0] = color[0];
			pxl[1] = color[1];
			pxl[2] = color[2];
			coord.1 += 1;
		}

		// draw bottom line
		for _ in 0..width {
			let pos = self.pos(&coord,);
			let pxl = self.slice_mut(pos, 3,);
			pxl[0] = color[0];
			pxl[1] = color[1];
			pxl[2] = color[2];
			coord.0 -= 1;
		}

		// draw left line
		for _ in 0..height {
			let pos = self.pos(&coord,);
			let pxl = self.slice_mut(pos, 3,);
			pxl[0] = color[0];
			pxl[1] = color[1];
			pxl[2] = color[2];
			coord.1 -= 1;
		}

		Ok((),)
	}
}
