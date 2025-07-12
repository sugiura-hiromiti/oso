use oso_error::Rslt;
use oso_error::parser::ParserError;

// ----------------- fundementals to generate parser

/// generate parser for an type
#[const_trait]
pub trait ParserGenerator<C: Context,> {
	// fn parser_lists(&self,) -> &[impl Parser<C,>];
	// fn parser<F: Fn(&Self,) -> Rslt<R, ParserError,>, R,>(&self,) -> F;
	fn parser(&self,) -> impl Parser;
}

/// parts of parser target. separated for better scalability
#[const_trait]
pub trait Context: DataState + IO {
	fn len(&self,) -> usize;
}

#[const_trait]
pub trait DataState {
	fn pos(&self,) -> usize;
}

#[const_trait]
pub trait IO {
	fn size<T,>() -> usize {
		size_of::<T,>()
	}
}

// --------------------- components for constructing parser

/// collection  of parser components.
/// each functionality itself is tiny parser
pub trait ParserComponents<C: Context,> {
	fn map<R,>(&self, context: &mut C,) -> Rslt<R,>;
}

// -------------------- output of parsergengen
#[const_trait]
pub trait Parser {}
