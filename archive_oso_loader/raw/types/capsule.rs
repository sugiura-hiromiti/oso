use super::Guid;

#[repr(C)]
pub struct CapsuleHeader {
	capsule_guid:       Guid,
	header_size:        u32,
	flags:              u32,
	capsule_image_size: u32,
}
