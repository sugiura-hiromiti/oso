use oso_bridge::graphic::FrameBufConf;
use oso_bridge::graphic::PixelFormatConf;

// pub struct Rgb;
// impl Draw for Rgb {
// 	fn draw_type(&self,) -> &str {
// 		"Rgb"
// 	}
//
// 	fn fill_rectangle(
// 		&mut self,
// 		left_top: &Coord,
// 		right_bottom: &Coord,
// 		color: &Color,
// 	) -> Result<(), (),> {
// 		todo!()
// 	}
// }
//
// pub struct Bgr;
// impl Draw for Bgr {
// 	fn draw_type(&self,) -> &str {
// 		"Bgr"
// 	}
//
// 	fn put_pixel(&mut self, coord: &Coord, color: &Color,) -> Result<(), (),> {
// 		todo!()
// 	}
//
// 	fn fill_rectangle(
// 		&mut self,
// 		left_top: &Coord,
// 		right_bottom: &Coord,
// 		color: &Color,
// 	) -> Result<(), (),> {
// 		todo!()
// 	}
// }
//
// pub struct Bitmask;
// impl Draw for Bitmask {
// 	fn draw_type(&self,) -> &str {
// 		"Bitmask"
// 	}
//
// 	fn put_pixel(&mut self, coord: &Coord, color: &Color,) -> Result<(), (),> {
// 		let (..,) = (coord, color,);
// 		core::todo!("put_pixel is not implemented for `{}` pixel format", self.draw_type());
// 	}
//
// 	fn fill_rectangle(
// 		&mut self,
// 		left_top: &Coord,
// 		right_bottom: &Coord,
// 		color: &Color,
// 	) -> Result<(), (),> {
// 		todo!()
// 	}
// }
//
// pub struct BltOnly;
// impl Draw for BltOnly {
// 	fn draw_type(&self,) -> &str {
// 		"BltOnly"
// 	}
//
// 	fn fill_rectangle(
// 		&mut self,
// 		left_top: &Coord,
// 		right_bottom: &Coord,
// 		color: &Color,
// 	) -> Result<(), (),> {
// 		todo!()
// 	}
// }

pub struct Coord {
	pub x: usize,
	pub y: usize,
}

impl From<(usize, usize,),> for Coord {
	fn from(value: (usize, usize,),) -> Self {
		Coord { x: value.0, y: value.1, }
	}
}

pub struct Color {
	red:   u8,
	green: u8,
	blue:  u8,
}

impl From<(u8, u8, u8,),> for Color {
	fn from(value: (u8, u8, u8,),) -> Self {
		Color { red: value.0, green: value.1, blue: value.2, }
	}
}

pub trait Draw {
	fn put_pixel(&mut self, coord: &Coord, color: &Color,) -> Result<(), (),>;

	/// # Params
	///
	/// letf_topとright_bottomの間には以下の関係が成り立っている必要があります
	/// ```no_run
	/// left_top.x < right_bottom.x && left_top.y < right_bottom.y
	/// ```
	fn fill_rectangle(
		&mut self,
		left_top: &Coord,
		right_bottom: &Coord,
		color: &Color,
	) -> Result<(), (),>;
}

pub struct FrameBuffer<'a,> {
	pub drawer: PixelFormat,
	pub buf:    &'a mut [u8],
	pub width:  usize,
	pub height: usize,
	pub stride: usize,
}

impl<'a,> FrameBuffer<'a,> {
	pub fn new(conf: FrameBufConf,) -> Self {
		let buf = unsafe { core::slice::from_raw_parts_mut(conf.base, conf.size,) };
		let pxl_fmt = conf.pixel_format.into();

		Self { drawer: pxl_fmt, buf, width: conf.width, height: conf.height, stride: conf.stride, }
	}

	// fn get_pixel(&mut self, coord: &Coord,) -> &mut [u8] {
	// 	let pos = self.pos(coord,);
	// 	&mut self.buf[pos..pos + 3]
	// }

	/// 指定された座標のポイントに該当するFramebuffer上でのindexの先頭を返します
	fn pos(&self, coord: &Coord,) -> usize {
		// 一つのピクセルの大きさが４バイトなので4をかけている
		(self.stride * coord.y + coord.x) * 4
	}
}

/// TOOD: Box<dyn Draw>を利用して条件分岐を無くす
impl<'a,> Draw for FrameBuffer<'a,> {
	fn put_pixel(&mut self, coord: &Coord, color: &Color,) -> Result<(), (),> {
		let pos = self.pos(coord,);
		let pxl = &mut self.buf[pos..pos + 3];
		let color = self.drawer.color_pixel(color,);

		self.drawer.put_color(pxl, color,);
		Ok((),)
	}

	fn fill_rectangle(
		&mut self,
		left_top: &Coord,
		right_bottom: &Coord,
		color: &Color,
	) -> Result<(), (),> {
		if left_top.x > right_bottom.x
			|| left_top.y > right_bottom.y
			|| right_bottom.x > self.width
			|| right_bottom.y > self.height
		{
			return Err((),);
		}

		let color = self.drawer.color_pixel(color,);

		for x in left_top.x..=right_bottom.x {
			for y in left_top.y..=right_bottom.y {
				let pos = self.pos(&Coord { x, y, },);
				let pxl = &mut self.buf[pos..pos + 3];
				self.drawer.put_color(pxl, color,);
			}
		}

		Ok((),)
	}
}

pub enum PixelFormat {
	Rgb,
	Bgr,
	Bitmask,
	BltOnly,
}

// macro_rules! pass_method {
// 	($pxl_fmt:ident, $method:ident, $($args:ident),*) => {
// 		match $pxl_fmt {
// 			PixelFormat::Rgb => rgb.$method($($args),*),
// 			PixelFormat::Bgr => bgr.$method($($args),*),
// 			PixelFormat::Bitmask => bm.$method($($args),*),
// 			PixelFormat::BltOnly => bo.$method($($args),*),
// 		}
// 	};
// }

// impl Draw for PixelFormat {
// 	fn put_pixel(&mut self, coord: &Coord, color: &Color,) -> Result<(), (),> {
// 		pass_method!(self, put_pixel, coord, color)
// 	}
//
// 	fn fill_rectangle(
// 		&mut self,
// 		left_top: &Coord,
// 		right_bottom: &Coord,
// 		color: &Color,
// 	) -> Result<(), (),> {
// 		pass_method!(self, fill_rectangle, left_top, right_bottom, color)
// 	}
// }

impl PixelFormat {
	///  TODO: この機能はColor構造体に移したほうがいいのでは？
	pub fn color_pixel(&self, color: &Color,) -> [u8; 3] {
		match self {
			PixelFormat::Rgb => [color.red, color.green, color.blue,],
			PixelFormat::Bgr => [color.blue, color.green, color.red,],
			PixelFormat::Bitmask => todo!(),
			PixelFormat::BltOnly => todo!(),
		}
	}

	pub fn put_color(&self, pixel: &mut [u8], color: [u8; 3],) {
		pixel[0] = color[0];
		pixel[1] = color[1];
		pixel[2] = color[2];
	}
}

impl From<PixelFormatConf,> for PixelFormat {
	fn from(value: PixelFormatConf,) -> Self {
		match value {
			PixelFormatConf::Rgb => Self::Rgb,
			PixelFormatConf::Bgr => Self::Bgr,
			PixelFormatConf::Bitmask => Self::Bitmask,
			PixelFormatConf::BltOnly => Self::BltOnly,
		}
	}
}
