//! This file contains UEFI EFI structures
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::efi::{EfiHandle, EfiSystemTable, EfiStatus};

/// Pointer to the EFI System Table which is saved upon the entry of the kernel
/// This pointer is needed for Console I/O
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());