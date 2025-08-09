use quote::format_ident;

pub enum MacroDef {
	FnStyle {
		name:      ParamChunk<syn::Ident,>,
		item_type: ParamChunk<syn::Type,>,
	},
	Derive {
		name:      ParamChunk<syn::Ident,>,
		item_type: ParamChunk<syn::Type,>,
		attrs:     ParamChunkList<syn::Ident,>,
	},
	Attr {
		name:      ParamChunk<syn::Ident,>,
		attr_type: ParamChunk<syn::Type,>,
		item_type: ParamChunk<syn::Type,>,
	},
}

pub struct ParamChunk<T,> {
	param: T,
	#[allow(dead_code)]
	colon: syn::Token![,],
}

pub struct ParamChunkList<T,>(Vec<ParamChunk<T,>,>,);

impl<T,> ParamChunkList<T,> {
	fn unwrap(self,) -> Vec<T,> {
		self.0.into_iter().map(|pc| pc.param,).collect()
	}
}

trait IdentOrType: syn::parse::Parse {}
impl IdentOrType for syn::Ident {}
impl IdentOrType for syn::Type {}

impl<T: IdentOrType,> syn::parse::Parse for ParamChunk<T,> {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		Ok(Self { param: input.parse()?, colon: input.parse()?, },)
	}
}

impl<T: IdentOrType,> syn::parse::Parse for ParamChunkList<T,> {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		let pcl = input.step(|c| {
			let mut pcl = vec![];
			while !c.eof() {
				pcl.push(input.parse()?,);
			}
			Ok((pcl, *c,),)
		},)?;
		Ok(ParamChunkList(pcl,),)
	}
}

impl syn::parse::Parse for MacroDef {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		let macro_kind: ParamChunk<syn::Ident,> = input.parse()?;

		let macro_kind_raw = macro_kind.param.to_string();
		let macro_kind_raw = macro_kind_raw.as_str();
		let params = match macro_kind_raw {
			"fn_style" => Self::FnStyle { name: input.parse()?, item_type: input.parse()?, },
			"derive" => Self::Derive {
				name:      input.parse()?,
				item_type: input.parse()?,
				attrs:     input.parse()?,
			},
			"attr" => Self::Attr {
				name:      input.parse()?,
				attr_type: input.parse()?,
				item_type: input.parse()?,
			},
			_ => {
				return Err(input.error(format!(
					"expected one of fn_style, derive, attr. found {macro_kind_raw}"
				),),);
			},
		};

		Ok(params,)
	}
}

trait CaseTranslate {
	fn snake_case(&self,) -> Self;
}

impl CaseTranslate for syn::Ident {
	fn snake_case(&self,) -> Self {
		let mut ident_str = self.to_string();
		let mut idx = 0;
		while let Some(sub_str,) = ident_str.get(idx..,)
			&& let Some(word_head,) = sub_str.find(|c: char| c.is_ascii_uppercase(),)
		{
			let range = word_head..=word_head;
			ident_str.replace_range(
				range.clone(),
				&format!("_{}", ident_str[range].to_ascii_lowercase()),
			);
			idx = word_head + 1;
		}
		format_ident!("{ident_str}")
	}
}

pub fn def(macro_def: MacroDef,) -> proc_macro2::TokenStream {
	let rslt = match macro_def {
		MacroDef::FnStyle { name, item_type, } => {
			let name = name.param;
			let ty = item_type.param;
			quote::quote! {
				#[proc_macro]
				fn #name(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
					let item = syn::parse_macro_input!(item as #ty);
					oso_proc_macro_logic::#name::#name(item).unwrap_or_emit().into()
				}
			}
		},
		MacroDef::Derive { name, item_type, attrs, } => {
			let name = name.param;
			let fn_name = name.snake_case();
			let ty = item_type.param;
			let attrs = attrs.unwrap();
			quote::quote! {
				#[proc_macro_derive(#name #(, attributes(#attrs))*)]
				fn #fn_name(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
					let item = syn::parse_macro_input!(item as #ty);
					oso_proc_macro_logic::#fn_name::#fn_name(item).unwrap_or_emit().into()
				}
			}
		},
		MacroDef::Attr { name, attr_type, item_type, } => {
			let name = name.param;
			let attr_type = attr_type.param;
			let item_type = item_type.param;
			quote::quote! {
				#[proc_macro_attribute]
				fn #name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
					let attr = syn::parse_macro_input!(item as #attr_type);
					let item = syn::parse_macro_input!(item as #item_type);
					oso_proc_macro_logic::#name::#name(attr, item).unwrap_or_emit().into()
				}
			}
		},
	};
	rslt
}
