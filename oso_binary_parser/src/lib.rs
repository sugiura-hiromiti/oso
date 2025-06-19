#![no_std]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]

use oso_error::Rslt;

pub mod parser_particle;

pub trait DoParse {
	type Output;
	fn parse<C,>(&self, context: C,) -> Rslt<Self::Output,>;
}

///
pub trait ParserFeed<T,> {
}

pub struct ParseArray;

pub trait ParseParticle: SourceReader + SourceEater {
	fn as_reader(&self,) -> &impl SourceReader {
		self
	}

	fn as_runner(&self,) -> &impl SourceReader {
		self
	}
}

pub trait SourceReader: Sized {
	type Particle;
	fn skip<D,>(&self, distance: D,) -> Rslt<(),>;
	fn next(&self,) -> Rslt<Self::Particle,>;
	fn prev(&self,) -> Rslt<Self::Particle,>;
	fn skip_until<C,>(&self, condition: C,);
	fn repeat(&self, n: usize,) -> impl DoParse;
}

pub trait SourceEater: Sized {}
