use crate::open_protocol_with;
use log::debug;
use oso_util::FrameBufConf;
use uefi::proto::console::gop::BltOp;
use uefi::proto::console::gop::BltPixel;
use uefi::proto::console::gop::BltRegion;
use uefi::proto::console::gop::GraphicsOutput;

#[derive(Clone,)]
struct Point {
	x: f32,
	y: f32,
}

impl Point {
	fn new(x: f32, y: f32,) -> Self {
		Self { x, y, }
	}
}

struct Buffer {
	width:  usize,
	height: usize,
	pixels: alloc::vec::Vec<BltPixel,>,
}

impl Buffer {
	fn new(width: usize, height: usize,) -> Self {
		Self { width, height, pixels: alloc::vec![BltPixel::new(0, 0, 0); height * width], }
	}

	/// 特定の1点を取得する
	fn pixel(&mut self, x: usize, y: usize,) -> Option<&mut BltPixel,> {
		self.pixels.get_mut(y * self.width + x,)
	}

	/// メモリをバイト毎にコピーする
	/// bltはblitとも呼ばれる
	/// この呼びはDEC PDP-10コンピュータのBLT命令に由来する
	/// (block transferの意味)
	///
	/// bufferをframebufferに移動させる
	fn blit(&self, gout: &mut GraphicsOutput,) -> uefi::Result {
		gout.blt(BltOp::BufferToVideo {
			buffer: &self.pixels,
			src:    BltRegion::Full,
			dest:   (0, 0,),
			dims:   (self.width, self.height,),
		},)
	}

	/// 特定の1点のみをframebufferに転送する
	fn blit_pixel(&self, gout: &mut GraphicsOutput, coord: (usize, usize,),) -> uefi::Result {
		gout.blt(BltOp::BufferToVideo {
			buffer: &self.pixels,
			src:    BltRegion::SubRectangle { coords: coord, px_stride: self.width, },
			dest:   coord,
			dims:   (1, 1,),
		},)
	}
}

pub fn fill_with(red: u8, green: u8, blue: u8,) -> uefi::Result {
	// graphics output protocolを開く
	let mut gout = open_protocol_with::<GraphicsOutput,>()?;
	debug!("opened graphics output protocol");

	let mode_info = gout.current_mode_info();
	debug!("resolution: {:?}", mode_info.resolution());
	debug!("pixel_format: {:?}", mode_info.pixel_format());
	debug!("pixel_bitmask: {:?}", mode_info.pixel_bitmask());
	debug!("stride: {:?}", mode_info.stride());

	let (width, height,) = mode_info.resolution();

	gout.blt(BltOp::VideoFill {
		color: BltPixel::new(red, green, blue,),
		dest:  (0, 0,),
		dims:  (width, height,),
	},)?;

	let mut buf = Buffer::new(width, height,);
	for y in 0..height {
		let r = ((y as f32) / ((height - 1) as f32)) * 255.0;
		for x in 0..width {
			let g = ((x as f32) / ((width - 1) as f32)) * 255.0;
			let pixel = buf.pixel(x, y,).unwrap();
			pixel.red = r as u8;
			pixel.green = g as u8;
			pixel.blue = 255;
		}
	}

	// 背景をセット
	buf.blit(&mut gout,)?;

	debug!("filled");

	Ok((),)
}

/// sierpinski図形を描く
pub fn draw_sierpinski() -> uefi::Result {
	debug!("entered draw_sierpinski");

	// graphics output protocolを開く
	let mut gout = open_protocol_with::<GraphicsOutput,>()?;
	debug!("opened graphics output protocol");

	// 描画のためのバッファを作成する
	let (width, height,) = gout.current_mode_info().resolution();
	debug!("resolution: width {width}, height {height}");
	let mut buf = Buffer::new(width, height,);
	debug!("cretae buffor for render");

	// シンプルなグラデーション背景で初期化
	for y in 0..height {
		let r = ((y as f32) / ((height - 1) as f32)) * 255.0;
		for x in 0..width {
			let g = ((x as f32) / ((width - 1) as f32)) * 255.0;
			let pixel = buf.pixel(x, y,).unwrap();
			pixel.red = r as u8;
			pixel.green = g as u8;
			pixel.blue = 255;
		}
	}

	// 背景をセット
	buf.blit(&mut gout,)?;

	let size = Point::new(width as f32, height as f32,);

	let border = 20.0;
	let triangle = [
		Point::new(size.x / 2.0, border,),
		Point::new(border, size.y - border,),
		Point::new(size.x - border, size.y - border,),
	];

	let mut p = Point::new(size.x / 2.0, size.y / 2.0,);

	let mut i = 0;
	loop {
		i += 2;
		// 三角形の頂点をランダムに選ぶ
		let v = triangle[i % 3].clone();
		p.x = (p.x + v.x) * 0.5;
		p.y = (p.y + v.y) * 0.5;

		let pixel = buf.pixel(p.x as usize, p.y as usize,).unwrap();
		pixel.red = 0;
		pixel.green = 0;
		pixel.blue = 0;

		buf.blit_pixel(&mut gout, (p.x as usize, p.y as usize,),)?;
	}
}
