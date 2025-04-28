use crate::Rslt;
use crate::chibi_uefi::protocol::Protocol;
use crate::chibi_uefi::table::boot_services;
use crate::print;
use crate::println;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::FileInfo;
use crate::raw::types::file::OpenMode;

pub fn kernel() -> Rslt<PhysicalAddress,> {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	let bs = boot_services();
	{
		let guid = SimpleFileSystemProtocol::GUID;
		assert_eq!(guid.time_low, 0x964e5b22);
		assert_eq!(guid.time_mid, [0x59, 0x64]);
		assert_eq!(guid.time_high_and_version, [0xd2, 0x64]);
		assert_eq!(guid.clock_seq_high_and_reserved, 0x8e);
		assert_eq!(guid.clock_seq_low, 0x39);
		assert_eq!(guid.node, [0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
	}

	let sfs_handle = unsafe { bs.handle_for_protocol::<SimpleFileSystemProtocol>() }?;

	let kernel_file = unsafe {
		bs.open_protocol_exclusive::<SimpleFileSystemProtocol>(sfs_handle,)?.interface().as_mut()
	}
	.open_volume()?
	.open("kernel.elf", open_mode, attrs,)?;
	let file_info = kernel_file.get_file_info()?;
	todo!("{file_info:?}");
}
