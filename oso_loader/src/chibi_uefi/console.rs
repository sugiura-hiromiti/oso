use crate::Rslt;
use crate::chibi_uefi::table::boot_services;
use crate::raw::protocol::TextOutputProtocol;
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

static CONSOLE: AtomicPtr<TextOutputProtocol,> = AtomicPtr::new(ptr::null_mut(),);

/// setup console
pub(crate) fn init() -> Rslt<(),> {
	let bt = boot_services();
	let handlers = unsafe { bt.handle_for_protocol::<TextOutputProtocol>() }?;
	//  WARN: this may drop after exitting this function
	let console = bt.open_protocol_exclusive::<TextOutputProtocol>(handlers[0].clone(),)?;
	unsafe { set_console(console.interface(),) };

	unsafe { CONSOLE.load(Ordering::Acquire,).as_mut() }.unwrap().output("0w0",)?;
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
