use oso_error::Rslt;
use oso_error::parser::ParserError;

// ----------------- fundementals to generate parser

/// generate parser for an type
#[const_trait]
pub trait ParserGenerator<C: Context,> {
	// fn parser_lists(&self,) -> &[impl Parser<C,>];
	// fn parser<F: Fn(&Self,) -> Rslt<R, ParserError,>, R,>(&self,) -> F;
	fn parser<PC: Context,>(&self,) -> impl Parser<PC,>;
}

/// parts of parser target. separated for better scalability
pub trait Context {
	type Output;
	const SIZE: usize = size_of::<Self::Output,>();
	fn pos(&self,) -> usize;
	fn field_count() {}
}

// --------------------- components for constructing parser

/// collection  of parser components.
/// each functionality itself is tiny parser
pub trait ParserComponents<C: Context,> {
	fn map<R,>(&self, context: &mut C,) -> Rslt<R,>;
}

// -------------------- output of parsergengen
/// TODO: implement this trait for `Tree`
pub trait Parser<C: Context,> {
	fn parse(&self,) -> Rslt<C::Output, ParserError,>;
}
