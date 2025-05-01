use crate::Rslt;
use crate::chibi_uefi::guid::AsBytes;
use crate::chibi_uefi::guid::Hex;
use crate::chibi_uefi::guid::read_to_hex;
use crate::chibi_uefi::protocol::Protocol;
use crate::chibi_uefi::table::boot_services;
use crate::guid;
use crate::print;
use crate::println;
use crate::raw::protocol::file::FileProtocolV1;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::Guid;
use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::FileInfo;
use crate::raw::types::file::OpenMode;
use core::ptr::NonNull;

pub fn kernel() -> Rslt<PhysicalAddress,> {
	let mut kernel_file = open_kernel_file()?;
	let contents = unsafe { kernel_file.as_mut() }.read_as_bytes()?;

	todo!("nyi!!!!");
}

fn open_kernel_file() -> Rslt<NonNull<FileProtocolV1,>,> {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	let bs = boot_services();
	let sfs_handle = unsafe { bs.handle_for_protocol::<SimpleFileSystemProtocol>() }?;

	let volume = unsafe {
		bs.open_protocol_exclusive::<SimpleFileSystemProtocol>(sfs_handle,)?.interface().as_mut()
	}
	.open_volume()?;
	let kernel_file = volume.open("oso_kernel.elf", open_mode, attrs,)?;
	let non_null_kernel_file = NonNull::new(kernel_file,).expect("reference can't be null",);
	Ok(non_null_kernel_file,)
}
