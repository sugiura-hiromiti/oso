use std::path::PathBuf;
use std::str::FromStr;

pub trait StrEnhanced: CaseConvert + StringKind {}

pub trait CaseConvert {
	type _Marker;
	fn is_camel(&self,) -> bool;
	fn is_snake(&self,) -> bool;
	fn is_screaming_snake(&self,) -> bool;
	fn is_kebab(&self,) -> bool;

	fn to_camel<S1: StringKind,>(&self,) -> S1 {
		self.case_transit(|s| format!("{}{}", s[..1].to_ascii_uppercase(), &s[1..]),)
	}

	fn to_snake<S1: StringKind,>(&self,) -> S1 {
		self.case_transit(|s| s.to_ascii_lowercase(),)
	}

	fn to_screaming_snake<S1: StringKind,>(&self,) -> S1 {
		self.case_transit(|s| s.to_ascii_uppercase(),)
	}

	fn to_kebab<S1: StringKind,>(&self,) -> S1 {
		self.case_transit(|s| s.to_ascii_lowercase(),)
	}

	fn case_transit<S: StringKind,>(&self, converter: impl FnMut(String,) -> String,) -> S {
		let converted: Vec<_,> = self.words().into_iter().map(converter,).collect();
		let converted = converted.join(&self.find_spacer().unwrap_or("".to_string(),),);
		S::from(converted,)
	}

	fn find_spacer<S: StringKind,>(&self,) -> Option<S,>;
	fn words(&self,) -> Vec<String,>;
	fn as_string_kind(&self,) -> Option<&impl StringKind,>;
}

pub trait StringKind {
	fn dump_string(&self,) -> String;
	fn from(s: impl Into<String,>,) -> Self;
	fn as_case_convert(&self,) -> Option<&impl CaseConvert,>;
}

impl StrEnhanced for String {}

impl CaseConvert for String {
	type _Marker = String;

	fn is_camel(&self,) -> bool {
		let s: String = self.clone();
		let first = s.chars().skip_while(|c| c == &'-' || c == &'_',).nth(0,);

		first.unwrap_or('a',).is_ascii_uppercase()
	}

	fn is_snake(&self,) -> bool {
		is_xxx_format_with_case(self.clone(), '_', false,)
	}

	fn is_screaming_snake(&self,) -> bool {
		is_xxx_format_with_case(self.clone(), '_', true,)
	}

	fn is_kebab(&self,) -> bool {
		is_xxx_format_with_case(self.clone(), '-', false,)
	}

	fn find_spacer<S1: StringKind,>(&self,) -> Option<S1,> {
		let s: String = self.clone();
		if s.contains("_",) {
			Some(S1::from("_".to_string(),),)
		} else if s.contains("-",) {
			Some(S1::from("-".to_string(),),)
		} else {
			None
		}
	}

	fn words(&self,) -> Vec<String,> {
		let s: String = self.clone();
		if self.is_camel() {
			let mut rslt = vec![];
			let mut idx = 0;
			while let Some(sub,) = s.get(idx..,)
				&& let Some(head,) = sub.find(|c: char| c.is_ascii_uppercase(),)
			{
				rslt.push(sub[..head].to_string(),);
				idx += head;
			}
			rslt
		} else {
			s.split(|c: char| s.find_spacer().unwrap_or(" ".to_string(),) == c.to_string(),)
				.map(|s| s.to_string(),)
				.collect()
		}
	}

	#[allow(refining_impl_trait)]
	fn as_string_kind(&self,) -> Option<&Self,> {
		Some(self,)
	}
}

impl StringKind for String {
	fn dump_string(&self,) -> String {
		self.clone()
	}

	fn from(s: impl Into<String,>,) -> Self {
		s.into()
	}

	#[allow(refining_impl_trait)]
	fn as_case_convert(&self,) -> Option<&Self,> {
		Some(self,)
	}
}

fn is_xxx_format_with_case(s: impl Into<String,> + Clone, spacer: char, with_upper: bool,) -> bool {
	let s: String = s.into();
	s.contains(|c:char| if with_upper {
	     c.is_ascii_lowercase()
	 }else {
		 c.is_ascii_uppercase()
	 } && c != spacer)
}

impl StrEnhanced for PathBuf {}

impl CaseConvert for PathBuf {
	type _Marker = PathBuf;

	fn is_camel(&self,) -> bool {
		self.dump_string().is_camel()
	}

	fn is_snake(&self,) -> bool {
		self.dump_string().is_snake()
	}

	fn is_screaming_snake(&self,) -> bool {
		self.dump_string().is_screaming_snake()
	}

	fn is_kebab(&self,) -> bool {
		self.dump_string().is_screaming_snake()
	}

	fn find_spacer<S: StringKind,>(&self,) -> Option<S,> {
		self.dump_string().find_spacer()
	}

	fn words(&self,) -> Vec<String,> {
		self.dump_string().words()
	}

	#[allow(refining_impl_trait)]
	fn as_string_kind(&self,) -> Option<&Self,> {
		Some(self,)
	}
}

impl StringKind for PathBuf {
	fn dump_string(&self,) -> String {
		self.file_name()
			.expect("failed to get file/dir name",)
			.to_str()
			.expect("failed to &str-fy file/dir name",)
			.to_string()
	}

	fn from(s: impl Into<String,>,) -> Self {
		unimplemented!("you should not use `PathBuf::from`")
		// let s: String = s.into();
		// let s = s.as_str();
		// PathBuf::from_str(s,).expect("s contains invalid character for path representation",)
	}

	#[allow(refining_impl_trait)]
	fn as_case_convert(&self,) -> Option<&Self,> {
		Some(self,)
	}
}
