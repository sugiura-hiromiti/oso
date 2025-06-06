# Troubleshooting aarch64 Kernel Execution in UEFI Environment: The MMU Mystery

## Introduction

As a systems programmer diving into OS development, I've been working on a project called "oso" - a full-scratch, Pure Rust, aarch64-targeted operating system. When working at this level of abstraction, you're often navigating uncharted territory where documentation is sparse and solutions require deep understanding of hardware-software interactions.

In this technical deep-dive, I'll analyze a particularly challenging issue I encountered with aarch64 kernel execution and the architectural differences that caused it. This article is intended for:

- OS developers working with UEFI environments
- Rust developers targeting bare metal systems
- Engineers working on aarch64 architecture
- Anyone interested in low-level system architecture differences between x86_64 and aarch64

## The Problem: Kernel Execution Failure on aarch64

The issue manifested when my kernel, which executed flawlessly on x86_64, failed to run properly on aarch64. The symptoms were:

1. The bootloader successfully loaded the kernel ELF file
2. The bootloader correctly identified the entry point (0x40010120)
3. The bootloader successfully exited UEFI boot services
4. The jump to the kernel entry point occurred
5. But the kernel code never executed as expected

What made this particularly intriguing was the architecture-specific nature of the problem - identical code worked on x86_64 but failed on aarch64, suggesting a fundamental architectural difference was at play.

## Project Architecture

My "oso" project follows a modular architecture with several key components:

```
oso/
├── oso_loader/       # UEFI bootloader (Rust)
├── oso_kernel/       # Kernel implementation (Rust)
├── oso_bridge/       # Shared code and interfaces
├── xtask/            # Build scripts and utilities
└── target/           # Build artifacts
```

### Bootloader Implementation

The bootloader's execution flow is implemented as follows:

```rust
pub fn boot(&mut self) -> ! {
    // Initialize UEFI environment
    self.init_uefi();
    
    // Load kernel ELF from disk
    let kernel_elf = self.load_kernel_elf();
    
    // Parse ELF and prepare for execution
    let entry_point = kernel_elf.entry_point_address();
    
    // Prepare memory for kernel
    self.prepare_memory_for_kernel();
    
    // Exit UEFI boot services
    self.exit_boot_services(None);
    
    // Jump to kernel entry point
    self.jump_to_kernel(entry_point);
    
    // Should never reach here
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

### Kernel Entry Point

The kernel entry point is defined in `oso_kernel/src/main.rs`:

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // Disable interrupts
    unsafe {
        asm!("msr daifset, #2");
    }
    
    // Initialize kernel components
    init_platform();
    
    // Enter wait loop for debugging
    loop {
        unsafe { asm!("wfe"); }
    }
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub extern "C" fn kernel_main() -> ! {
    // x86_64 specific initialization
    unsafe {
        asm!("cli");  // Disable interrupts
    }
    
    // Initialize kernel components
    init_platform();
    
    // Enter wait loop for debugging
    loop {
        unsafe { asm!("hlt"); }
    }
}
```

## Systematic Investigation

To diagnose this issue, I employed a systematic approach to rule out potential causes:

### 1. Verifying ELF Loading and Entry Point

First, I needed to confirm the kernel ELF file was being loaded correctly and the entry point was accurately identified. I used `readelf` to examine the kernel binary:

```bash
$ readelf -h target/oso_kernel.elf
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           AArch64
  Version:                           0x1
  Entry point address:               0x40010120
  Start of program headers:          64 (bytes into file)
  Start of section headers:          2656688 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         4
  Size of section headers:           64 (bytes)
  Number of section headers:         14
  Section header string table index: 12
```

I also examined the program headers to understand the memory layout:

```bash
$ readelf -l target/oso_kernel.elf
Elf file type is EXEC (Executable file)
Entry point 0x40010120
There are 4 program headers, starting at offset 64

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  PHDR           0x0000000000000040 0x0000000040000040 0x0000000040000040
                 0x00000000000000e0 0x00000000000000e0  R      0x8
  LOAD           0x0000000000000000 0x0000000040000000 0x0000000040000000
                 0x0000000000000120 0x0000000000000120  R      0x10000
  LOAD           0x0000000000000120 0x0000000040010120 0x0000000040010120
                 0x0000000000000010 0x0000000000000010  R E    0x10000
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x0
```

To verify the actual code at the entry point, I disassembled the `.text` section:

```bash
$ readelf --hex-dump=.text target/oso_kernel.elf
Hex dump of section '.text':
  0x40010120 ff4300d1 01000014 5f2003d5 ffffff17 .C......_ ......
```

This translates to the following aarch64 assembly:

```asm
0x40010120:  d10043ff  sub      sp, sp, #0x10
0x40010124:  14000001  b        #0x40010128
0x40010128:  d503205f  wfe
0x4001012c:  17ffffff  b        #0x40010128
```

This confirmed that my kernel code was being loaded correctly and the entry point address was accurate.

### 2. Validating Boot Services Exit

Next, I needed to ensure that UEFI boot services were being properly exited. My implementation of `exit_boot_services()` looked like this:

```rust
pub unsafe fn exit_boot_services(custom_memory_type: Option<MemoryType>) -> MemoryMapOwned {
    let memory_type = custom_memory_type.unwrap_or(MemoryType::LOADER_DATA);
    
    // Allocate memory for the memory map
    let mut buf = MemoryMapBackingMemory::new(memory_type)
        .expect("Failed to allocate memory");
    
    let mut status = Status::ABORTED;
    
    // Try to exit boot services (may need multiple attempts)
    for _ in 0..2 {
        match unsafe { get_memory_map_and_exit_boot_services(buf.as_mut_slice()) } {
            Ok(memory_map) => {
                return MemoryMapOwned::from_initialized_mem(buf, memory_map);
            }
            Err(err) => {
                status = err.status()
            }
        }
    }
    
    // If we reach here, we failed to exit boot services
    runtime::reset(ResetType::COLD, status, None);
}
```

The helper function `get_memory_map_and_exit_boot_services` was implemented as:

```rust
unsafe fn get_memory_map_and_exit_boot_services(buf: &mut [u8]) -> Result<MemoryMapMeta> {
    let bt = boot_services_raw_panicking();
    let bt = unsafe { bt.as_ref() };
    
    // Get the current memory map
    let memory_map = get_memory_map(buf)?;
    
    // Exit boot services using the memory map key
    unsafe { (bt.exit_boot_services)(image_handle().as_ptr(), memory_map.map_key.0) }
        .to_result_with_val(|| memory_map)
}
```

I added debug logging before the exit_boot_services call and confirmed it was succeeding without errors.

### 3. Examining the Jump to Kernel

The jump to the kernel entry point was implemented using a function pointer:

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // Define the function type for the kernel entry point
    type KernelEntry = extern "C" fn() -> !;
    
    // Convert the entry point address to a function pointer
    let kernel_main: KernelEntry = unsafe { 
        core::mem::transmute(entry_point) 
    };
    
    // Call the kernel entry point
    kernel_main();
    
    // Should never reach here
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

To debug this, I added a binary breakpoint right before the jump:

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // Debug marker - distinctive instruction sequence
    unsafe { asm!("nop; nop; nop; nop"); }
    
    type KernelEntry = extern "C" fn() -> !;
    let kernel_main: KernelEntry = unsafe { core::mem::transmute(entry_point) };
    
    // Debug log the entry point address
    debug!("Jumping to kernel at address: {:#x}", entry_point);
    
    // Jump to kernel
    kernel_main();
    
    // Fallback loop
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

Using QEMU's monitor (accessed with Ctrl+Alt+2), I could confirm execution reached this point but never returned from the kernel_main() call, suggesting the issue occurred during or immediately after the jump.

### 4. Advanced Debugging Techniques

When working at this level, traditional debugging tools are often unavailable. I employed several specialized techniques:

#### Binary Breakpoints

I strategically placed distinctive instruction sequences that could be identified in memory dumps:

```rust
// In bootloader, before jumping to kernel
unsafe {
    asm!("mov x0, #0xDEAD");
    asm!("mov x1, #0xBEEF");
    asm!("nop; nop; nop; nop");
}

// In kernel entry point
unsafe {
    asm!("mov x2, #0xCAFE");
    asm!("mov x3, #0xBABE");
    asm!("nop; nop; nop; nop");
}
```

These sequences create recognizable patterns in memory that can be spotted when examining memory dumps.

#### QEMU Monitor Commands

QEMU's monitor provides powerful debugging capabilities. I used these commands:

```
info registers        # Display current register values
x/10i $pc            # Disassemble 10 instructions at program counter
x/20wx 0x40010120    # Examine 20 words at kernel entry point
info mem              # Display memory mapping information
```

#### Custom Wait Instructions

I used different wait instructions for different purposes:

```rust
#[inline(always)]
pub fn wfi() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "aarch64")]
            asm!("wfi");  // Wait For Interrupt
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

#[inline(always)]
pub fn wfe() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "aarch64")]
            asm!("wfe");  // Wait For Event
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}
```

By using `wfi` in one location and `wfe` in another, I could determine where execution was stopping by examining the instruction at the program counter.

### 5. Examining Memory Mapping

A crucial insight came when I examined the memory mapping in QEMU. On x86_64, the virtual memory was identity-mapped (virtual address = physical address) for the entire address space. However, on aarch64, only specific regions were identity-mapped.

To verify this, I added code to dump the UEFI memory map before exiting boot services:

```rust
fn dump_memory_map(memory_map: &MemoryMap) {
    for descriptor in memory_map.entries() {
        debug!(
            "Type: {:?}, Physical: {:#x}-{:#x}, Virtual: {:#x}, Attributes: {:#x}",
            descriptor.ty,
            descriptor.phys_start,
            descriptor.phys_start + (descriptor.page_count * 4096),
            descriptor.virt_start,
            descriptor.att
        );
    }
}
```

This revealed that on aarch64, UEFI only guarantees identity mapping for memory regions it manages, not for all memory. This is a critical architectural difference from x86_64.

## The Root Cause: MMU Configuration Differences

After extensive investigation, I identified the root cause: the Memory Management Unit (MMU) configuration differs significantly between x86_64 and aarch64 UEFI implementations.

### aarch64 MMU Behavior

On aarch64, UEFI enables the MMU during boot with a specific configuration:

1. Memory regions managed by UEFI are identity-mapped
2. Other regions may not be identity-mapped
3. The MMU remains enabled after `exit_boot_services()`

When my bootloader jumped to the kernel entry point (0x40010120), this address was a virtual address. If the MMU mapping didn't have a valid translation for this address, execution would fail.

### x86_64 MMU Behavior

In contrast, on x86_64:

1. UEFI typically sets up identity mapping for the entire address space
2. The paging structures ensure virtual addresses match physical addresses
3. This identity mapping persists after `exit_boot_services()`

This explains why the same code worked on x86_64 but failed on aarch64.

## Technical Solution: MMU Management

The solution required explicit management of the MMU before jumping to the kernel. Here's the implementation:

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // Architecture-specific preparation
    #[cfg(target_arch = "aarch64")]
    unsafe {
        // Ensure data is written to memory
        asm!("dsb sy");
        
        // Flush instruction cache
        asm!("ic iallu");       // Invalidate all instruction caches to PoU
        asm!("dsb ish");        // Ensure completion of cache operations
        asm!("isb");            // Synchronize context
        
        // Disable MMU by modifying SCTLR_EL1
        asm!(
            "mrs x0, sctlr_el1",    // Read current SCTLR_EL1
            "bic x0, x0, #1",       // Clear bit 0 (M) to disable MMU
            "msr sctlr_el1, x0",    // Write back to SCTLR_EL1
            "isb",                  // Instruction synchronization barrier
            out("x0") _
        );
    }
    
    // Define the function type for the kernel entry point
    type KernelEntry = extern "C" fn() -> !;
    
    // Convert the entry point address to a function pointer
    let kernel_main: KernelEntry = unsafe { 
        core::mem::transmute(entry_point) 
    };
    
    // Jump to kernel entry point
    kernel_main();
    
    // Should never reach here
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

This code:

1. Ensures all pending memory operations are completed with `dsb sy`
2. Invalidates instruction caches with `ic iallu`
3. Ensures cache operations are complete with `dsb ish`
4. Synchronizes the instruction stream with `isb`
5. Reads the current value of SCTLR_EL1 (System Control Register)
6. Clears bit 0 (M) to disable the MMU
7. Writes the modified value back to SCTLR_EL1
8. Synchronizes again with `isb`
9. Jumps to the kernel entry point

### Understanding SCTLR_EL1

The System Control Register (SCTLR_EL1) controls fundamental system behaviors, including the MMU. Here's a breakdown of its key bits:

```
Bit 0 (M): MMU enable
  0 = MMU disabled
  1 = MMU enabled

Bit 2 (C): Data cache enable
  0 = Data cache disabled
  1 = Data cache enabled

Bit 12 (I): Instruction cache enable
  0 = Instruction cache disabled
  1 = Instruction cache enabled
```

By clearing bit 0, we disable the MMU while leaving other features intact. This ensures that virtual addresses are treated as physical addresses, effectively creating an identity mapping for all memory.

### Synchronization Barriers

The ARM architecture provides several synchronization barriers that are crucial for MMU operations:

```
DSB (Data Synchronization Barrier)
  - Ensures all memory accesses are completed before proceeding
  - "dsb sy" affects all memory operations

ISB (Instruction Synchronization Barrier)
  - Flushes the pipeline and ensures all previous instructions are completed
  - Essential after changing system registers

IC IALLU (Instruction Cache Invalidate All to Point of Unification)
  - Invalidates all instruction caches
  - Necessary when code might be cached with old translations
```

These barriers ensure that the MMU state change is fully applied before jumping to the kernel.

## Kernel-Side Considerations

With the MMU disabled, the kernel needs to be aware of this state. Here's how the kernel entry point can be modified to handle this situation:

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // Disable interrupts
    unsafe {
        asm!("msr daifset, #2");
    }
    
    // Set up kernel page tables
    let page_tables = setup_kernel_page_tables();
    
    // Enable MMU with kernel page tables
    unsafe {
        // Point to our page tables
        asm!("msr ttbr0_el1, {}", in(reg) page_tables.ttbr0_el1);
        asm!("msr ttbr1_el1, {}", in(reg) page_tables.ttbr1_el1);
        
        // Configure translation control
        asm!("msr tcr_el1, {}", in(reg) page_tables.tcr_el1);
        
        // Ensure changes are visible
        asm!("isb");
        
        // Enable MMU by setting bit 0 in SCTLR_EL1
        asm!(
            "mrs x0, sctlr_el1",
            "orr x0, x0, #1",      // Set bit 0 (M) to enable MMU
            "msr sctlr_el1, x0",
            "isb",
            out("x0") _
        );
    }
    
    // Continue with kernel initialization
    init_platform();
    
    // Enter main kernel loop
    kernel_main_loop();
}
```

This approach gives the kernel complete control over the MMU configuration, allowing it to set up its own virtual memory mapping.

## Advanced Implementation: Preserving MMU State

An alternative approach is to preserve the MMU state but ensure proper mapping for the kernel:

```rust
pub fn prepare_kernel_execution(&self, entry_point: usize) -> ! {
    // Get current MMU configuration
    let mut sctlr_el1: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);
    }
    
    // Check if MMU is enabled
    let mmu_enabled = (sctlr_el1 & 1) != 0;
    
    if mmu_enabled {
        // Option 1: Disable MMU
        unsafe {
            asm!(
                "mrs x0, sctlr_el1",
                "bic x0, x0, #1",
                "msr sctlr_el1, x0",
                "isb",
                out("x0") _
            );
        }
        
        // Option 2: Add identity mapping for kernel
        // This would require manipulating page tables
        // unsafe {
        //     add_identity_mapping_for_kernel(entry_point);
        // }
    }
    
    // Jump to kernel
    type KernelEntry = extern "C" fn() -> !;
    let kernel_main: KernelEntry = unsafe { core::mem::transmute(entry_point) };
    kernel_main();
    
    // Should never reach here
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

## Testing the Solution

To verify the solution, I implemented a simple test kernel that writes to a known memory location:

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // Write signature to memory
    unsafe {
        core::ptr::write_volatile(0x40020000 as *mut u32, 0xCAFEBABE);
    }
    
    // Enter wait loop
    loop {
        unsafe { asm!("wfe"); }
    }
}
```

Using QEMU's monitor, I could verify that:
1. The kernel was executing correctly
2. The memory write was successful
3. The system remained stable

## Architecture-Specific Considerations

This investigation highlights important architectural differences between x86_64 and aarch64:

### aarch64 Specifics

1. **MMU Behavior**: The MMU is enabled during UEFI boot and remains enabled after `exit_boot_services()`
2. **Memory Mapping**: Only UEFI-managed memory regions are guaranteed to be identity-mapped
3. **Cache Coherency**: Explicit cache management is often required when changing MMU state

### x86_64 Specifics

1. **Paging Structure**: Uses a multi-level paging structure that's typically identity-mapped in UEFI
2. **Legacy Compatibility**: Maintains compatibility with older modes, affecting memory management
3. **Cache Behavior**: Cache invalidation is often handled automatically during page table changes

## Broader Implications and Best Practices

This experience yields several important lessons for cross-architecture OS development:

### 1. Architecture-Aware Design

When developing for multiple architectures, be aware of fundamental differences:

```rust
// Example of architecture-aware code
pub fn initialize_platform() {
    #[cfg(target_arch = "aarch64")]
    {
        // aarch64-specific initialization
        init_mmu_aarch64();
        setup_exception_vectors();
    }
    
    #[cfg(target_arch = "x86_64")]
    {
        // x86_64-specific initialization
        init_gdt();
        init_idt();
    }
    
    // Common initialization
    init_memory_allocator();
}
```

### 2. MMU Management Strategies

Different approaches to MMU management have trade-offs:

| Strategy | Advantages | Disadvantages |
|----------|------------|--------------|
| Disable MMU | Simple, reliable | Loses benefits of virtual memory |
| Identity map all memory | Preserves addressing | Requires page table manipulation |
| Custom mapping | Full control | Most complex to implement |

### 3. Debugging Techniques for Bare Metal

Develop a toolkit of debugging techniques that work without OS support:

```rust
// Debug utilities for bare metal development
pub mod debug {
    // Write to a known memory location
    pub fn write_debug_signature(signature: u32) {
        unsafe {
            core::ptr::write_volatile(0xDEAD0000 as *mut u32, signature);
        }
    }
    
    // Create a distinctive instruction pattern
    pub fn binary_breakpoint(id: u8) {
        unsafe {
            // Create unique pattern with ID
            asm!("mov x0, #{}", in(const) id);
            asm!("nop; nop; nop; nop");
        }
    }
    
    // Halt execution in a detectable way
    pub fn halt_with_code(code: u32) -> ! {
        write_debug_signature(code);
        loop {
            unsafe { asm!("wfe"); }
        }
    }
}
```

### 4. Memory Barrier Usage

Proper use of memory barriers is essential for reliable operation:

```rust
// Memory barrier utilities
pub mod barriers {
    // Ensure all memory writes are visible
    pub fn data_synchronization_barrier() {
        unsafe { asm!("dsb sy"); }
    }
    
    // Flush instruction pipeline
    pub fn instruction_synchronization_barrier() {
        unsafe { asm!("isb"); }
    }
    
    // Ensure memory operations complete in order
    pub fn data_memory_barrier() {
        unsafe { asm!("dmb sy"); }
    }
    
    // Full system synchronization
    pub fn full_system_barrier() {
        data_synchronization_barrier();
        instruction_synchronization_barrier();
    }
}
```

## Advanced Topic: ELF Loading and Memory Layout

Understanding the ELF format and memory layout is crucial for proper kernel loading. Here's a deeper look at the ELF loading process:

```rust
pub fn load_elf(&self, elf_data: &[u8]) -> Result<LoadedElf, ElfError> {
    // Parse ELF header
    let elf_file = ElfFile::new(elf_data)?;
    let elf_header = elf_file.header;
    
    // Verify architecture compatibility
    if !self.is_compatible_architecture(&elf_header) {
        return Err(ElfError::IncompatibleArchitecture);
    }
    
    // Allocate memory for program segments
    let mut loaded_segments = Vec::new();
    
    for program_header in elf_file.program_headers {
        if program_header.p_type != PT_LOAD {
            continue;
        }
        
        // Calculate memory requirements
        let virt_addr = program_header.p_vaddr as usize;
        let mem_size = program_header.p_memsz as usize;
        let file_size = program_header.p_filesz as usize;
        
        // Allocate physical memory
        let phys_addr = self.allocate_physical_memory(mem_size, 0x1000)?;
        
        // Copy segment data
        let src_offset = program_header.p_offset as usize;
        let src_data = &elf_data[src_offset..src_offset + file_size];
        unsafe {
            core::ptr::copy_nonoverlapping(
                src_data.as_ptr(),
                phys_addr as *mut u8,
                file_size
            );
            
            // Zero remaining memory (bss section)
            if mem_size > file_size {
                core::ptr::write_bytes(
                    (phys_addr + file_size) as *mut u8,
                    0,
                    mem_size - file_size
                );
            }
        }
        
        // Record segment mapping
        loaded_segments.push(LoadedSegment {
            virt_addr,
            phys_addr,
            size: mem_size,
            flags: program_header.p_flags,
        });
    }
    
    Ok(LoadedElf {
        entry_point: elf_header.e_entry as usize,
        segments: loaded_segments,
    })
}
```

### Memory Layout Considerations

When loading an ELF file, consider these memory layout aspects:

1. **Virtual vs Physical Addressing**: ELF segments specify virtual addresses, but you need to load them into physical memory
2. **Alignment Requirements**: Memory regions often need specific alignment (typically page-aligned)
3. **Permissions**: Different segments have different permission requirements (read, write, execute)
4. **BSS Handling**: The BSS section needs to be zeroed but isn't stored in the ELF file

## Conclusion: Cross-Architecture Development Insights

Developing an OS that works across different architectures requires understanding the subtle differences in hardware behavior. The MMU configuration issue highlighted in this article is just one example of the architecture-specific challenges you might encounter.

Key takeaways:

1. **Architectural Differences Matter**: What works on one architecture may fail on another due to fundamental differences in hardware behavior.

2. **MMU Configuration is Critical**: The MMU setup significantly affects the transition from bootloader to kernel, especially on aarch64.

3. **Systematic Debugging is Essential**: When working at the bare metal level, methodical debugging techniques are your most valuable tools.

4. **Memory Barriers are Crucial**: Proper synchronization is essential when manipulating system state, especially the MMU.

5. **Documentation is Sparse**: For niche combinations like Rust on aarch64 UEFI, you often need to piece together information from multiple sources.

By sharing these insights, I hope to help other OS developers avoid similar pitfalls and better understand the intricacies of cross-architecture development.

## References and Further Reading

For those interested in diving deeper into these topics:

1. [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest) - Definitive guide to the ARM architecture
2. [UEFI Specification](https://uefi.org/specifications) - Official UEFI documentation
3. [Rust for Embedded Development](https://docs.rust-embedded.org/) - Resources for embedded Rust development
4. [OSDev Wiki](https://wiki.osdev.org/) - Community knowledge base for OS development
5. [ELF Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf) - Executable and Linkable Format documentation

## Appendix A: Complete MMU Management Implementation

Here's a complete implementation of MMU management for aarch64:

```rust
/// MMU configuration for aarch64
pub struct MmuConfig {
    pub ttbr0_el1: u64,  // Translation Table Base Register 0
    pub ttbr1_el1: u64,  // Translation Table Base Register 1
    pub tcr_el1: u64,    // Translation Control Register
    pub mair_el1: u64,   // Memory Attribute Indirection Register
}

/// Disable the MMU
pub unsafe fn disable_mmu() {
    // Ensure all memory operations complete
    asm!("dsb sy");
    
    // Read current SCTLR_EL1
    let mut sctlr_el1: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);
    
    // Clear bit 0 (M) to disable MMU
    sctlr_el1 &= !1;
    
    // Write back to SCTLR_EL1
    asm!("msr sctlr_el1, {}", in(reg) sctlr_el1);
    
    // Ensure changes are applied
    asm!("isb");
}

/// Enable the MMU with the given configuration
pub unsafe fn enable_mmu(config: &MmuConfig) {
    // Set up translation tables
    asm!("msr ttbr0_el1, {}", in(reg) config.ttbr0_el1);
    asm!("msr ttbr1_el1, {}", in(reg) config.ttbr1_el1);
    
    // Configure translation control
    asm!("msr tcr_el1, {}", in(reg) config.tcr_el1);
    
    // Set up memory attributes
    asm!("msr mair_el1, {}", in(reg) config.mair_el1);
    
    // Ensure changes are visible
    asm!("isb");
    
    // Read current SCTLR_EL1
    let mut sctlr_el1: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);
    
    // Set bit 0 (M) to enable MMU
    sctlr_el1 |= 1;
    
    // Write back to SCTLR_EL1
    asm!("msr sctlr_el1, {}", in(reg) sctlr_el1);
    
    // Ensure changes are applied
    asm!("isb");
}

/// Create identity mapping for a memory region
pub unsafe fn identity_map_region(
    page_tables: &mut PageTables,
    start_addr: usize,
    size: usize,
    attributes: PageAttributes
) -> Result<(), MapError> {
    let start_page = start_addr / PAGE_SIZE;
    let page_count = (size + PAGE_SIZE - 1) / PAGE_SIZE;
    
    for i in 0..page_count {
        let virt_addr = (start_page + i) * PAGE_SIZE;
        let phys_addr = virt_addr;  // Identity mapping
        
        page_tables.map_page(virt_addr, phys_addr, attributes)?;
    }
    
    Ok(())
}
```

## Appendix B: QEMU Debugging Commands

Here's a reference of useful QEMU monitor commands for debugging:

```
# Basic system information
info registers        # Display CPU registers
info cpus             # Show CPU states
info tlb              # Display TLB content
info mem              # Show memory mapping

# Memory examination
x/10i $pc            # Disassemble 10 instructions at program counter
x/20wx 0x40010000    # Examine 20 words at address 0x40010000
xp/20wx 0x40010000   # Examine 20 words at physical address 0x40010000

# Execution control
c                     # Continue execution
s                     # Step one instruction
p                     # Step through call/return

# Breakpoints
b 0x40010120          # Set breakpoint at address
watch 0x40020000      # Watch for memory changes
delete 1              # Delete breakpoint #1

# System control
system_reset          # Reset the system
quit                  # Exit QEMU
```

These commands can be invaluable when debugging low-level issues in your OS development journey.
