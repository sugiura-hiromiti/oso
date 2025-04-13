use super::Boolean;

#[repr(C)]
pub struct Time {
	year:        u16,
	month:       u8,
	day:         u8,
	hour:        u8,
	minute:      u8,
	second:      u8,
	pad1:        u8,
	nano_second: u32,
	time_zone:   u16,
	daylight:    u8,
	pad2:        u8,
}

#[repr(C)]
pub struct TimeCapabilities {
	resolution:   u32,
	accuracy:     u32,
	sets_to_zero: Boolean,
}

#[repr(C)]
pub enum TimerDelay {
	Cancel,
	Periodic,
	Relative,
}
