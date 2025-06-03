use syn::Signature;

pub fn method_args(sig: &Signature,) -> impl Iterator<Item = std::boxed::Box<syn::Pat,>,> {
	sig.inputs.iter().filter_map(|a| match a {
		syn::FnArg::Receiver(_,) => None,
		syn::FnArg::Typed(pty,) => Some(pty.pat.clone(),),
	},)
}
