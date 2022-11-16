#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;
mod panic_handler;
mod mem;
mod efi;

use crate::efi::{EFI_HANDLE, EFI_SYSTEM_TABLE, EFI_STATUS};

/// See: https://edk2-docs.gitbook.io/edk-ii-module-writer-s-guide/4_uefi_applications/42_write_uefi_application_entry_point
/// The EFI Firmware passes the EFI_SYSTEM_TABLE to efi_main()
#[no_mangle]
extern fn efi_main(image_handle: EFI_HANDLE, system_table: *mut EFI_SYSTEM_TABLE) -> EFI_STATUS{
    // First, register the system table in a global so we can use it in other places such as the `print!` macro
    unsafe {
        efi::register_system_table(system_table);
    }

    efi::GetMemoryMap(image_handle);
    
    panic!("LazarusOS Is Live!\n");
}
