#![no_std]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]

use oso_error::Rslt;
use oso_error::parser::ParserError;

///
#[const_trait]
pub trait Parser<C: Context,> {}

// ----------------- fundementals to generate parser

/// generate parser for an type
#[const_trait]
pub trait ParserGenerator<C: Context,> {
	fn parser_lists(&self,) -> &[impl Parser<C,>];
	fn parser(&self,) -> impl Parser<C,>;
}

/// contains infos about parseing deetail
// pub trait ParserTarget<C:Context> {
// 	type Target;
// 	fn try_convert(&self, context: C,) -> Self::Target;
// }

/// parts of parser target. separated for better scalability
#[const_trait]
pub trait Context: DataState + IO {
	//IO
}

#[const_trait]
pub trait DataState {}

#[const_trait]
pub trait IO {}

// --------------------- components for constructing parser

/// collection  of parser components.
/// each functionality itself is tiny parser
pub trait ParserComponents {
	fn map(&self,);
}
