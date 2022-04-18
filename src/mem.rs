/// This implements libc mem* functions in Rust
/// alternatively, you can add the following lines to .cargo/config.toml
///
/// [unstable]
/// build-std = ["core", "compiler_builtins"]      
/// build-std-features = ["compiler-builtins-mem"]
///
/// But note that these functions are super slow(almost 4times slower)
///
/// libc `memcpy` implementation in Rust
/// Note that this is in accordance to man memcpy(3)
///
/// Parameters:
/// dest: Pointer to memory to copy to
/// src: Pointer to memory to copy from
/// n: Number of bytes to copy
///
/// Returns:
/// dest: Pointer to memory to copy to
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8{
    core::arch::asm!("rep movsb",   // Move string block `rcx` number of times
            inout("rcx") n => _,        // move  value of n to rcx to repeat instruction n times
            inout("rdi") dest => _,
            inout("rsi") src => _
        );
    // Architecture independent code
    // See: https://docs.rs/rlibc/1.0.0/src/rlibc/lib.rs.html#41-57
    dest
}


/// libc `memset` implementation in Rust
/// Note that this is in accoradance with man memset(3)
/// 
/// Parameters:
/// 
/// s - Pointer to memory to set
/// c - Character to set `n` bytes in `s` to 
/// n - Number of bytes to set
/// 
/// Returns:
/// s - Pointer to memory to set
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8{
    core::arch::asm!("rep stosb",
            inout("rcx") n => _,
            inout("rdi") s => _,
            in("eax") c as u32
        );
    s
}

/// libc `memcmp` implementation in Rust
/// Note that this is in accoradance with man memcmp(3)
/// 
/// Parameters:
/// 
/// s1 - Pointer to memory to compare to s2
/// s2 - Pointer to memory to compare to s1
/// n - Number of bytes to compare
/// 
/// Returns:
/// The function returns an integer less than, equal to, or greater than zero if the first n bytes of s1 is found, respectively, to be less than, to match, or be greater than the first n bytes of s2.
/// For  a  nonzero  return value, the sign is determined by the sign of the difference between the first pair of bytes that differ in s1 and s2.
#[no_mangle]
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize)-> i32{
    if n==0 {
        return 0;
    }

    let mut i = 0;
    while i < n{
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        i = i.wrapping_add(1);
    }

    0
}

/// libc `memmove` implemntation in Rust
/// Note that this is in accoradance with man memmove(3)
/// 
/// Parameters:
/// dest: Pointer to memory to copy to
/// src: Pointer to memory to copy from
/// n: Number of bytes to copy
///
/// Returns:
/// dest: Pointer to memory to copy to

#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, mut n: usize) -> *mut u8{
    // Check if there is an overlap with the source coming prior to the dest
    // Even if there is an overlap, if the destination is earlier in memory than
    // the source, we can copy forwards
    // +-------------+
    // | src         | src + n
    // +----+--------+----+
    //      | dest        | dest + n
    //      +-------------+

    // Determine if the dest comes after the source and there is overlap between them
    if (dest as usize) > (src as usize) && (src as usize).wrapping_add(n) > (dest as usize){
        // Compute the delta between source and address
        let delta = (dest as usize) - (src as usize);

        // If the non-overlapping region is quite small copy chunk in reverse
        if delta < 64 {
            // 8-byte align the dest with one byte copies
            // See: https://stackoverflow.com/a/54307719
            while n != 0 && ((dest as usize).wrapping_add(n) & 0x7) != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }

            //  Do a reverse copy 8-bytes at a time
            while n >= 8 {
                n = n.wrapping_sub(8);
                
                // Read value to copy
                let val = core::ptr::read_unaligned(
                    src.offset(n as isize) as *const u64
                );

                // Write value to destination
                core::ptr::write(
                    dest.offset(n as isize) as *mut u64, val
                );

            }

            // Copy the remainder
            while n != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }

            return dest;
        }

        // Copy the non-overlapping tail while there are `delta` sized chunks
        while n >= delta {
            // Update the remaining length
            n = n.wrapping_sub(delta);

            // Copy the remaining parts
            let src = src.offset(n as isize);
            let dest = dest.offset(n as isize);
            memcpy(dest,src, delta);
        }

        if n == 0 {
            return dest;
        }
    }

    // Just copy forward
    memcpy(dest, src, n);
    dest
}