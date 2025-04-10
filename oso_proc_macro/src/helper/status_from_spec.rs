use anyhow::Result as Rslt;
use std::net::TcpStream;
use std::sync::Arc;

pub fn status_spec_page(status_spec_url: String,) {
	let rsp = ureq::get(status_spec_url,).call()?;
}

// struct HttpClient{
// 	inner Arc<ClientRef>,
// }
//
// struct ClientRef{
// 	headers:HeaderMap,
// }
//
// #[derive(Clone)]
// struct HeaderMap<T=HeaderValue>{
// 	mask: Size,
// 	indices: Box<[Pos]>,
// 	entries: Vec<Bucket<T>>,
// 	extra_values: Vec<Extra>
// }
