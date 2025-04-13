use core::ffi::c_void;

use super::protocol::DevicePathProtocol;
use super::protocol::InterfaceType;
use super::protocol::LocateSearchType;
use super::protocol::OpenProtocolInformationEntry;
use super::types::Boolean;
use super::types::Guid;
use super::types::Header;
use super::types::Status;
use super::types::capsule::CapsuleHeader;
use super::types::memory::AllocateType;
use super::types::memory::MemoryDescriptor;
use super::types::memory::MemoryType;
use super::types::misc::ResetType;
use super::types::time::Time;
use super::types::time::TimeCapabilities;
use super::types::time::TimerDelay;

#[repr(C)]
pub struct RuntimeServices {
	header: Header,

	// time services
	get_time:        unsafe extern "efiapi" fn(time: *mut Time, *mut TimeCapabilities,) -> Status,
	set_time:        unsafe extern "efiapi" fn(time: *const Time,) -> Status,
	get_wakeup_time: unsafe extern "efiapi" fn(
		enabled: *mut Boolean,
		pending: *mut Boolean,
		time: *mut Time,
	) -> Status,
	set_wakeup_time: unsafe extern "efiapi" fn(enable: Boolean, time: *mut Time,) -> Status,

	// virtual memory services
	set_virtual_address_map: unsafe extern "efiapi" fn(
		memory_map_size: usize,
		descriptor_size: usize,
		descriptor_version: u32,
		virtual_map: *mut MemoryDescriptor,
	) -> Status,
	convert_pointer:
		unsafe extern "efiapi" fn(debug_dispositioon: usize, *mut *const c_void,) -> Status,

	// variable services
	get_variable: unsafe extern "efiapi" fn(
		variable_name: *const u16,
		vendor_guid: *const Guid,
		attributes: *mut u32,
		data_size: *mut usize,
		data: *mut c_void,
	) -> Status,
	get_next_variable_name: unsafe extern "efiapi" fn(
		variable_name_size: *mut usize,
		variable_name: *mut u16,
		vendor_guid: *mut Guid,
	) -> Status,
	set_variable: unsafe extern "efiapi" fn(
		variable_name: *const u16,
		vendor_guid: *const Guid,
		attributes: u32,
		data_size: usize,
		data: *const c_void,
	) -> Status,

	// miscellaneous
	get_next_high_monotonic_count: unsafe extern "efiapi" fn(count: *mut u64,) -> Status,
	reset_system: unsafe extern "efiapi" fn(
		reset_type: ResetType,
		reset_status: Status,
		data_size: usize,
		reset_data: *const c_void,
	) -> Status,

	// uefi 2.0 capsule services
	update_capsule: unsafe extern "efiapi" fn(
		capsule_header_array: *const *const CapsuleHeader,
		capsule_count: usize,
		scatter_gather_list: u64,
	) -> Status,
	query_capsule_capabilities: unsafe extern "efiapi" fn(
		capsule_heaader_array: *const *const CapsuleHeader,
		capsule_count: usize,
		maximum_capsule_size: *mut u64,
		reset_type: *mut ResetType,
	) -> Status,

	// miscellaneous uefi 2.0 services
	query_variable_info: unsafe extern "efiapi" fn(
		attributes: u32,
		maximum_variable_storage_size: *mut u64,
		remaining_variable_storage_size: *mut u64,
		maximum_variable_size: *mut u64,
	) -> Status,
}

#[repr(C)]
pub struct BootServices {
	header: Header,

	// task priority services
	raise_tpl:   unsafe extern "efiapi" fn(new_tpl: usize,) -> usize,
	restore_tpl: unsafe extern "efiapi" fn(old_tpl: usize,),

	// memory services
	allocate_pages: unsafe extern "efiapi" fn(
		allocate_type: AllocateType,
		memory_type: MemoryType,
		pages: usize,
		memory: *mut u64,
	) -> Status,
	free_pages:     unsafe extern "efiapi" fn(memory: u64, pages: usize,) -> Status,
	get_memory_map: unsafe extern "efiapi" fn(
		memory_map_size: *mut usize,
		memory_map: *mut MemoryDescriptor,
		map_key: *mut usize,
		descriptor_size: *mut usize,
		descriptor_version: *mut u32,
	) -> Status,
	allocate_pool: unsafe extern "efiapi" fn(
		pool_type: MemoryType,
		size: usize,
		buffer: *mut *mut u8,
	) -> Status,
	free_pool:      unsafe extern "efiapi" fn(buffer: *mut u8,) -> Status,

	// event & timer services
	create_event: unsafe extern "efiapi" fn(
		event_type: u32,
		notify_tpl: usize,
		notify_function: Option<
			unsafe extern "efiapi" fn(event: *mut c_void, context: *const c_void,),
		>,
		notify_context: *const c_void,
		event: *mut *mut c_void,
	) -> Status,
	set_timer: unsafe extern "efiapi" fn(
		event: *mut c_void,
		time_type: TimerDelay,
		trigger_time: u64,
	) -> Status,
	wait_for_event: unsafe extern "efiapi" fn(
		number_of_events: usize,
		event: *mut *mut c_void,
		index: *mut usize,
	) -> Status,
	signal_event:   unsafe extern "efiapi" fn(event: *mut c_void,) -> Status,
	close_event:    unsafe extern "efiapi" fn(event: *mut c_void,) -> Status,
	check_event:    unsafe extern "efiapi" fn(event: *mut c_void,) -> Status,

	// protocol handler services

	// `Guid` definition is https://docs.rs/uguid/2.2.1/src/uguid/guid.rs.html#40-52
	install_protocol_interface: unsafe extern "efiapi" fn(
		handle: *mut *mut c_void,
		protocol: *const Guid,
		interface_type: InterfaceType,
		interface: *const c_void,
	) -> Status,
	reinstall_protocol_interface: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const Guid,
		old_interface: *const c_void,
		new_interface: *const c_void,
	) -> Status,
	uninstall_protocol_interface: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const c_void,
		interface: *const c_void,
	) -> Status,
	handle_protocol: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const Guid,
		interface: *mut *mut c_void,
	) -> Status,
	reserved:                     *mut c_void,
	register_protocol_notify: unsafe extern "efiapi" fn(
		protocol: *const Guid,
		event: *mut c_void,
		registration: *mut *const c_void,
	) -> Status,
	locate_handle: unsafe extern "efiapi" fn(
		search_type: LocateSearchType,
		protocol: *const Guid,
		search_key: *const c_void,
		buffer_size: *mut usize,
		buffer: *mut *mut c_void,
	) -> Status,
	locate_device_path: unsafe extern "efiapi" fn(
		protocol: *const Guid,
		device_path: *mut *const DevicePathProtocol,
		device: *mut *mut c_void,
	) -> Status,
	install_configuration_table:
		unsafe extern "efiapi" fn(guid: *const Guid, table: *const c_void,) -> Status,

	// image services
	load_image: unsafe extern "efiapi" fn(
		boot_policy: Boolean,
		parent_image_handle: *mut c_void,
		device_path: *const DevicePathProtocol,
		source_buffer: *const c_void,
		source_size: usize,
		image_handle: *mut c_void,
	) -> Status,
	start_image: unsafe extern "efiapi" fn(
		image_handle: *mut c_void,
		exit_data_size: *mut usize,
		exit_data: *mut *mut u16,
	) -> Status,
	exit: unsafe extern "efiapi" fn(
		image_handle: *mut c_void,
		exit_status: Status,
		exit_data_size: usize,
		exit_data: *mut u16,
	) -> Status,
	unload_image:       unsafe extern "efiapi" fn(image_handle: *mut c_void,) -> Status,
	exit_boot_services:
		unsafe extern "efiapi" fn(image_handle: *mut c_void, map_key: usize,) -> Status,

	// miscellaneous services
	get_next_monotonic_count: unsafe extern "efiapi" fn(count: *mut u64,) -> Status,
	stall:                    unsafe extern "efiapi" fn(micro_second: usize,) -> Status,
	set_watchdog_timer: unsafe extern "efiapi" fn(
		timeout: usize,
		watchdog_code: u64,
		data_size: usize,
		watchdog_data: *const u16,
	) -> Status,
	connect_controller: unsafe extern "efiapi" fn(
		controller_handle: *mut c_void,
		driver_image_handle: *mut c_void,
		remaining_device_path: *const DevicePathProtocol,
		recursive: Boolean,
	) -> Status,
	disconnect_controller: unsafe extern "efiapi" fn(
		controller_handle: *mut c_void,
		driver_image_handle: *mut c_void,
		child_handle: *mut c_void,
	) -> Status,

	// open and close protocol services
	open_protocol: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const Guid,
		interface: *mut *mut c_void,
		agent_handle: *mut c_void,
		controller_handle: *mut c_void,
		attributes: u32,
	) -> Status,
	close_protocol: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const Guid,
		agent_handle: *mut c_void,
		controller_handle: *mut c_void,
	) -> Status,
	open_protocol_information: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol: *const Guid,
		entry_buffer: *mut *const OpenProtocolInformationEntry,
		entry_count: *mut usize,
	) -> Status,

	// library services
	protocols_per_handle: unsafe extern "efiapi" fn(
		handle: *mut c_void,
		protocol_buffer: *mut *mut *const Guid,
		protocol_buffer_count: *mut usize,
	) -> Status,
	locate_handle_buffer: unsafe extern "efiapi" fn(
		search_type: LocateSearchType,
		protocol: *const Guid,
		search_key: *const c_void,
		handles_count: *mut usize,
		buffer: *mut *mut *mut c_void,
	) -> Status,
	locate_protocol: unsafe extern "efiapi" fn(
		protocol: *const Guid,
		registration: *mut c_void,
		interface: *mut *mut c_void,
	) -> Status,
	install_multiple_protocol_interfaces:
		unsafe extern "C" fn(handle: *mut *mut c_void, ...) -> Status,
	uninstall_multiple_protocol_interfaces:
		unsafe extern "C" fn(handle: *mut c_void, ...) -> Status,

	// crc services
	calculate_crc32:
		unsafe extern "efiapi" fn(data: *const c_void, data_size: usize, crc32: *mut u32,)
			-> Status,

	// misc services
	copy_mem: unsafe extern "efiapi" fn(dest: *mut u8, source: *const u8, len: usize,) -> Status,
	set_mem:         unsafe extern "efiapi" fn(buf: *mut u8, size: usize, value: u8,) -> Status,
	create_event_ex: unsafe extern "efiapi" fn(
		event_type: u32,
		notify_tpl: usize,
		notify_function: Option<unsafe extern "efiapi" fn(event: Event, context: *const c_void,),>,
		notify_context: *mut c_void,
		event_group: *mut Guid,
		event: *mut *mut c_void,
	) -> Status,
}
