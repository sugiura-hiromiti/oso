use oso_bridge::graphic::FrameBufConf;
use oso_bridge::graphic::PixelFormatConf;

// WARN: do not remove this comment outed code for future implementation
//
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

/// trait for types which can represent 2 dimentional area
/// implement this trait ensures to be able to get value of x axis & y axis
pub trait Coordinal {
	fn x(&self,) -> usize;
	fn y(&self,) -> usize;
}

pub struct Coord {
	pub x: usize,
	pub y: usize,
}

impl Coordinal for Coord {
	fn x(&self,) -> usize {
		self.x
	}

	fn y(&self,) -> usize {
		self.y
	}
}

impl Coordinal for (usize, usize,) {
	fn x(&self,) -> usize {
		self.0
	}

	fn y(&self,) -> usize {
		self.1
	}
}

impl From<(usize, usize,),> for Coord {
	fn from(value: (usize, usize,),) -> Self {
		Coord { x: value.0, y: value.1, }
	}
}

/// trait for types which can represent color format
/// implement this trait ensures to be able to get value of red, green, blue
pub trait ColorRpr {
	fn red(&self,) -> u8;
	fn green(&self,) -> u8;
	fn blue(&self,) -> u8;
}

pub struct Color {
	red:   u8,
	green: u8,
	blue:  u8,
}

impl ColorRpr for Color {
	fn red(&self,) -> u8 {
		self.red
	}

	fn green(&self,) -> u8 {
		self.green
	}

	fn blue(&self,) -> u8 {
		self.blue
	}
}

impl ColorRpr for (u8, u8, u8,) {
	fn red(&self,) -> u8 {
		self.0
	}

	fn green(&self,) -> u8 {
		self.1
	}

	fn blue(&self,) -> u8 {
		self.2
	}
}

/// this impl assumes format such as `#012345`
impl ColorRpr for &str {
	fn red(&self,) -> u8 {
		u8::from_str_radix(&self[1..3], 16,).expect("incorrect representation of color format",)
	}

	fn green(&self,) -> u8 {
		u8::from_str_radix(&self[3..5], 16,).expect("incorrect representation of color format",)
	}

	fn blue(&self,) -> u8 {
		u8::from_str_radix(&self[5..7], 16,).expect("incorrect representation of color format",)
	}
}

impl From<(u8, u8, u8,),> for Color {
	fn from(value: (u8, u8, u8,),) -> Self {
		Color { red: value.0, green: value.1, blue: value.2, }
	}
}

/// draw to display
pub trait Draw {
	fn put_pixel(&mut self, coord: &impl Coordinal, color: &impl ColorRpr,) -> Result<(), (),>;

	/// # Params
	///
	/// letf_topとright_bottomの間には以下の関係が成り立っている必要があります
	/// ```no_run
	/// left_top.x < right_bottom.x && left_top.y < right_bottom.y
	/// ```
	fn fill_rectangle(
		&mut self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), (),>;
}

/// contains frame buffer itself & some helper data like display width/height, pixel format ..
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
	fn pos(&self, coord: &impl Coordinal,) -> usize {
		// 一つのピクセルの大きさが４バイトなので4をかけている
		(self.stride * coord.y() + coord.x()) * 4
	}
}

/// TOOD: Box<dyn Draw>を利用して条件分岐を無くす
impl<'a,> Draw for FrameBuffer<'a,> {
	fn put_pixel(&mut self, coord: &impl Coordinal, color: &impl ColorRpr,) -> Result<(), (),> {
		let pos = self.pos(coord,);
		let pxl = &mut self.buf[pos..pos + 3];
		let color = self.drawer.color_pixel(color,);

		self.drawer.put_color(pxl, color,);
		Ok((),)
	}

	fn fill_rectangle(
		&mut self,
		left_top: &impl Coordinal,
		right_bottom: &impl Coordinal,
		color: &impl ColorRpr,
	) -> Result<(), (),> {
		if left_top.x() > right_bottom.x()
			|| left_top.y() > right_bottom.y()
			|| right_bottom.x() > self.width
			|| right_bottom.y() > self.height
		{
			return Err((),);
		}

		let color = self.drawer.color_pixel(color,);

		for x in left_top.x()..=right_bottom.x() {
			for y in left_top.y()..=right_bottom.y() {
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
	pub fn color_pixel(&self, color: &impl ColorRpr,) -> [u8; 3] {
		match self {
			PixelFormat::Rgb => [color.red(), color.green(), color.blue(),],
			PixelFormat::Bgr => [color.blue(), color.green(), color.red(),],
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
