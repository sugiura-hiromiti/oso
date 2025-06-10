#![no_std]
#![feature(alloc_error_handler)]
#![feature(ptr_as_ref_unchecked)]
#![feature(iter_next_chunk)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![feature(assert_matches)]
// #![feature(nonzero_internals)]
//#![feature(stdarch_arm_hints)]

extern crate alloc;

use alloc::vec::Vec;
use chibi_uefi::protocol::HandleSearchType;
use chibi_uefi::table::boot_services;
use core::arch::asm;
use error::OsoLoaderError;
use oso_bridge::graphic::FrameBufConf;
use oso_bridge::wfe;
use oso_bridge::wfi;
use raw::table::SystemTable;
use raw::types::Status;
use raw::types::UnsafeHandle;

pub mod chibi_uefi;
pub mod elf;
pub mod error;
pub mod load;
pub mod raw;

pub type Rslt<T = Status,> = Result<T, OsoLoaderError,>;

#[panic_handler]
fn panic(panic: &core::panic::PanicInfo,) -> ! {
	println!("{panic:#?}");
	wfe();
}

#[macro_export]
/// ?演算子で処理できないエラーがあった場合に使う
macro_rules! on_error {
	($e:ident, $situation:expr) => {{
		log::error!("error happen {}", $situation);
		log::error!("error msg:");
		log::error!("{}", $e);
	}};
}

/// # Panics
///
/// panics  when initialization failed
pub fn init(image_handle: UnsafeHandle, syst: *const SystemTable,) {
	unsafe { syst.as_ref().unwrap().stdout.as_mut().unwrap().clear().unwrap() };
	chibi_uefi::table::set_system_table_panicking(syst,);
	chibi_uefi::set_image_handle_panicking(image_handle,);

	// connect devices
	let bs = boot_services();

	// uefi only installs DevicePathProtocol on devices that are fully connected
	// `AllHandles` is the only way to find unconnected devices
	let handles = unsafe {
		bs.locate_handle_buffer(HandleSearchType::AllHandles,)
			.expect("failed to locate all handles ",)
	};
	handles.iter().for_each(|handle| {
		// ignore errors from connect_controller intendly
		unsafe { bs.connect_controller(*handle, None, None, raw::types::Boolean::TRUE,) };
	},);
}

fn into_null_terminated_utf16(s: impl AsRef<str,>,) -> Vec<u16,> {
	let mut utf16_repr: Vec<u16,> = s.as_ref().encode_utf16().collect();
	utf16_repr.push(0,);
	utf16_repr
}

pub fn exec_kernel(kernel_entry: u64, _graphic_config: FrameBufConf,) {
	// これ要る?
	let kernel_entry = kernel_entry as *const ();

	#[cfg(target_arch = "riscv64")]
	type KernelEntry = extern "C" fn();
	#[cfg(target_arch = "aarch64")]
	type KernelEntry = extern "C" fn();
	#[cfg(target_arch = "x86_64")]
	type KernelEntry = extern "sysv64" fn();

	let entry_point = unsafe { core::mem::transmute::<_, KernelEntry,>(kernel_entry,) };

	#[cfg(target_arch = "aarch64")]
	unsafe {
		// 全てのデータアクセスが完了するまで待機
		asm!("dsb sy");

		// 念の為キャッシュを全削除
		asm!("ic iallu"); // 命令キャッシュを全て無効にする
		asm!("dsb ish"); // ↑が完了するまで待機
		asm!("isb"); // キャッシュクリア後に再度命令を読み込む
		// ↑既にキャッシュを読み込んでいるかもしれないため、リロードする必要がある

		// Disable MMU by modifying SCTLR_EL1
		asm!(
			"mrs x0, sctlr_el1", // 現在のMMUの状態をx0レジスタに読み込む。有効になっているはず
			"bic x0, x0, #1", // x0レジスタにx0レジスタの持つ値の最下位ビットをクリアした値をセット
			// この値は、MMUが無効である状態を表す
			"msr sctlr_el1, x0", // x0の値を反映。MMUを実際に無効化している行
			"isb", // システムの状態を変更したので命令をリロードする
			out("x0") _
		);
	}

	// Jump to kernel with MMU disabled
	entry_point();

	// 失敗したら到達する
	wfi();
}
