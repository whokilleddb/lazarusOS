#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;
mod panic_handler;
mod mem;

#[no_mangle]
extern fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus{
    // First, register the system table in a global so we can use it in other places such as the `print!` macro
    unsafe {
        efi::register_system_table(system_table);
    }
}
