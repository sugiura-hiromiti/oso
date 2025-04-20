use crate::Rslt;
use crate::chibi_uefi::table::boot_services;
use crate::raw::protocol::TextOutputProtocol;
use crate::wfi;
use core::arch::asm;
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

use super::Handle;
use super::table::system_table;

static CONSOLE: AtomicPtr<TextOutputProtocol,> = AtomicPtr::new(ptr::null_mut(),);

/// setup console
pub(crate) fn init() -> Rslt<(),> {
	// let bt = boot_services();
	// let handlers = unsafe { bt.handle_for_protocol::<TextOutputProtocol>() }?;
	// let handle = unsafe { Handle::from_ptr(handlers[0],) }.unwrap();
	//
	// //  WARN: this may drop after exitting this function
	// let console = bt.open_protocol_exclusive::<TextOutputProtocol>(handle,)?;
	//
	// unsafe { set_console(console.interface(),) };
	//
	// console_mut().output("0w0",)?;

	let st = unsafe { system_table().as_ref() };

	unsafe { st.stdout.as_mut() }.unwrap().clear()?;
	unsafe { st.stdout.as_mut() }.unwrap().output("0w0 0w0 0w0 0w0 0w0 -v-",)?;

	// let handle = unsafe { Handle::from_ptr(st.stdout_handle,) }.unwrap();

	// let bs = boot_services();
	// let pi = bs.open_protocol_exclusive::<TextOutputProtocol>(handle,)?;
	// wfi();
	// unsafe { pi.interface().as_mut() }.output("0w0",)?;
	Ok((),)
}

pub fn console() -> &'static TextOutputProtocol {
	unsafe { CONSOLE.load(Ordering::Acquire,).as_ref() }.unwrap()
}

pub fn console_mut() -> &'static mut TextOutputProtocol {
	unsafe { CONSOLE.load(Ordering::Acquire,).as_mut() }.unwrap()
}

pub unsafe fn set_console(top: NonNull<TextOutputProtocol,>,) {
	CONSOLE.store(top.as_ptr(), Ordering::Release,);
	// unsafe { CONSOLE.load(Ordering::Acquire,).as_mut() }
	// 	.unwrap()	// console is null
	// 	.output("0w0",)
	// 	.unwrap();
	// failed to output string
}

impl core::fmt::Write for TextOutputProtocol {
	fn write_str(&mut self, s: &str,) -> core::fmt::Result {
		self.output(s,)?;
		Ok((),)
	}
}
