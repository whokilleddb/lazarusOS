//! This file contains UEFI EFI structures
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::efi::{EfiHandle, EfiSystemTable, EfiStatus};

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

// Get memory map for the System from UEFI
pub fn get_memory_map(){
    // Get the system table
    let system_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // Check null
    if system_table.is_null() {return;}

    // Create an empty memory map
    let mut memory_map = [0u8; 2*1024];
}
