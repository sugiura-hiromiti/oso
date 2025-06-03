Based on the QEMU execution log, I can see that the virtual machine is running but there might be an issue with your kernel execution. To diagnose what's preventing
your kernel from executing properly, I would need the following additional information:

1. Memory map information:
   • Run info mtree in QEMU to see the memory layout
   • Run info tlb to check the translation lookaside buffer

2. Disassembly of the current execution point:
   • The PC (Program Counter) is at 0x470f5174, which doesn't match your kernel's entry point (0x40010120)
   • Run x/10i 0x470f5174 to disassemble the current execution point

3. Kernel loading verification:
   • Check if your kernel was properly loaded into memory with xp /10xg 0x40010120
   • This will show if your entry point contains the expected instructions

4. Boot process logs:
   • UEFI boot logs showing the handoff to your kernel
   • Any error messages during the boot process

5. Current execution state:
   • Run info qtree to see the QEMU device tree
   • Run info trace-events to check if there are any relevant events

6. Kernel source code:
   • The entry point implementation in your kernel
   • How you're handling the transition from UEFI to your kernel

The current state shows the system is running in EL1 (Exception Level 1), but not at your kernel's entry point address. This suggests either:

1. The kernel wasn't properly loaded
2. The boot process didn't correctly jump to your entry point
3. There was an exception/crash that diverted execution elsewhere

Could you provide these additional details to help diagnose the issue?
