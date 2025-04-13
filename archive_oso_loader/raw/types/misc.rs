#[repr(u32)]
pub enum ResetType {
	ResetCold             = 0,
	ResetWarm             = 1,
	ResetShutdown         = 2,
	ResetPlatformSpecific = 3,
}
