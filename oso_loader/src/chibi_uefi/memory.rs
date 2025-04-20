use super::table::boot_services;
use super::table::system_table;
use crate::Rslt;
use crate::raw::service::BootServices;
use crate::raw::types::Status;
use crate::raw::types::memory::MemoryType;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr::NonNull;

#[global_allocator]
static LOADER_ALLOCATOR: LoaderAllocator = LoaderAllocator;

pub struct LoaderAllocator;

unsafe impl GlobalAlloc for LoaderAllocator {
	unsafe fn alloc(&self, layout: core::alloc::Layout,) -> *mut u8 {
		if layout.align() > 8 {
			panic!()
		}
		let mem_ty = MemoryType::LOADER_DATA;
		let bs = boot_services();
		bs.allocate_pool(mem_ty, layout.size(),).expect("allocation failed",).as_ptr()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout,) {
		if layout.align() > 8 {
			panic!()
		}
		let bs = boot_services();
		bs.free_pool(unsafe { ptr.as_mut_unchecked() },).expect("deallocation failed",);
	}
}

#[alloc_error_handler]
fn alloc_error(layout: Layout,) -> ! {
	panic!("system run out of memory: {layout:#?}")
}

impl BootServices {
	pub fn allocate_pool(&self, mem_ty: MemoryType, size: usize,) -> Rslt<NonNull<u8,>,> {
		let mut buf = core::ptr::null_mut();
		unsafe { (self.allocate_pool)(mem_ty, size, &mut buf,).ok_or()? };
		Ok(NonNull::new(buf,).expect("allocate_pool must not return a null pointer if successful",),)
	}

	pub fn free_pool(&self, ptr: &mut u8,) -> Rslt<Status,> {
		unsafe { (self.free_pool)(ptr,).ok_or() }
	}
}
