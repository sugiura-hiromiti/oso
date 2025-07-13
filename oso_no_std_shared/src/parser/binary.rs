//! functionality of parsing binary data

use crate::parser::generator::Context;
use crate::parser::generator::Parser;
// use crate::parser::generator::ParserGenerator;
use core::marker::PhantomData;

pub trait BinaryParser<C: Context,>: Parser<C,> {}

pub struct BinaryParserBuilder<T,> {
	__marker: PhantomData<T,>,
}

// impl<C: Context, T,> ParserGenerator<C,> for BinaryParserBuilder<T,> {
// 	fn parser(&self,) -> impl Parser<C,> {
// 		todo!()
// 	}
// }

impl<T,> BinaryParserBuilder<T,> {}
