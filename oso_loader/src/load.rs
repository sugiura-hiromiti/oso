use crate::Rslt;
use crate::guid;
use crate::chibi_uefi::protocol::Protocol;
use crate::chibi_uefi::table::boot_services;
use crate::print;
use crate::println;
use crate::raw::protocol::file::SimpleFileSystemProtocol;
use crate::raw::types::Guid;
use crate::raw::types::PhysicalAddress;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::FileInfo;
use crate::raw::types::file::OpenMode;

pub fn kernel() -> Rslt<PhysicalAddress,> {
	let open_mode = OpenMode::READ;
	let attrs = FileAttributes(0,);

	let bs = boot_services();
	{
		assert_eq!(
			guid!("01234567-89ab-cdef-0123-456789abcdef"),
			Guid::new(
				[0x67, 0x45, 0x23, 0x01],
				[0xab, 0x89],
				[0xef, 0xcd],
				0x01,
				0x23,
				[0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
			)
		);
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
