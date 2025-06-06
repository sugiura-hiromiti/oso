# Troubleshooting aarch64 Kernel Execution in UEFI Environment: The MMU Mystery

## Introduction

As a hobbyist OS developer, I've been working on a project called "oso" - a full-scratch, Pure Rust, aarch64-targeted operating system. One of the challenges with such niche projects is that when you encounter errors, solutions are often hard to find. There's not a lot of documentation or community knowledge specifically for this combination of technologies.

In this article, I'll share a particularly frustrating issue I encountered recently and how I solved it. If you're working on similar projects, especially if you're:

- Building a bootloader from scratch
- Developing for aarch64 architecture
- Using QEMU for development
- Writing an OS in Rust

...then this article might save you hours of debugging.

## The Problem: Kernel Not Executing

The issue I faced was deceptively simple: my kernel wouldn't execute properly on aarch64 hardware, despite working fine on x86_64. The symptoms were:

1. The bootloader would successfully load the kernel ELF file
2. The bootloader would attempt to jump to the kernel entry point
3. But the kernel code would never execute properly

What made this particularly confusing was that the exact same code worked perfectly on x86_64 architecture. This suggested that there was something architecture-specific causing the problem.

## Project Structure and Implementation

Before diving into the troubleshooting process, let me briefly explain the structure of my "oso" project:

- **oso_loader**: The UEFI bootloader written in Rust
- **oso_kernel**: The kernel itself, also written in Rust
- **oso_bridge**: Shared code between the loader and kernel
- **xtask**: Build scripts and utilities

The bootloader's job is to:

1. Initialize the UEFI environment
2. Load the kernel ELF file from the ESP (EFI System Partition)
3. Parse the ELF file to find the entry point
4. Exit UEFI boot services
5. Jump to the kernel entry point

The kernel entry point is defined in `oso_kernel/src/main.rs`:

```rust
#[unsafe(no_mangle)]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() {
    // Disable IRQ(interrupt request)
    unsafe {
        asm!("msr daifset, #2");
    }

    // Wait for interrupt (infinite loop for debugging)
    wfi();
}
```

## Investigating Possible Causes

When faced with this kind of issue, it's important to systematically rule out potential causes. Here are the possibilities I considered:

### 1. Exit Boot Services Failure

My first suspicion was that `exit_boot_services()` might be failing. This UEFI call is crucial because it transitions control from the UEFI firmware to your OS. If it fails, the firmware remains in control.

My implementation of `exit_boot_services()` looked like this:

```rust
pub fn exit_boot_services(&self, custom_memory_type: Option<MemoryType>) {
    let memory_type = custom_memory_type.unwrap_or(MemoryType::LOADER_DATA);

    for _ in 0..2 {
        let mut buf = MemoryMapBackingMemory::new(memory_type).expect("failed to allocate memory");
        let status = unsafe { self.try_exit_boot_services(buf.as_mut_slice()) };

        match status.is_success() {
            true => {
                return;
            },
            false => {
                // Try again if failed
            }
        }
    }

    // If we reach here, we failed to exit boot services
    todo!("failed to exit boot service. reset the machine");
}
```

I carefully reviewed this code and confirmed that it was working correctly. My bootloader was designed to panic on any error before exiting boot services, and I wasn't seeing any panics. This suggested that exit_boot_services was succeeding.

### 2. Incorrect Kernel Entry Address

Next, I wondered if the ELF parser might be extracting the wrong entry point address. I had implemented a custom ELF parser in my bootloader to extract the entry point from the kernel ELF file:

```rust
pub fn entry_point_address(&self) -> usize {
    self.header.entry as usize
}
```

To verify this was working correctly, I:

- Dumped the ELF header using `readelf -h target/oso_kernel.elf`
- Confirmed the entry point address (0x40010120) matched what my bootloader was using
- Used QEMU's monitor to disassemble the memory at that address

The output from `readelf -h` showed:

```
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
  ...
```

And disassembling the memory at that address showed my kernel code was indeed loaded correctly:

```
0x40010120:  d10043ff  sub      sp, sp, #0x10
0x40010124:  14000001  b        #0x40010128
0x40010128:  d503205f  wfe
0x4001012c:  17ffffff  b        #0x40010128
```

This confirmed that the entry point address was correct, and the memory at that address contained my kernel code.

### 3. Exception Handling

Another possibility was that an exception was occurring immediately after jumping to the kernel. On aarch64, if you don't set up exception handlers, an exception will cause the system to halt.

To test this theory, I added simple exception handling code to my kernel, but the problem persisted. I also tried disabling interrupts immediately in the kernel entry point:

```rust
unsafe {
    asm!("msr daifset, #2");  // Disable interrupts
}
```

But this didn't solve the issue either.

### 4. Boot Service Usage After Exit

Using UEFI Boot Services after calling `exit_boot_services()` would cause undefined behavior. I carefully reviewed my code to ensure I wasn't making any such calls, and found no issues.

## The Breakthrough: Memory Mapping Differences

After exhausting the obvious possibilities, I started looking at more subtle architectural differences between x86_64 and aarch64. This led me to investigate the Memory Management Unit (MMU) behavior.

I discovered that UEFI firmware enables the MMU during boot, and the configuration differs between architectures:

- On x86_64, UEFI typically sets up identity mapping for the entire address space
- On aarch64, UEFI only guarantees identity mapping for memory regions it manages

This meant that when my bootloader jumped to the kernel entry point on aarch64, the virtual address might not map to the correct physical address if it was outside UEFI-managed memory.

### Debug Techniques That Helped

Before finding the solution, I employed several debugging techniques that proved invaluable:

#### Print Debugging

While traditional print debugging is limited after exiting boot services, I was able to use it before that point to verify the bootloader's behavior.

#### Mnemonic Debugging (Binary Breakpoints)

I inserted specific instruction sequences that I could identify in memory dumps. For example:

```rust
// In my kernel entry point
unsafe {
    asm!("wfe"); // Wait For Event - a distinctive instruction
}
```

Then in QEMU's monitor console (accessed with Ctrl+Alt+2), I could:

- Use `info registers` to see the current execution point
- Use `x /10i 0xXXXXXXX` to disassemble memory at specific addresses

This allowed me to confirm that execution was not reaching my kernel code.

#### Strategic Use of WFI and WFE

I used different wait instructions for different purposes:

- `wfi` (Wait For Interrupt) in my panic handler
- `wfe` (Wait For Event) as binary breakpoints

This made it easier to identify where execution was stopping. In my `oso_bridge` crate, I defined these helper functions:

```rust
#[inline(always)]
pub fn wfi() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "aarch64")]
            asm!("wfi");
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
            asm!("wfe");
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}
```

## The Root Cause: UEFI MMU Configuration

The breakthrough came when I realized that the issue was related to the MMU configuration. When UEFI firmware boots on aarch64, it enables the MMU with a specific virtual memory mapping. This mapping ensures that memory regions managed by UEFI are identity-mapped (virtual address = physical address), but it doesn't guarantee this for all memory regions.

In contrast, on x86_64, UEFI typically sets up identity mapping for the entire address space, which is why my code worked fine on that architecture.

When my bootloader jumped to the kernel entry point on aarch64, the virtual address (0x40010120) might not have been correctly mapped to the corresponding physical address, causing the jump to fail or execute incorrect code.

## The Solution: Disable MMU Before Jumping to Kernel

The fix was surprisingly simple once I understood the problem. I needed to disable the MMU before jumping to the kernel entry point:

```rust
#[cfg(target_arch = "aarch64")]
unsafe {
    // Ensure data is written to memory
    asm!("dsb sy");

    // Flush caches
    asm!("ic iallu"); // Invalidate all instruction caches to PoU
    asm!("dsb ish"); // Ensure completion of cache operations
    asm!("isb"); // Synchronize context

    // Disable MMU by modifying SCTLR_EL1
    asm!(
        "mrs x0, sctlr_el1",          // Read current SCTLR_EL1
        "bic x0, x0, #1",             // Clear bit 0 (M) to disable MMU
        "msr sctlr_el1, x0",          // Write back to SCTLR_EL1
        "isb",                         // Instruction synchronization barrier
        out("x0") _
    );
}

// Jump to kernel with MMU disabled
entry_point();

unsafe {
    // Fallback loop if jump fails
    loop {
        asm!("wfi");
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
10. Provides a fallback loop in case the jump fails

After implementing this change, my kernel started executing correctly on aarch64!

## Key Takeaways

This experience taught me several important lessons about OS development for different architectures:

1. **Architecture-Specific Behavior**: Even with standards like UEFI, there are significant differences in how architectures behave. What works on x86_64 might not work on aarch64.

2. **MMU Configuration Matters**: The MMU setup is critical when transitioning from bootloader to kernel. Understanding the virtual memory mapping is essential.

3. **UEFI Memory Guarantees**: UEFI only guarantees identity mapping for memory it manages, not for all memory. This is a subtle but crucial detail.

4. **Debugging Techniques**: Having multiple debugging approaches is essential when working at this low level. Print debugging, binary breakpoints, and QEMU's monitor are all valuable tools.

5. **Binary Analysis**: Tools like `readelf` and QEMU's monitor are invaluable for understanding what's happening at the machine code level.

## Conclusion

If you're developing an OS for aarch64 and experiencing issues with kernel execution, check your MMU configuration. The solution might be as simple as disabling the MMU before jumping to your kernel.

This kind of architecture-specific behavior is exactly why OS development is both challenging and fascinating. Each problem solved deepens your understanding of how computers really work at the lowest levels.

I hope this article helps others who might be facing similar issues. Remember, in the world of OS development, you're often exploring uncharted territory - persistence and systematic debugging are your best tools.

Happy hacking!
