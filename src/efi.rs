//! This file contains UEFI EFI structures
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::efi::{EfiHandle, EfiSystemTable, EfiStatus};


/// Struct to store EFI_HANDLE
/// Definition is analogous to the C definition as seen in:
/// https://dox.ipxe.org/include_2ipxe_2efi_2efi_8h_source.html#l00050
#[derive(Clone, Copy, Debug)]
#[repr(C)]  // See: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
pub struct EfiHandle(usize);


/// Struct to store UEFI status code
/// For definition, see: https://developer.apple.com/documentation/kernel/efi_status
/// See(Page 23): https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiStatus(pub usize);


/// A scan code and unicode value for an input key press
/// See: https://dox.ipxe.org/structEFI__INPUT__KEY.html
/// See: https://docs.rs/uefi-ffi/latest/uefi_ffi/struct.EFI_INPUT_KEY.html
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiInputKey {
    // Scan code for key press
    scan_code: u16,

    // Unicode Representation of the key
    unicode_char : u16,
}


/// EFI memory types
/// See: https://youtu.be/VW6WIe3aY_Q?t=577
/// See: https://docs.rs/redox_uefi/latest/uefi/memory/enum.MemoryType.html
/// See: https://uefi.org/specs/ACPI/6.4/15_System_Address_Map_Interfaces/uefi-getmemorymap-boot-services-function.html
/// 
/// Boot Services vs Runtime Services
/// See: https://www.reddit.com/r/osdev/comments/gougq6/uefi_boot_services_vs_runtime_services/
/// See: https://forum.osdev.org/viewtopic.php?f=1&t=40937
pub enum EfiMemoryType {
    EfiReservedMemoryType,      // Not Used
    EfiLoaderCode,              // The code portions of a loaded application. (Note that UEFI OS loaders are UEFI applications.)
    EfiLoaderData,              // The data portions of a loaded application and the default data allocation type used by an application to allocate pool memory.
    EfiBootServicesCode,        // The code portions of a loaded Boot Services Driver
    EfiBootServicesData,        // The data portions of a loaded Boot Serves Driver, and the default data allocation type used by a Boot Services Driver to allocate pool memory.
    EfiRuntimeServiceCode,      // The code portions of a loaded Runtime Services Driver.
    EfiRuntimeServicesData,     // The data portions of a loaded Runtime Services Driver and the default data allocation type used by a Runtime Services Driver to allocate pool memory.
    EfiConventionalMemory,      // Free (unallocated) memory.
    EfiUnusableMemory,          // Memory in which errors have been detected.
    EfiACPIReclaimMemory,       // Memory that holds the ACPI tables.
    EfiACPIMemoryNVS,           // Address space reserved for use by the firmware.
    EfiMemoryMappedIO,          // Used by system firmware to request that a memory-mapped IO region be mapped by the OS to a virtual address so it can be accessed by EFI runtime services.
    EfiMemoryMappedIOPortSpace, // System memory-mapped IO region that is used to translate memory cycles to IO cycles by the processor.
    EfiPalCode,                 // Address space reserved by the firmware for code that is part of the processor.
    EfiPersistentMemory,        // A memory region that operates as EfiConventionalMemory, however it happens to also support byte-addressable non-volatility.
    Reserved,
}

/// Corresponding numeric codes to each of UEFI Memory types
impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        match val {
            0 => EfiMempryType::EfiReservedMemoryType,
            1 => EfiMempryType::EfiLoaderCode,
            2 => EfiMempryType::EfiLoaderData,
            3 => EfiMempryType::EfiBootServicesCode,
            4 => EfiMempryType::EfiBootServicesData,
            5 => EfiMempryType::EfiRuntimeServiceCode,
            6 => EfiMempryType::EfiRuntimeServicesData,
            7 => EfiMempryType::EfiConventionalMemory,
            8 => EfiMempryType::EfiUnusableMemory,
            9 => EfiMempryType::EfiACPIReclaimMemory,
           10 => EfiMempryType::EfiACPIMemoryNVS,
           11 => EfiMempryType::EfiMemoryMappedIO,
           12 => EfiMempryType::EfiMemoryMappedIOPortSpace,
           13 => EfiMempryType::EfiPalCode,
           14 => EfiMempryType::EfiPersistentMemory,
           _ => EfiMempryType::Reserved,
        }
    }
}

impl EfiMemoryType {
    // Returns whether or not this memory is available for general purpose use after the boot services have been exited

    // From Wikipedia: https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface#Services
    // EFI defines two types of services: `boot` services and `runtime` services.
    // Boot services are available only while the firmware owns the platform (i.e., before the `ExitBootServices()` call),
    // and they include text and graphical consoles on various devices, and bus, block and file services.
    // Runtime services are still accessible while the operating system is running;
    // they include services such as date, time and NVRAM access.`

    fn avail_post_exit_boot_services(&self) -> bool {
        match self{
            EfiMemoryType::EfiBootServicesCode |
            EfiMemoryType::EfiBootServicesData |
            EfiMemoryType::EfiConventionalMemory |
            EfiMemoryType::EfiPersistentMemory => true,

            _ => false
        }
    }
}



/// Memory descriptor to store the return type from `GetMemoryMap()`
/// See: https://dox.ipxe.org/structEFI__MEMORY__DESCRIPTOR.html
/// See: https://github.com/tianocore/edk2/blob/91a03f78ba0b75bc4ed2c4b756cbe57c685d9c72/MdePkg/Include/Uefi/UefiSpec.h#L127
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
struct EfiMemoryDescriptor {
    // Type of the memory region.
    typ: u32;

    // Physical address of the first byte in the memory region
    // It must be aligned to a 4KiB boundary and must not be above 
    // 0xfffffffffffff000. Why the funny hex address? 
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // Why 4KiB?
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/comment/i50kny8/?utm_source=share&utm_medium=web2x&context=3
    physical_start: u64; // 64 bit address

    // Virtual address of the first byte in the memory region
    // It must be aligned to a 4KiB boundary and must not be above 
    // 0xfffffffffffff000. Why the funny hex address? 
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // Why 4KiB?
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/comment/i50kny8/?utm_source=share&utm_medium=web2x&context=3
    virtual_start: u64; // 64 bit address

    // Number of 4KiB pages in the memory region. Number of pages cannot
    // Number of Pages must not be 0, and must not be any value
    // that would represent a memory page with a start address,
    // either physical or virtual, above 0xfffffffffffff000.
    number_of_pages: u64;

    // Attributes of the memory region that describe the bit mask of capabilities
    // for that memory region, and not necessarily the current settings for that
    // memory region.
    attribute: u64,
}



/// Pointer to the EFI System Table which is saved upon the entry of the kernel
/// This pointer is needed for Console I/O
/// This needs to be global because `print()` functions don't get a `&self` pointer
/// Declaring it global is the only way we can get access to the system table in a print macro
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());


/// Read More about UEFI System Table: https://edk2-docs.gitbook.io/edk-ii-uefi-driver-writer-s-guide/3_foundation/33_uefi_system_table
/// EFI System Table: https://dox.ipxe.org/structEFI__SYSTEM__TABLE.html
/// For Detailed Reading, See Chapter 4(Page: 93): https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf


/// Register a system table pointer.
/// Only the first non-null system table pointer will be stored in the `EFI_SYSTEM_TABLE` global
pub unsafe fn register_system_table(system_table: *mut EfiSystemTable){
    // See: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.compare_exchange
    EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        EfiSystemTable,
        Ordering::SeqCst,    // See: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
        Ordering::SeqCst
    );
}


/// Write a `string` to UEFI output
pub fun output_string(string: &str){
    // Get the system table
    let system_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // Check if pointer is null
    if system_table.is_null(){return ;}

    // Get the console output_pointer
    let console_std_out = unsafe {
        (*system_table).console_out
    };

    // Create a temporary buffer capable of holding 31 characters and a null
    // UEFI uses UCS-2 encoding instead of UTF-16
    let mut tmp = [0u16; 32];
    let mut in_use = 0;

    // Loop through all characters
    for chr in string.encode_utf16(){
        // Add CRLF
        // CRLFs are required by serial consoles at times instead
        if chr == b'\n' as u16{
            tmp[in_use] = b'\r' as u16;
            in_use += 1;
        }

        // Write character into buffer
        tmp[in_use] = chr;
        in_use += 1;

        // Note the -2 instead of the usual -1
        // This is because of `\r\n`
        if in_use == (tmp.len()-2){
            // Null Terminate the buffer
            tmp[in_use] = 0;

            // Write output to buffer
            // See: https://github.com/rust-osdev/uefi-rs/blob/dfca11c419a6b2d943ef02af4c7d6c7e3732a195/src/proto/console/text/output.rs#L46
            unsafe {
                ((*console_std_out)
                    .output_string)(out, tmp.as_ptr());
            }

            // Clear the buffer
            in_use = 0;
        }
    }

    // Write out any remaining characters
    if in_use > 0 {
        // Null terminate the buffer
        tmp[in_use] = 0;

        unsafe {
            ((*console_std_out)
                .output_string)(out, tmp.as_ptr());
        }
    }
}


/// Get memory map for the System from UEFI
/// See: https://wiki.osdev.org/Detecting_Memory_(x86)
pub fn get_memory_map(){
    // Get the system table
    let system_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // Check null
    if system_table.is_null() {return;}

    // Create an empty memory map
    let mut memory_map = [0u8; 2*1024];

    let mut free_memory - 0u64;

    // See: https://www.youtube.com/watch?v=VW6WIe3aY_Q
    unsafe{
        let mut map_size = core::mem::size_of_val(&memory_map);
        let mut map_key;
        let mut map_descriptor_size = 0;
        let mut map_descriptor_version = 0;

        // GetMemoryMap() Call
        // See: https://uefi.org/specs/ACPI/6.4/15_System_Address_Map_Interfaces/uefi-getmemorymap-boot-services-function.html
        let ret = ((*(*system_table).boot_services).get_memory_map)(
            &mut map_size,
            memory_map.as_mut_ptr(),
            &mut map_key,
            &mut map_descriptor_size,
            &mut map_descriptor_version
        );

        // Check if Descriptor Table is empty
        assert!(ret.0 == 0, "{:x?}", ret);

        for off in (0..size).step_by(map_descriptor_size) {
            let entry = core::ptr::read_unaligned(
                memory_map[off..].as_ptr() as *const EfiMemoryDescriptor
            );

            let typ: EfiMemoryType = entry.typ.into();

            if typ.avail_post_exit_boot_services(){
                free_memory += entry.number_of_pages * 4096;
            }

            print!("{:16x} {:16x} {:?}\n",
                entry.physical_start,
                entry.number_of_pages * 4096,
                typ);
        }
    }

    print!("Total free bytes: {}\n", free_memory);
}
