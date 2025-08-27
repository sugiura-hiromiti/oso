# OSO No-Std Shared Library

A foundational `no_std` Rust library providing shared utilities and data structures for the OSO operating system. This crate serves as a common foundation for system-level programming in bare-metal environments.

## Features

- **🔧 Hardware Abstraction**: Low-level CPU control and hardware interface utilities
- **🌳 Data Structures**: Generic tree structures optimized for system programming
- **📝 Parser Framework**: Extensible parsing utilities for binary data and markup
- **⚡ Zero-Cost Abstractions**: Compile-time optimizations with no runtime overhead
- **🚫 No Standard Library**: Designed for bare-metal and embedded environments

## Architecture

### Modules

#### `bridge` - Hardware Interface Layer
- **Graphics**: Framebuffer configuration and pixel format management
- **Device Tree**: Hardware description parsing utilities
- **CPU Control**: Platform-specific power management functions

#### `data` - Data Structures
- **Tree**: Generic tree data structure with comprehensive traversal capabilities
- Lifetime-managed references for memory safety without heap allocation
- Trait-based design for flexible operations

#### `parser` - Parsing Framework
- **Binary Parser**: Specialized binary data parsing utilities
- **Generator**: Core parser generation framework and traits
- **HTML Parser**: Placeholder for future HTML parsing capabilities

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
oso_no_std_shared = { path = "path/to/oso_no_std_shared" }
```

### Basic Example

```rust
#![no_std]

use oso_no_std_shared::bridge::graphic::{FrameBufConf, PixelFormatConf};
use oso_no_std_shared::wfi;

fn main() -> ! {
    // Configure a framebuffer for graphics output
    let framebuf = FrameBufConf::new(
        PixelFormatConf::Rgb,
        0x1000_0000 as *mut u8, // Base address
        1024 * 768 * 4,         // Size in bytes
        1024,                   // Width in pixels
        768,                    // Height in pixels
        1024 * 4,               // Stride (bytes per row)
    );

    // Perform graphics operations...
    
    // Enter low-power state until interrupt
    wfi(); // Never returns
}
```

### Tree Data Structure Example

```rust
use oso_no_std_shared::data::tree::{Tree, Node, TreeWalk};

// Create a tree node
let root_node = Node("root".to_string());
let children = [];
let tree = Tree {
    value: root_node,
    children: &children,
    parent: None,
};

// Navigate the tree (when TreeWalk is implemented)
// let walker = tree.walker();
// walker.first_child();
```

## Platform Support

The library supports multiple architectures with platform-specific optimizations:

- **AArch64 (ARM64)**: Native `wfi`, `wfe`, and `nop` instructions
- **x86_64**: `hlt` instruction for power management

## Requirements

- Rust 2024 edition
- `no_std` environment
- Unstable features:
  - `unboxed_closures`
  - `associated_type_defaults`
  - `impl_trait_in_assoc_type`
  - `const_trait_impl`
  - `type_alias_impl_trait`

## Development Status

This library is under active development as part of the OSO operating system project. Some modules are complete while others serve as architectural foundations for future development.

### Completed
- ✅ Bridge module with graphics and device tree support
- ✅ CPU control functions with platform-specific implementations
- ✅ Tree data structure framework and traits

### In Progress
- 🔄 Tree traversal algorithm implementations
- 🔄 Binary parser concrete implementations
- 🔄 Parser component composition system

### Planned
- 📋 HTML parser implementation
- 📋 Additional data structures (graphs, heaps)
- 📋 Memory management utilities
- 📋 Interrupt handling abstractions

## Contributing

This library is part of the OSO operating system project. Contributions should focus on:

1. **Performance**: Zero-cost abstractions and compile-time optimizations
2. **Safety**: Memory safety without heap allocation
3. **Portability**: Support for multiple target architectures
4. **Documentation**: Comprehensive docs for system-level programming

## License

[License information to be added]

## Related Projects

- `oso_error`: Error handling utilities for the OSO ecosystem
- OSO Operating System: The main operating system project

---

**Note**: This library requires unstable Rust features and is designed specifically for system-level programming in `no_std` environments.
