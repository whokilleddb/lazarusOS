//! Memory Management Routines

/// A strongly typed physical address
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

/// Read a `T` from physical address add `paddr`
/// Read: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/attributes.html#inline-attribute
#[inline]
pub unsafe fn read_phys<T>(paddr: PhysAddr) -> T {
    // See: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
    core::ptr::read_volatile(paddr.0 as *const T)
}
