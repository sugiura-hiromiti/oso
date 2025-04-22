use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::OpenMode;

pub fn kernel() -> PhysicalAddress {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	//let kernel_file;
	todo!()
}
