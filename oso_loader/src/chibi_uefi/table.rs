use crate::raw::service::BootServices;
use crate::raw::table::SystemTable;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

static SYSTEM_TABLE: AtomicPtr<SystemTable,> = AtomicPtr::new(core::ptr::null_mut(),);

pub(crate) unsafe fn set_system_table(ptr: *const SystemTable,) {
	SYSTEM_TABLE.store(ptr.cast_mut(), Ordering::Release,);
}

pub fn system_table() -> NonNull<SystemTable,> {
	let p = SYSTEM_TABLE.load(Ordering::Acquire,);
	NonNull::new(p,).expect("set_system_table has not been called",)
}

pub fn boot_services<'a,>() -> &'a BootServices {
	let syst = system_table();
	unsafe { syst.as_ref().boot_services.as_ref() }.unwrap()
}
