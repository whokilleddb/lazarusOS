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
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiStatus(pub usize);


/// A scan code and unicode value for an input key press
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

impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        match val {
            0	=> EfiMempryType::EfiReservedMemoryType,
            1	=> EfiMempryType::EfiLoaderCode,
            2	=> EfiMempryType::EfiLoaderData,
            3	=> EfiMempryType::EfiBootServicesCode,
            4	=> EfiMempryType::EfiBootServicesData,
            5	=> EfiMempryType::EfiRuntimeServiceCode,
            6	=> EfiMempryType::EfiRuntimeServicesData,
            7	=> EfiMempryType::EfiConventionalMemory,
            8	=> EfiMempryType::EfiUnusableMemory,
            9	=> EfiMempryType::EfiACPIReclaimMemory,
           10	=> EfiMempryType::EfiACPIMemoryNVS,
           11	=> EfiMempryType::EfiMemoryMappedIO,
           12	=> EfiMempryType::EfiMemoryMappedIOPortSpace,
           13	=> EfiMempryType::EfiPalCode,
           14	=> EfiMempryType::EfiPersistentMemory,
           _	=> EfiMempryType::Reserved,
        }
    }
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

// Get memory map for the System from UEFI
pub fn get_memory_map(){
    // Get the system table
    let system_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // Check null
    if system_table.is_null() {return;}

    // Create an empty memory map
    let mut memory_map = [0u8; 2*1024];
}
