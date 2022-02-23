#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;
mod panic_handler;
mod mem;

#[no_mangle]
extern fn efi_main(){
    //See: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
    unsafe{
        core::ptr::write_volatile(0x4141414141414141 as *mut u64, 0);
    }

}
