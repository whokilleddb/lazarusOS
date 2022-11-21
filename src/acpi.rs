#![allow(non_camel_case_types)]
#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

/// A very basic ACPI implentation for extracting basic information 
/// about CPU topography and NUMA regions

use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering, AtomicU8};
use core::convert::TryInto;

/// Maximum Number of cores allowed on the system
pub const MAX_CORES: usize = 1024;


/// In-memory representation of RSDP ACPI structure(v 1.0)
/// RSDP Strcut definiton -> https://wiki.osdev.org/RSDP
/// "packed" -> https://developer.arm.com/documentation/dui0491/i/Compiler-specific-Features/--attribute----packed---variable-attribute
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct RSDPDescriptor {
    Signature:      [u8; 8],
    Checksum:       u8,
    OEMID:          [u8; 6],
    Revision:       u8,
    RsdtAddress:    u32,
}


/// In-memory representation of Extended RSDP ACPI structure(v 2.0)
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct RSDPDescriptor20 {
    firstPart:          RSDPDescriptor,
    Length:             u32,
    XsdtAddress:        u64 ,
    ExtendedChecksum:   u8,
    reserved:           [u8; 3],
}

