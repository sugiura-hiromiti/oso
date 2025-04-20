use crate::c_style_enum;
use core::ops::RangeInclusive;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,)]
pub struct MemoryDescriptor {
	pub memory_type:    MemoryType,
	pub physical_start: u64,
	pub virtual_start:  u64,
	pub page_count:     u64,
	pub attribute:      MemoryAttribute,
}

c_style_enum! {
	pub enum AllocateType: isize => {
		ALLOCATE_ANY_PAGES   = 0,
		ALLOCATE_MAX_ADDRESS = 1,
		ALLOCATE_ADDRESS     = 2,
		MAX_ALLOCATE_TYPE    = 3,
	}
}

c_style_enum! {
	pub enum MemoryType: u32 => {
		RESERVED              = 0,
		LOADER_CODE           = 1,
		LOADER_DATA           = 2,
		BOOT_SERVICES_CODE    = 3,
		BOOT_SERVICES_DATA    = 4,
		RUNTIME_SERVICES_CODE = 5,
		RUNTIME_SERVICES_DATA = 6,
		CONVENTIONAL          = 7,
		UNUSABLE              = 8,
		ACPI_RECLAIM          = 9,
		ACPI_NON_VOLATILE     = 10,
		MMIO                  = 11,
		MMIO_PORT_SPACE       = 12,
		PAL_CODE              = 13,
		PERSISTENT_MEMORY     = 14,
		UNACCEPTED            = 15,
		MAX                   = 16,
	}
}

impl MemoryType {
	pub const RESERVED_FOR_OEM: RangeInclusive<u32,> = 0x7000_0000..=0x7fff_ffff;
	pub const RESERVED_FOR_OS_LOADER: RangeInclusive<u32,> = 0x8000_0000..=0xffff_ffff;

	pub const fn custom(value: u32,) -> Self {
		assert!(value >= 0x8000_0000);
		Self(value,)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,)]
#[repr(transparent)]
pub struct MemoryAttribute(pub u64,);

impl MemoryAttribute {
	pub const EFI_MEMORY_CPU_CRYPTO: u64 = 0x0000000000080000;
	pub const EFI_MEMORY_HOT_PLUGGABLE: u64 = 0x0000000000100000;
	pub const EFI_MEMORY_ISA_MASK: u64 = 0x0FFFF00000000000;
	pub const EFI_MEMORY_ISA_VALID: u64 = 0x4000000000000000;
	pub const EFI_MEMORY_MORE_RELIABLE: u64 = 0x0000000000010000;
	pub const EFI_MEMORY_NV: u64 = 0x0000000000008000;
	pub const EFI_MEMORY_RO: u64 = 0x0000000000020000;
	pub const EFI_MEMORY_RP: u64 = 0x0000000000002000;
	pub const EFI_MEMORY_RUNTIME: u64 = 0x8000000000000000;
	pub const EFI_MEMORY_SP: u64 = 0x0000000000040000;
	pub const EFI_MEMORY_UC: u64 = 0x0000000000000001;
	pub const EFI_MEMORY_UCE: u64 = 0x0000000000000010;
	pub const EFI_MEMORY_WB: u64 = 0x0000000000000008;
	pub const EFI_MEMORY_WC: u64 = 0x0000000000000002;
	pub const EFI_MEMORY_WP: u64 = 0x0000000000001000;
	pub const EFI_MEMORY_WT: u64 = 0x0000000000000004;
	pub const EFI_MEMORY_XP: u64 = 0x0000000000004000;
}
