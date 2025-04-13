#[repr(C)]
pub enum ResetType {
	ResetCold             = 0u32,
	ResetWarm             = 1u32,
	ResetShutdown         = 2u32,
	ResetPlatformSpecific = 3u32,
}
