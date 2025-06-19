#![no_std]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]

use oso_error::Rslt;
use oso_error::parser::ParserError;

pub mod parser_particle;

///
pub trait ParserFeed {
	fn repeat() {}
}

// pub trait ParserParticle<T,> {
// 	type Output = T;
// 	type SkipOutput;
//
// 	fn convert<C,>(&self, context: C,) -> Self::Output;
// 	fn skip<C,>(&self, context: C,) -> Self::SkipOutput;
// }

pub trait ParseParticle<T,>: SourceReader<T,> + SourceEater<T,> {
	fn as_reader(&self,) -> &impl SourceReader<T,> {
		self
	}

	fn as_runner(&self,) -> &impl SourceReader<T,> {
		self
	}
}

pub trait SourceReader<T,>: Sized {
	fn convert<C,>(context: C,) -> Rslt<T, ParserError,>;
	fn skip<D,>(distance: D,);
	fn next();
	fn prev();
	fn skip_until();
}

pub trait SourceEater<T,>: Sized {}
