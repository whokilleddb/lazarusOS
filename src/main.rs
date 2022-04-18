#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;
mod panic_handler;
mod mem;
mod efi;

use crate::efi::{EFI_HANDLE, EFI_SYSTEM_TABLE, EFI_STATUS};

#[no_mangle]
extern fn efi_main(_image_handle: EFI_HANDLE, system_table: *mut EFI_SYSTEM_TABLE) -> EFI_STATUS{
    // First, register the system table in a global so we can use it in other places such as the `print!` macro
    unsafe {
        efi::register_system_table(system_table);
    }
    panic!("LazarusOS Is Live!\n");
}
