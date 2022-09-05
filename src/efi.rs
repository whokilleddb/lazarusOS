//! This file contains UEFI EFI structures
#![allow(non_camel_case_types)]
#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use core::sync::atomic::{AtomicPtr, Ordering};


/// Struct to store EFI_HANDLE
/// Definition is analogous to the C definition as seen in:
/// https://dox.ipxe.org/include_2ipxe_2efi_2efi_8h_source.html#l00050
#[derive(Clone, Copy, Debug)]
#[repr(C)]  // See: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
pub struct EFI_HANDLE(usize);


/// Struct to store UEFI status code
/// For definition, see: https://developer.apple.com/documentation/kernel/efi_status
/// See(Page 23): https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EFI_STATUS(pub usize);


/// A scan code and unicode value for an input key press
/// See: https://dox.ipxe.org/structEFI__INPUT__KEY.html
/// See: https://docs.rs/uefi-ffi/latest/uefi_ffi/struct.EFI_INPUT_KEY.html
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EFI_INPUT_KEY {
    // Scan code for key press
    ScanCode: u16,

    // Unicode Representation of the key
    UnicodeChar : u16,
}


/// EFI memory types
/// See: https://youtu.be/VW6WIe3aY_Q?t=577
/// See: https://docs.rs/redox_uefi/latest/uefi/memory/enum.MemoryType.html
/// See: https://uefi.org/specs/ACPI/6.4/15_System_Address_Map_Interfaces/uefi-getmemorymap-boot-services-function.html
/// See: https://dox.ipxe.org/UefiMultiPhase_8h.html#a0e2cdd0290e753cca604d3977cbe8bb9
///
/// Boot Services vs Runtime Services
/// See: https://www.reddit.com/r/osdev/comments/gougq6/uefi_boot_services_vs_runtime_services/
/// See: https://forum.osdev.org/viewtopic.php?f=1&t=40937
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum EFI_MEMORY_TYPE {
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
    EfiMaxMemoryType,
}

/// Corresponding numeric codes to each of UEFI Memory types
impl From<u32> for EFI_MEMORY_TYPE {
    fn from(val: u32) -> Self {
        match val {
            0 => EFI_MEMORY_TYPE::EfiReservedMemoryType,
            1 => EFI_MEMORY_TYPE::EfiLoaderCode,
            2 => EFI_MEMORY_TYPE::EfiLoaderData,
            3 => EFI_MEMORY_TYPE::EfiBootServicesCode,
            4 => EFI_MEMORY_TYPE::EfiBootServicesData,
            5 => EFI_MEMORY_TYPE::EfiRuntimeServiceCode,
            6 => EFI_MEMORY_TYPE::EfiRuntimeServicesData,
            7 => EFI_MEMORY_TYPE::EfiConventionalMemory,
            8 => EFI_MEMORY_TYPE::EfiUnusableMemory,
            9 => EFI_MEMORY_TYPE::EfiACPIReclaimMemory,
           10 => EFI_MEMORY_TYPE::EfiACPIMemoryNVS,
           11 => EFI_MEMORY_TYPE::EfiMemoryMappedIO,
           12 => EFI_MEMORY_TYPE::EfiMemoryMappedIOPortSpace,
           13 => EFI_MEMORY_TYPE::EfiPalCode,
           14 => EFI_MEMORY_TYPE::EfiPersistentMemory,
           _ => EFI_MEMORY_TYPE::EfiMaxMemoryType,
        }
    }
}

impl EFI_MEMORY_TYPE {
    // Returns whether or not this memory is available for general purpose use after the boot services have been exited

    // From Wikipedia: https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface#Services
    // EFI defines two types of services: `boot` services and `runtime` services.
    // Boot services are available only while the firmware owns the platform (i.e., before the `ExitBootServices()` call),
    // and they include text and graphical consoles on various devices, and bus, block and file services.
    // Runtime services are still accessible while the operating system is running;
    // they include services such as date, time and NVRAM access.`

    fn avail_post_exit_boot_services(&self) -> bool {
        match self{
            EFI_MEMORY_TYPE::EfiBootServicesCode |
            EFI_MEMORY_TYPE::EfiBootServicesData |
            EFI_MEMORY_TYPE::EfiConventionalMemory |
            EFI_MEMORY_TYPE::EfiPersistentMemory => true,

            _ => false
        }
    }
}


/// Data structure that preceeds all the standard EFI Table types
/// See: https://dox.ipxe.org/structEFI__TABLE__HEADER.html
#[repr(C)]
struct EFI_TABLE_HEADER{
    // A 64 bit signature that identifies the type of table that follows.
    // Unique Signatures have been generated for the EFI_SYSTEM_TABLE, the Boot Service
    // Table, and the EFI Runtime Services Table
    Signature: u64,

    // The revision of the EFI Specification to which this table confroms. The 
    // upper 16 bits of this field contains the major revision value, and the
    // lower 16 bits contains the minor revision value. The minor revision values 
    // are binary coded decimals and are limited to the range of 00-99
    //
    //  When printed or displayed UEFI spec revision is referred as (Major
    //  revision).(Minor revision upper decimal).(Minor revision lower
    //  decimal) or (Major revision).(Minor revision upper decimal) in
    //  case Minor revision lower decimal is set to 0. For example:
    //
    //  A specification with the revision value ((2<<16) | (30)) would be referred as 2.3
	//
    //  A specification with the revision value ((2<<16) | (31)) would be referred as 2.3.1
    Revision: u32,

    // The size of the entire table including EFI_TABLE_HEADER in bytes
    HeaderSize: u32,

    // The 32 bit CRC for the entire table. This value is computed by
    // setting this field to 0, and computing the 32-bit CRC for HeaderSize bytes.
    CRC32: u32,

    // Reserved field that must be set to 0
    Reserved: u32,
}


/// Memory descriptor to store the return type from `GetMemoryMap()`
/// See: https://dox.ipxe.org/structEFI__MEMORY__DESCRIPTOR.html
/// See: https://github.com/tianocore/edk2/blob/91a03f78ba0b75bc4ed2c4b756cbe57c685d9c72/MdePkg/Include/Uefi/UefiSpec.h#L127
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
struct EFI_MEMORY_DESCRIPTOR{
    // Type of the memory region.
    Type: u32,

    // Physical address of the first byte in the memory region
    // It must be aligned to a 4KiB boundary and must not be above 
    // 0xfffffffffffff000. Why the funny hex address? 
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // Why 4KiB?
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/comment/i50kny8/?utm_source=share&utm_medium=web2x&context=3
    PhysicalAddress: u64, // 64 bit address

    // Virtual address of the first byte in the memory region
    // It must be aligned to a 4KiB boundary and must not be above 
    // 0xfffffffffffff000. Why the funny hex address? 
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/help_with_understanding_uefi_memory_descriptor/
    // Why 4KiB?
    // See: https://www.reddit.com/r/osdev/comments/u56t5c/comment/i50kny8/?utm_source=share&utm_medium=web2x&context=3
    VirtualAddress: u64, // 64 bit address

    // Number of 4KiB pages in the memory region. Number of pages cannot
    // Number of Pages must not be 0, and must not be any value
    // that would represent a memory page with a start address,
    // either physical or virtual, above 0xfffffffffffff000.
    NumberOfPages: u64,

    // Attributes of the memory region that describe the bit mask of capabilities
    // for that memory region, and not necessarily the current settings for that
    // memory region.
    Attribute: u64,
}


/// Contains a table header and pointers to all boot services
/// See: https://dox.ipxe.org/UefiSpec_8h_source.html#l01836
#[repr(C)]
struct EFI_BOOT_SERVICES {
    // The table header for the EFI Boot Service Table. 
    // This header contains the EF_BOOT_SERVICES_SIGNATURE and 
    // EFI_BOOT_SERVICES_REVISION values along with the size of the 
    // EFI_BOOT_SERVICES structure and a 32-bit CRC to verify the contents
    // of the EFI Boot Service Tables are valid
    Hdr: EFI_TABLE_HEADER,
 
    // TASK PRIORITY SERVICES

    // Raise the task priority level
    _RaiseTPL: usize,

    // Restores/Lowers the task priority level
    _RestoreTPL: usize,

    // MEMORY SERVICES

    // Allocate Pages of a particular type
    _AllocatePages: usize,

    // Frees allocated pages
    _FreePages: usize,

    // Returns the current boot services memory map and memory map key
    // See Page 157: https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf
    GetMemoryMap: unsafe fn(
        MemoryMapSize: &mut usize,
        MemoryMap: *mut u8,
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptorVersion: &mut u32,
    ) -> EFI_STATUS,

    // Allocates a pool of a particular type
    _AllocatePool: usize,
    
    // Free Allocate pool
    _FreePool: usize,

    // EVENT & TIMER SERVICES

    // Creates a general-purpose event structure
    _CreateEvent: usize,

    // Sets an event to be signaled at a particular time
    _SetTimer: usize,

    // Stop execution until an event is signaled
    _WaitForEvent: usize,

    // Signals an Event
    _SignalEvent: usize,

    // Closes and frees an event structure
    _CloseEvent: usize,

    // Check whether an event is in the signaled state
    _CheckEvent: usize,

    // PROTOCOL HANDLER SERVICES

    // Installs a protocol interface on a device handle
    _InstallProtocolInterface: usize,

    // Reinstalls a protocol interface on a device handle
    _ReinstallProtocolInterface: usize,

    // Removes a protocol interface on a device handle
    _UninstallProtocolInterface: usize,

    // Queries a handle to check if it supports a specific protocol
    _HandleProtocol: usize,

    // Reserved
    _Reserved: usize,

    // Register an event that is to be signaled whenever an interface is
    // installed for a specified protocol
    _RegisterProtocolNotify: usize,

    // Returns an array of handles that support a specified protocol
    _LocateHandle: usize,

    // Locate all devices on a device path that support a specified protocol and 
    // returns the handle to the device that is closest to the path
    _LocateDevicePath: usize,

    // Adds, update, or removes a configuration table from the EFI System Table
    _InstallConfigurationTable: usize,

    // IMAGE SERVICES

    // Loads an EFI image into memory
    _LoadImage: usize,

    // Transfer control to a loaded image's entry point
    _StartImage: usize,

    // Exits an image's entry point
    _Exit: usize,

    // Unloads an image
    _UnloadImage: usize,

    // Terminate boot services 
    // See Page 222: https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf 
    ExitBootServices: unsafe fn(
        ImageHandle: EFI_HANDLE,
        MapKey: usize
    )-> EFI_STATUS,
}


/// This protocol is used to obtain input from the ConsoleIn device. The EFI specification
/// requires that EFI_SIMPLE_TEXT_INPUT_PROTOCOL supports the same language as
/// the corresponding EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
/// See page 467: https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf
#[repr(C)]
struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    // Reset Input Device hardware
    // See: https://dox.ipxe.org/SimpleTextIn_8h.html#adf982c71dcc0af2e4495044e66201b53
    Reset: unsafe fn(
        This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ExtendedVerification: bool) -> EFI_STATUS, 

    // Reads the next keystroke from input device
    // See: https://dox.ipxe.org/SimpleTextIn_8h.html#a09083a7dedf5d4f8fd1d437289869d39
    ReadKeyStroke: unsafe fn(
        This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Key: *mut EFI_INPUT_KEY,
    )-> EFI_STATUS,
    
    // Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key
    // to be available. We don't use the event API thus we dont expose this function pointer
    _WaitForKey: usize,
}


/// This protocol is used to control Text Based output devices
/// See page 470: https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf
/// See: https://edk2-docs.gitbook.io/edk-ii-uefi-driver-writer-s-guide/22_text_console_driver_design_guidelines/readme.3
#[repr(C)]
struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    // Resets the text output device hardware
    Reset: unsafe fn(
        This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ExtendedVerification: bool) -> EFI_STATUS,  

    // Write String to output device
    // See: https://dox.ipxe.org/SimpleTextOut_8h.html#afcf652d19afcb35e585089c15a51b115
    OutputString: unsafe fn(
        This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
        String: *const u16,
    )->EFI_STATUS,

    // Verfies that all the characters in the string can be output to the target device
    TestString: unsafe fn(
        This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
        String: *const u16,
    )->EFI_STATUS,

    // Returns information for an available text mode 
    // that the output device supports
    _QueryMode: usize,

    // Sets output device to a specific mode
    _SetMode: usize,

    // Set background and foreground colors for the OutputString()
    // and ClearScreen() functions
    _SetAttribute: usize,

    // Clears output device to display the currently selected background color
    _ClearScreen: usize,

    // Sets the current co-ordinates of the cursor position
    _SetCursorPosition: usize, 

    // Makes the cursor visible or invisible
    _EnableCursor: usize,

    // Pointer to SIMPLE_TEXT_OUTPUT_MODE data
    _Mode: usize,
}

/// Contains pointers to runtime and boot time service tables
/// See: https://dox.ipxe.org/structEFI__SYSTEM__TABLE.html
#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    // The table header for the EFI System Table. This header contains the
    // EFI_SYSTEM_TABLE_SIGNATURE and EFI_SYSTEM_TABLE_REVISION values along
    // with the size of the EFI_SYSTEM_TABLE structure and a 32-bit CRC to
    // verify that the contents of the EFI System table are valid
    Hdr: EFI_TABLE_HEADER,

    // A pointer to a null terminated string that identifies the vendor that
    // produces system firmware for the platform
    FirmwareVendor: *const u16,

    // A firmware vendor specific value that identifies the revision of
    // the system firmware for the platform
    FirmwareRevision: u32,

    // The handle for the active console input device. This handle must support
    // EFI_SIMPLE_TEXT_INPUT_PROTOCOL and EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL
    ConsoleInHandle: EFI_HANDLE,

    // A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is 
    // associated with ConsoleInHandle
    ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,

    // The handle for the active console output device. This handle must support
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL and EFI_SIMPLE_TEXT_OUTPUT_EX_PROTOCOL
    ConsoleOutHandle: EFI_HANDLE,

    // A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is 
    // associated with ConsoleOutHandle
    ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    // The handle for the active console standard error device. This handle must support
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL and EFI_SIMPLE_TEXT_OUTPUT_EX_PROTOCOL
    StandardErrHandle: EFI_HANDLE,

    // A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is 
    // associated with StandardErrorHandle
    StdErr: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    // A pointer to the EFI Runtime Service handle
    _RuntimeServices: usize,

    // A pointer to the EFI Boot Service handle
    BootServices: *const EFI_BOOT_SERVICES,
}

/// Pointer to the EFI System Table which is saved upon the entry of the kernel
/// This pointer is needed for Console I/O
/// This needs to be global because `print()` functions don't get a `&self` pointer
/// D3eclaring it global is the only way we can get access to the system table in a print macro
static EfiSystemTable: AtomicPtr<EFI_SYSTEM_TABLE> = AtomicPtr::new(core::ptr::null_mut());


/// Read More about UEFI System Table: https://edk2-docs.gitbook.io/edk-ii-uefi-driver-writer-s-guide/3_foundation/33_uefi_system_table
/// EFI System Table: https://dox.ipxe.org/structEFI__SYSTEM__TABLE.html
/// For Detailed Reading, See Chapter 4(Page: 93): https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf


/// Register a system table pointer.
/// Only the first non-null system table pointer will be stored in the `EfiSystemTable` global
pub unsafe fn register_system_table(system_table: *mut EFI_SYSTEM_TABLE){
    // See: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.compare_exchange
    match EfiSystemTable.compare_exchange(
        core::ptr::null_mut(),
        system_table,
        Ordering::SeqCst,    // See: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
        Ordering::SeqCst){
        Err(e) => {
            eprint!("[!] Failed to register EFI_SYSTEM_TABLE\n");
            eprint!("[!] EFI_SYSTEM_TABLE at Error: {:?}\n",e);
            return ;
        },
        _ => (),
    };

    print!("[i] Registered EFI_SYSTEM_TABLE!\n");
    print!("[i] EFI_SYSTEM_TABLE is located at: {:?}\n", system_table);
}


/// Write a `string` to UEFI output
pub fn output_string(string: &str){
    // Get the system table
    let system_table = EfiSystemTable.load(Ordering::SeqCst);

    // Check if pointer is null
    if system_table.is_null(){return ;}

    // Get the console output_pointer
    let console_std_out = unsafe {
        (*system_table).ConOut
    };

    // Check if console_std_out is NULL
    if console_std_out.is_null(){return ;}

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
                    .OutputString)(console_std_out, tmp.as_ptr());
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
                .OutputString)(console_std_out, tmp.as_ptr());
        }
    }
}


/// Write a `string` to UEFI stderr
pub fn stderr_string(string: &str){
    // Get the system table
    let system_table = EfiSystemTable.load(Ordering::SeqCst);

    // Check if pointer is null
    if system_table.is_null(){return ;}

    // Get the console output_pointer
    let console_std_err = unsafe {
        (*system_table).StdErr
    };

    // Check is console_std_err is NULL
    if console_std_err.is_null(){return ;}

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
                ((*console_std_err)
                    .OutputString)(console_std_err, tmp.as_ptr());
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
            ((*console_std_err)
                .OutputString)(console_std_err, tmp.as_ptr());
        }
    }
}


/// Get memory map for the System from UEFI
/// See: https://wiki.osdev.org/Detecting_Memory_(x86)
pub fn GetMemoryMap(){
    // Get the system table
    let system_table = EfiSystemTable.load(Ordering::SeqCst);

    // Check null
    if system_table.is_null() {return;}

    // Create an empty memory map
    // Make sure this size entry is large enough to hold the MemoryMap!
    // Or else, it will throw an error 8000000000000005
    let mut memory_map = [0u8; 8*1024];

    let mut free_memory = 0u64;

    // See: https://www.youtube.com/watch?v=VW6WIe3aY_Q
    unsafe{
        let mut map_size = core::mem::size_of_val(&memory_map);
        let mut map_key = 0;
        let mut map_descriptor_size = 0;
        let mut map_descriptor_version = 0;


        // GetMemoryMap() Call
        // See: https://uefi.org/specs/ACPI/6.4/15_System_Address_Map_Interfaces/uefi-getmemorymap-boot-services-function.html

        let ret = ((*(*system_table).BootServices).GetMemoryMap)(
            &mut map_size,
            memory_map.as_mut_ptr(),
            &mut map_key,
            &mut map_descriptor_size,
            &mut map_descriptor_version
        );


        // Check if Descriptor Table is empty
        assert!(ret.0 == 0, "{:x?}", ret);
        print!("[i] Memory Map:\n");
        print!("\tPhysical Addr\t  No of Pages\tType\n");

        for off in (0..map_size).step_by(map_descriptor_size) {
            let entry = core::ptr::read_unaligned(
                memory_map[off..].as_ptr() as *const EFI_MEMORY_DESCRIPTOR
            );

            let typ: EFI_MEMORY_TYPE = entry.Type.into();

            if typ.avail_post_exit_boot_services(){
                free_memory += entry.NumberOfPages * 4096;
            }

            print!("{:16x} {:16x}\t{:?}\n",
                entry.PhysicalAddress,
                entry.NumberOfPages * 4096,
                typ
            );
        }
    }

    print!("\n[+] Total free bytes: {}\n", free_memory);
}
