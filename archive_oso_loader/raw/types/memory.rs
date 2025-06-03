#[repr(C)]
pub struct MemoryDescriptor {
	memory_type:     u32,
	physical_start:  u64,
	virtual_start:   u64,
	number_of_pages: u64,
	attribute:       u64,
}

#[repr(C)]
pub enum AllocateType {
	AnyPages,
	MaxAddress,
	Address,
	MaxAllocateType,
}

#[repr(C)]
pub enum MemoryType {
	ReservedMemoryType,
	LoaderCode,
	LoaderData,
	BootServicesCode,
	BootServicesData,
	RuntimeServicesCode,
	RuntimeServicesData,
	ConvertionalMemory,
	UnstableMemory,
	AcpiReclaimMemory,
	AcpiMemoryNvs,
	MemoryMappedIo,
	MemoryMappedIoPortSpace,
	PalCode,
	PersistentMemory,
	UnacceptedMemoryType,
	MaxMemoryType,
}
