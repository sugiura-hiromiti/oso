use anyhow::Result as Rslt;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Node;
use markup5ever_rcdom::RcDom;
use proc_macro::Diagnostic;
use proc_macro::Level;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// element id of main section of status code page in uefi specification
const MAIN_SECTION_ID: &str = "status-codes";
/// element id of table of success codes of status code
const SUCCESS_CODE_TABLE_ID: &str = "efi-status-success-codes-high-bit-clear-apx-d-status-codes";
/// element id of table of error codes of status code
const ERROR_CODE_TABLE_ID: &str = "efi-status-error-codes-high-bit-set-apx-d-status-codes";
/// element id of table of warning codes of status code
const WARN_CODE_TABLE_ID: &str = "efi-status-warning-codes-high-bit-clear-apx-d-status-codes";

pub fn status_spec_page(status_spec_url: impl Into<String,>,) -> Rslt<(),> {
	let mut rsp = ureq::get(status_spec_url.into(),).call()?;
	let rsp_body = rsp.body_mut().read_to_string()?;
	// Diagnostic::new(Level::Note, &rsp_body,).emit();
	let dom =
		html5ever::parse_document(RcDom::default(), Default::default(),).one(rsp_body.as_str(),);
	let node = dom.document;
	let main_section = query_by_id(node.clone(), MAIN_SECTION_ID,);
	let success_code_table = query_by_id(node, SUCCESS_CODE_TABLE_ID,);

	Diagnostic::new(Level::Note, format!("{},{:#?}", stringify!(main_section), main_section),)
		.emit();
	Diagnostic::new(
		Level::Note,
		format!(
			"length of children in main section is: {}",
			main_section.as_ref().unwrap().children.borrow().len()
		),
	)
	.emit();

	Diagnostic::new(
		Level::Note,
		format!(
			"{},{:#?}",
			stringify!(success_code_table),
			success_code_table.as_ref().unwrap().parent.take().is_some()
		),
	)
	.emit();
	todo!()
}

fn query_by_id(node: Rc<Node,>, id: &str,) -> Option<Rc<Node,>,> {
	use html5ever::local_name;
	use markup5ever::interface::QualName;
	use markup5ever_rcdom::NodeData;

	let found = if let NodeData::Element { attrs, .. } = &node.data {
		let attrs_borrow = attrs.borrow();
		let a = attrs_borrow.iter().find(|a| {
			let value =
				unsafe { tendril::StrTendril::from_byte_slice_without_validating(id.as_bytes(),) };

			matches!(a.name, QualName { local: local_name!("id"), .. }) && (a.value == value)
		},);
		if a.is_some() {
			Diagnostic::new(Level::Note, format!("{:#?}", node.children.borrow().len()),).emit();
		}
		a.is_some()
	} else {
		false
	};

	if found {
		// replace `node.children` to newly allocated data
		// if do not do this, `node.children` of returned value by query_by_id may be empty array
		// because `node` can be freed.
		// this occurs due to borrowing from `RefCell` at recursion which is freed after existing
		// scope
		let children = node.children.clone().into_inner();
		Diagnostic::new(Level::Note, format!("children len: {}", children.len()),).emit();
		node.children.replace(children,);
		Diagnostic::new(
			Level::Note,
			format!("node.children address: {}", node.children.borrow().len()),
		)
		.emit();
		//let node = Rc::new(*node.deref(),);

		Diagnostic::new(Level::Note, "===========================",).emit();
		Some(node,)
	} else {
		Diagnostic::new(Level::Note, "---------------------------",).emit();
		let rslt = node.children.borrow().iter().find_map(|n| query_by_id(n.clone(), id,),);
		if let Some(ref n,) = rslt {
			Diagnostic::new(Level::Note, format!("lenlen: {}", n.children.borrow().len()),);
		}
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
