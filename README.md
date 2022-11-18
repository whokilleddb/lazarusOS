<h1 align=center>LazarusOS - A Minimal Operating System Written In Rust</h1>
<p align=center>
   <img src="https://shields.io/badge/Made_With-Rust-green" />
</p>

## Abstract
LazarusOS is an operating system written from scratch in Rust featuring a UEFI bootloader with active Kernel development. The OS derives the memory safety nature of Rust [1]. The performance specs of Rust are comparable to languages like C and C++. The current development targets x86_64 CPUs with plans for targetting ARMv7 chipsets soon.

## Introduction
This project was directly inspired by the Rust for Linux project[2] which aims at integrating Rust components into the Linux Kernel. The project also derives inspiration from projects like Redox-OS[3] and BlogOS[4]. 
Rust is comparable to low-level languages like C, and C++ when it comes to performance while ensuring safety, speed, and concurrency. These performance benefits are a by-product of the strict compile-time checks.  It is best known for its memory safety, but it also allows full access to raw pointers using the `unsafe` keyword. However, this does not exempt the program from the Rust Borrow Checker [5] or disable any other of Rust’s safety checks. This allows Rust programs to retain memory safety features that help in preventing a lot of vulnerabilities like Buffer Overflows, Dangling Pointers, and Iterator Invalidation. 

Rust has some more salient features that make it an ideal choice for LazarusOS. One of the primary advantages of Rust is that the Rust build system using `rustc` and `cargo` allows for easy cross compilations. The Rust standard library(std) has two components to it:

	- `core`: It includes the primitive components of the standard library which provides mostly platform-independent components. However, it does not provide resources for allocation, threading, and other high-level features.

	- `alloc`: It provides resources for managing heap allocations, smart pointers, data collection, etc.
	
You can make a free-standing Rust binary without including the `std` library by adding the `no_std` annotation to the main source file. We can then build either of the above libraries at runtime. For example, to build the `core` library as a part of the crate graph, we can just add the following configuration option to the `.cargo/config.toml` file:

```toml
[unstable]
build-std = ["core"]
```

This allows for the easy generation of free-standing binaries without much hassle. Also, most of these features are only available on nightly channels. However, the Rust build environment easily allows us to switch channels that makes the process very programmer-friendly.
 
## The Bootloader
LazarusOS features an UEFI bootloader instead of Legacy BIOS. This is so that once the development is complete, it can run on most real hardware. Since the platform initilization procedure of UEFI is standardized, it allows for vendor neutral extension of UEFI firmware.

The implemented bootloader closely follows the EDK2 decalrations, directly deriving from structures and functions as defined in the edk2 sources. This helps in the standardiazation of the bootloader. To ensure compatibility with UEFI firmware, we ensure that the various structs used are interoperable with the C Language using the `repr(C)` attribute. 

As of present, the `efi_main()` function calls a function `register_system_table()` to register the global System Table Pointer and then panics. It is to be noted that the panic handler is also written from scratch. The `panic` function prints some information regarding the source of the panic and a custom message if provided. Finally it halts the CPU by issuing the `hlt` instruction.

## Compilation and Emulation
Compiling `lazarusOS` is very simple using `cargo`.  The configuration file located in `.cargo/config.toml`, specifies the compile time options as such:

```toml
[build]
target = "x86_64-unknown-uefi" 

[unstable]
build-std = ["core"]

[target.x86_64-unknown-uefi]
rustflags = ["-C", "link-args=/debug:dwarf"] 
```
Here, we compile our binary for x86_64 systems with a UEFI compatible target system. We also implicitly compile the `core` library and add darwin type debug symbols to the resultant binary. Following this, building the binary is as simple as:
```bash
$ cargo build
```
Or, for a release build
```bash
$ cargo build --release
```

Emulation is done via `QEMU`  with full `KVM` virtualization support with 128 MiB of RAM.  We also use the `OVMF_CODE.fd` BIOS file from the `EDK2` suite. The full emulation command is:

```bash
qemu-system-x86_64  \
    -enable-kvm \
    -m 128 \
    -nographic \
    -bios /usr/share/edk2-ovmf/x64/OVMF_CODE.fd \
    -device driver=e1000,netdev=n0 \
    -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=lazarus.efi
```


## Future Prospects
The primary goal as of present is to write a minimal Kernel on Rust for x86_64 systems. Once the said goal is achieved, the resources can be resused, after making the necessary modifications, to compile a Operating System capable of running on ARMv7 chipsets.

Further inspiration can be drawn from CoreOS RISCV[6] project to extend the usage to `RISC V` CPUs at a later stage.

## References
	1 - Memory Safety in Rust https://stanford-cs242.github.io/f18/lectures/05-1-rust-memory-safety.html
	2 - https://github.com/Rust-for-Linux
	3 - https://www.redox-os.org/
	4 - https://os.phil-opp.com/
	5 - https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
	6 - https://github.com/skyzh/core-os-riscv
