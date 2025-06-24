use anyhow::Result as Rslt;
use anyhow::anyhow;
use html5ever::local_name;
use html5ever::tendril::TendrilSink;
use markup5ever::LocalNameStaticSet;
use markup5ever_rcdom::Node;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use proc_macro::Diagnostic;
use proc_macro::Level;
use std::rc::Rc;

/// element id of main section of status code page in uefi specification
const MAIN_SECTION_ID: &str = "status-codes";
/// element id of table of success codes of status code
const SUCCESS_CODE_TABLE_ID: &str = "efi-status-success-codes-high-bit-clear-apx-d-status-codes";
/// element id of table of error codes of status code
const ERROR_CODE_TABLE_ID: &str = "efi-status-error-codes-high-bit-set-apx-d-status-codes";
/// element id of table of warning codes of status code
const WARN_CODE_TABLE_ID: &str = "efi-status-warning-codes-high-bit-clear-apx-d-status-codes";

#[derive(Debug,)]
pub struct StatusCode {
	pub success: Vec<StatusCodeInfo,>,
	pub error:   Vec<StatusCodeInfo,>,
	pub warn:    Vec<StatusCodeInfo,>,
}

#[derive(Debug,)]
pub struct StatusCodeInfo {
	pub mnemonic: String,
	pub value:    usize,
	pub desc:     String,
}

impl StatusCodeInfo {
	pub const ERROR_BIT: usize = 1 << (usize::BITS - 1);
}

pub fn status_spec_page(status_spec_url: impl Into<String,>,) -> Rslt<StatusCode,> {
	let mut rsp = ureq::get(status_spec_url.into(),).call()?;
	let rsp_body = rsp.body_mut().read_to_string()?;
	let dom =
		html5ever::parse_document(RcDom::default(), Default::default(),).one(rsp_body.as_str(),);

	let node = dom.document;
	let main_section = get_element_by_id(node.clone(), MAIN_SECTION_ID,)
		.expect("failed to get main section node",);
	let success_code_table = get_element_by_id(main_section.clone(), SUCCESS_CODE_TABLE_ID,)
		.ok_or(anyhow!("ELEMENT WITH ID NOT FOUND: {SUCCESS_CODE_TABLE_ID}"),)?;
	let error_code_table = get_element_by_id(main_section.clone(), ERROR_CODE_TABLE_ID,)
		.ok_or(anyhow!("ELEMENT WITH ID NOT FOUND: {ERROR_CODE_TABLE_ID}"),)?;
	let warn_code_table = get_element_by_id(main_section.clone(), WARN_CODE_TABLE_ID,)
		.ok_or(anyhow!("ELEMENT WITH ID NOT FOUND: {WARN_CODE_TABLE_ID}"),)?;

	let success_code_table_rows = table_rows(success_code_table.clone(),);
	let error_code_table_rows = table_rows(error_code_table.clone(),);
	let warn_code_table_rows = table_rows(warn_code_table.clone(),);

	let success_codes_info: Vec<Vec<String,>,> =
		success_code_table_rows.iter().map(|n| table_data(n.clone(),),).collect();
	let error_codes_info: Vec<Vec<String,>,> =
		error_code_table_rows.iter().map(|n| table_data(n.clone(),),).collect();
	let warn_codes_info: Vec<Vec<String,>,> =
		warn_code_table_rows.iter().map(|n| table_data(n.clone(),),).collect();

	let success_codes = status_codes_info(success_codes_info,);
	let mut error_codes = status_codes_info(error_codes_info,);
	let warn_codes = status_codes_info(warn_codes_info,);

	error_codes.iter_mut().for_each(|sci| {
		sci.value = sci.value | StatusCodeInfo::ERROR_BIT;
	},);

	Ok(StatusCode { success: success_codes, error: error_codes, warn: warn_codes, },)
}

fn get_element_by_id(node: Rc<Node,>, id: &str,) -> Option<Rc<Node,>,> {
	let found = if let NodeData::Element { attrs, .. } = &node.data {
		let attrs_borrow = attrs.borrow();
		attrs_borrow
			.iter()
			.find(|a| {
				let value = unsafe {
					tendril::StrTendril::from_byte_slice_without_validating(id.as_bytes(),)
				};
				let local_name = local_name!("id");

				*a.name.local == *local_name && a.value == value
			},)
			.is_some()
	} else {
		false
	};

	if found {
		Some(node,)
	} else {
		node.children.borrow().iter().find_map(|n| get_element_by_id(n.clone(), id,),)
	}
}

/// if there is not elements match, return None
/// if this function returns Some but vector is len 0, something went wrong
#[allow(dead_code)]
fn get_elements_by_attribute(node: Rc<Node,>, attr: &str, value: &str,) -> Vec<Rc<Node,>,> {
	let mut rslt = vec![];
	let matches = match &node.data {
		NodeData::Element { attrs, .. } => attrs
			.borrow()
			.iter()
			.find(|a| {
				let local_name = string_cache::Atom::<LocalNameStaticSet,>::from(attr,);
				*a.name.local == *local_name && a.value.contains(value,)
			},)
			.is_some(),
		_ => false,
	};
	if matches {
		rslt.push(node.clone(),);
	}

	node.children.borrow().iter().for_each(|n| {
		let mut child_matches = get_elements_by_attribute(n.clone(), attr, value,);
		rslt.append(&mut child_matches,);
	},);

	rslt
}

fn get_elements_by_name(node: Rc<Node,>, tag_name: &str,) -> Vec<Rc<Node,>,> {
	let mut rslt = vec![];
	let matches = match &node.data {
		NodeData::Element { name, .. } => {
			let element_name = string_cache::Atom::<LocalNameStaticSet,>::from(tag_name,);
			*name.local == *element_name
		},
		_ => false,
	};

	if matches {
		rslt.push(node.clone(),);
	}

	node.children.borrow().iter().for_each(|n| {
		let mut child_matches = get_elements_by_name(n.clone(), tag_name,);
		rslt.append(&mut child_matches,);
	},);

	rslt
}

fn table_rows(node: Rc<Node,>,) -> Vec<Rc<Node,>,> {
	get_elements_by_name(node.clone(), "tr",)[1..].to_vec()
}

fn table_data(node: Rc<Node,>,) -> Vec<String,> {
	let mut rslt = vec![];
	let row = get_elements_by_name(node.clone(), "p",);

	let NodeData::Text { ref contents, } = row[0].clone().children.borrow()[0].clone().data else {
		panic!("text node expected: {:#?}", row[0].clone())
	};
	rslt.push(contents.borrow().as_str().to_string(),);

	let NodeData::Text { ref contents, } = row[1].clone().children.borrow()[0].clone().data else {
		panic!("text node expected: {:#?}", row[1].clone())
	};
	rslt.push(contents.borrow().as_str().to_string(),);

	let NodeData::Text { ref contents, } = row[2].clone().children.borrow()[0].clone().data else {
		panic!("text node expected: {:#?}", row[2].clone())
	};
	rslt.push(contents.borrow().as_str().to_string(),);

	rslt
}

fn status_codes_info(rows: Vec<Vec<String,>,>,) -> Vec<StatusCodeInfo,> {
	rows.into_iter()
		.map(|row| StatusCodeInfo {
			mnemonic: row[0].clone(),
			value:    row[1].parse().expect("value expected being parsable to integer",),
			desc:     row[2].clone(),
		},)
		.collect()
}

#[allow(dead_code)]
fn inspect_children(node: Rc<Node,>,) {
	Diagnostic::new(Level::Help, "start inspect_children --------------------------------",).emit();
	node.children.borrow().iter().enumerate().for_each(|(i, n,)| {
		let name = match &n.data {
			markup5ever_rcdom::NodeData::Document => todo!("inspect_children/Document"),
			markup5ever_rcdom::NodeData::Doctype { .. } => todo!("inspect_children/Doctype"),
			markup5ever_rcdom::NodeData::Text { contents, } => format!("text: {contents:?}"),
			markup5ever_rcdom::NodeData::Comment { .. } => todo!("inspect_children/Comment"),
			markup5ever_rcdom::NodeData::Element { name, .. } => format!("element: {name:?}"),
			markup5ever_rcdom::NodeData::ProcessingInstruction { .. } => {
				todo!("inspect_children/ProcessingInstruction")
			},
		};
		Diagnostic::new(Level::Note, format!("{i}, {name}"),).emit();
	},);
	Diagnostic::new(Level::Help, "start inspect_children --------------------------------",).emit();
}

#[allow(dead_code)]
fn inspect_node(node: Rc<Node,>,) {
	Diagnostic::new(Level::Note, format!("{node:#?}"),).emit();
}

#[cfg(test)]
mod tests {
	use super::*;
	use html5ever::QualName;
	use html5ever::ns;
	use markup5ever::namespace_url;

	const BASIC_HTML: &str = r#"
<div class="wow" id="identical">
	<p style="color: blue">text</p>
</div>
<section>hohoho</section>
<section class="main_sec wow">
	<h1 id="first_header">welcome</h1>
	<p class="wow">0w0</p>
</section>"#;

	/// this fn converts text input into node representation
	fn parse_text(txt: impl Into<String,>,) -> Rc<Node,> {
		let dom = html5ever::parse_fragment(
			RcDom::default(),
			Default::default(),
			QualName::new(None, ns!(), local_name!(""),),
			vec![],
		)
		.one(txt.into(),);
		dom.document
	}

	#[test]
	fn test_parse_text() {
		let node = parse_text(BASIC_HTML,);
		eprintln!("{node:#?}")
	}

	#[test]
	fn test_get_element_by_id() -> Rslt<(),> {
		let node = parse_text(BASIC_HTML,);
		get_element_by_id(node.clone(), "identical",)
			.ok_or(anyhow!("failed to get element with id identical"),)?;
		get_element_by_id(node.clone(), "first_header",)
			.ok_or(anyhow!("failed to get element with id first_header"),)?;
		get_element_by_id(node.clone(), "non_exist_id",).ok_or(anyhow!("success"),).unwrap_err();
		Ok((),)
	}

	#[test]
	fn test_get_elements_by_attribute() {
		let node = parse_text(BASIC_HTML,);
		let class_wow = get_elements_by_attribute(node.clone(), "class", "wow",);
		assert_eq!(class_wow.len(), 3);

		let class_main_sec = get_elements_by_attribute(node.clone(), "class", "main_sec",);
		assert_eq!(class_main_sec.len(), 1);

		let style_color_bule = get_elements_by_attribute(node.clone(), "style", "color: blue",);
		assert_eq!(style_color_bule.len(), 1);
	}

	#[test]
	fn test_get_elements_by_name() {
		let node = parse_text(BASIC_HTML,);
		let div = get_elements_by_name(node.clone(), "div",);
		assert_eq!(div.len(), 1);

		let p = get_elements_by_name(node.clone(), "p",);
		assert_eq!(p.len(), 2);

		let section = get_elements_by_name(node.clone(), "section",);
		assert_eq!(section.len(), 2);

		let h1 = get_elements_by_name(node.clone(), "h1",);
		assert_eq!(h1.len(), 1);
	}
}
