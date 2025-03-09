//! このモジュールは`oso_util::graphic::FrameBuffer`のビルダーを提供します

use super::FrameBuffer;

pub(super) trait DrawMarker {
	type Drawer;
}

pub(super) struct FrameBufferBuilder<F1, F2, F3, F4, F5, F6,> {
	drawer: F1,
	buf:    F2,
	width:  F3,
	height: F4,
	stride: F5,
	marker: F6,
}

impl FrameBufferBuilder<Empty, Empty, Empty, Empty, Empty, Empty,> {
	pub fn new() -> Self {
		Self {
			drawer: Default::default(),
			buf:    Default::default(),
			width:  Default::default(),
			height: Default::default(),
			stride: Default::default(),
			marker: Default::default(),
		}
	}
}

impl<'a, F1, F2, F3, F4, F5, F6,> FrameBufferBuilder<F1, F2, F3, F4, F5, F6,> {
	fn build<T,>(self,) -> FrameBuffer<'a, T,>
	where
		T: DrawMarker,
		F1: HasValue<ValueType = T::Drawer,>,
		F2: HasValue<ValueType = &'a mut [u8],>,
		F3: HasValue<ValueType = usize,>,
		F4: HasValue<ValueType = usize,>,
		F5: HasValue<ValueType = usize,>,
		F6: HasValue<ValueType = T,>,
	{
		FrameBuffer {
			drawer: self.drawer.value(),
			buf:    self.buf.value(),
			width:  self.width.value(),
			height: self.height.value(),
			stride: self.stride.value(),
		}
	}

	fn buf<'b,>(
		self,
		buf: &'b mut [u8],
	) -> FrameBufferBuilder<F1, Assigned<&'b mut [u8],>, F3, F4, F5, F6,>
	where
		F2: Assignable<&'b mut [u8],>,
	{
		FrameBufferBuilder {
			drawer: self.drawer,
			buf:    Assigned(buf,),
			width:  self.width,
			height: self.height,
			stride: self.stride,
			marker: self.marker,
		}
	}

	fn width(self, width: usize,) -> FrameBufferBuilder<F1, F2, Assigned<usize,>, F4, F5, F6,>
	where F3: Assignable<usize,> {
		FrameBufferBuilder {
			drawer: self.drawer,
			buf:    self.buf,
			width:  Assigned(width,),
			height: self.height,
			stride: self.stride,
			marker: self.marker,
		}
	}

	fn height(self, height: usize,) -> FrameBufferBuilder<F1, F2, F3, Assigned<usize,>, F5, F6,>
	where F4: Assignable<usize,> {
		FrameBufferBuilder {
			drawer: self.drawer,
			buf:    self.buf,
			width:  self.width,
			height: Assigned(height,),
			stride: self.stride,
			marker: self.marker,
		}
	}

	fn stride(self, stride: usize,) -> FrameBufferBuilder<F1, F2, F3, F4, Assigned<usize,>, F6,>
	where F5: Assignable<usize,> {
		FrameBufferBuilder {
			drawer: self.drawer,
			buf:    self.buf,
			width:  self.width,
			height: self.height,
			stride: Assigned(stride,),
			marker: self.marker,
		}
	}

	fn marker<T,>(self, marker: T,) -> FrameBufferBuilder<F1, F2, F3, F4, F5, Assigned<T,>,>
	where
		T: DrawMarker,
		F6: Assignable<T,>,
	{
		FrameBufferBuilder {
			drawer: self.drawer,
			buf:    self.buf,
			width:  self.width,
			height: self.height,
			stride: self.stride,
			marker: Assigned(marker,),
		}
	}
}

impl<F2, F3, F4, F5, T: DrawMarker,> FrameBufferBuilder<Empty, F2, F3, F4, F5, Assigned<T,>,> {
	fn drawer(
		self,
		drawer: T::Drawer,
	) -> FrameBufferBuilder<Assigned<T::Drawer,>, F2, F3, F4, F5, Assigned<T,>,> {
		FrameBufferBuilder {
			drawer: Assigned(drawer,),
			buf:    self.buf,
			width:  self.width,
			height: self.height,
			stride: self.stride,
			marker: self.marker,
		}
	}
}

#[derive(Default,)]
struct Empty;
struct Assigned<T,>(T,);

trait Assignable<T,> {}
impl<T,> Assignable<T,> for Empty {}

trait HasValue {
	type ValueType;
	fn value(self,) -> Self::ValueType;
}

impl<T,> HasValue for Assigned<T,> {
	type ValueType = T;

	fn value(self,) -> Self::ValueType {
		self.0
	}
}
