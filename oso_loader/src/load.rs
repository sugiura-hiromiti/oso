use crate::Rslt;
use crate::chibi_uefi::table::boot_services;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::OpenMode;

pub fn kernel() -> Rslt<PhysicalAddress,> {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	let bs = boot_services();
	let sfs_handle = unsafe { bs.handle_for_protocol::<SimpleFileSystemProtocol>() }?;

	let kernel_file = unsafe {
		bs.open_protocol_exclusive::<SimpleFileSystemProtocol>(sfs_handle,)?.interface().as_mut()
	}
	.open_volume()?
	.open("kernel.elf", open_mode, attrs,)?;
	todo!()
}
