#![allow(non_camel_case_types)]
#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

/// A very basic ACPI implentation for extracting basic information 
/// about CPU topography and NUMA regions

use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering, AtomicU8};
use core::convert::TryInto;
use crate::mm::{self, PhysAddr};


/// Maximum Number of cores allowed on the system
pub const MAX_CORES: usize = 1024;


/// In-memory representation of RSDP ACPI structure(v 1.0)
/// RSDP Strcut definiton -> https://wiki.osdev.org/RSDP
/// "packed" -> https://developer.arm.com/documentation/dui0491/i/Compiler-specific-Features/--attribute----packed---variable-attribute
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct RSDPDescriptor {
    Signature:      [u8; 8],
    Checksum:       u8,
    OEMID:          [u8; 6],
    Revision:       u8,
    RsdtAddress:    u32,
}


/// In-memory representation of Extended RSDP ACPI structure(v 2.0)
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct RSDPDescriptor20 {
    firstPart:          RSDPDescriptor,
    Length:             u32,
    XsdtAddress:        u64 ,
    ExtendedChecksum:   u8,
    reserved:           [u8; 3],
}


/// In-memory representation of ACPI Table Header
/// Master ACPI Table Header. This common header is used by
/// all ACPI tables except the RSDP and FACS.
/// 
/// See: https://github.com/torvalds/linux/blob/eb7081409f94a9a8608593d0fb63a1aa3d6f95d8/include/acpi/actbl.h#L68
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct acpi_table_header {
    signature:              [u8; 4],
    length:                  u32,
    revision:                u8,
    checksum :               u8,
    oem_id:                 [u8; 6],
    oem_table_id:            u64,
    oem_revision:            u32,
    asl_compiler_id:         u32,
    asl_compiler_revision:   u32
}


/// Parse a standard ACPI table header 
/// This will parse out the header, validate the checksum and length,
/// and return a physical address and size of payload following the header
/// Check: https://wiki.osdev.org/RSDP#Validating_the_RSDP
unsafe fn parse_header(addr: PhysAddr) -> (acpi_table_header, PhysAddr, usize){
    // Read the header
    let head = mm::read_phys::<acpi_table_header>(addr);

    // Get the number of bytes for the table
    let payload_len = head.length
            .checked_sub(size_of::<acpi_table_header>() as u32)
            .expect("[!] Integer Underflow on table length");

    // Check the checksum for the table
    let sum = (addr.0..addr.0 + head.length as u64)
            .fold(0u8, |acc, paddr| {
                acc.wrapping_add(mm::read_phys(PhysAddr(paddr as u64)))
            });

    // Validate signature
    assert!(sum == 0, "[!] Table checksum invalid {:?}",
                core::str::from_utf8(&head.signature)
        );

    // Return the parsed header
    (
        head, 
        PhysAddr(addr.0 + size_of::<acpi_table_header>() as u64),
        payload_len as usize
    )
}


/// Initialize the ACPI subsystem 
/// Mainly looking for APICs and memory maps
/// Bring up all cores on system
pub unsafe fn init(){
    // Specification says that we have to scan the first 1KiB of the EDBA and
    // the range from 0xe0000 to 0xfffff
    // See: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_8_final.pdf
    // See: 2.5.1.2 Fixed Resources for Working with Option ROMs
    let ebda = mm::read_phys::<u16>(PhysAddr(0x40e)) as u64;

    // Compute the regions we need to scan for the RSDP
    let regions = [
        // First 1 KiB of the EBDA
        (ebda, ebda + 1024 - 1),

        // From 0xe0000 to 0xfffff
        (0xe0000, 0xfffff)
    ];

    // Holds the RSDP structure if found
    let mut rsdp = None;

    'rsdp_search: for &(start, end) in &regions {
        // 16-byte align the start address upwards
        let start = (start + 0xf) & !0xf;

        // Go through each 16 byte offset in the range specified
        for paddr in (start..=end).step_by(16) {
            // Compute the end address of RSDP structure
            let struct_end = start + size_of::<RSDPDescriptor>() as u64 - 1;

            // Break out of the scan if we are out of bounds of this region
            if struct_end > end {
                break;
            }

            // Read the table
            let table = mm::read_phys::<RSDPDescriptor>(PhysAddr(paddr));
            if &table.Signature != b"RSD PTR " {
                continue;
            }
            
            // Read the tables bytes so we can checksum it
            let table_bytes = mm::read_phys::
                <[u8; size_of::<RSDPDescriptor>()]>(PhysAddr(paddr));

            // Checksum the table
            let sum = table_bytes.iter()
                .fold(0u8, |acc, &x| acc.wrapping_add(x));
            if sum != 0 {
                continue;
            }

            // Checksum the extended RSDP if needed
            if table.Revision > 0 {
                // Read the tables bytes so we can checksum it
                const N: usize = size_of::<RSDPDescriptor20>();
                let extended_bytes = mm::read_phys::<[u8; N]>(PhysAddr(paddr));

                // Checksum the table
                let sum = extended_bytes.iter()
                    .fold(0u8, |acc, &x| acc.wrapping_add(x));
                if sum != 0 {
                    continue;
                }
            }

            rsdp = Some(table);
            break 'rsdp_search;
        }
    }

    // Get access to the RSDP
    let _rsdp = rsdp.expect("Failed to find RSDP for ACPI");

    /*
    // Parse out the RSDT
    let (rsdt, rsdt_payload, rsdt_size) =
        parse_header(PhysAddr(rsdp.rsdt_addr as u64));

    // Check the signature and 
    assert!(&rsdt.Signature == b"RSDT", "RSDT signature mismatch");
    assert!((rsdt_size % size_of::<u32>()) == 0,
        "Invalid table size for RSDT");
    let rsdt_entries = rsdt_size / size_of::<u32>();

    // Set up the structures we're interested as parsing out as `None` as some
    // of them may or may not be present.
    let mut apics          = None;
    let mut apic_domains   = None;
    let mut memory_domains = None;

    // Go through each table described by the RSDT
    for entry in 0..rsdt_entries {
        // Get the physical address of the RSDP table entry
        let entry_paddr = rsdt_payload.0 as usize + entry * size_of::<u32>();

        // Get the pointer to the table
        let table_ptr: u32 = mm::read_phys(PhysAddr(entry_paddr as u64));

        // Get the signature for the table
        let signature: [u8; 4] = mm::read_phys(PhysAddr(table_ptr as u64));

        if &signature == b"APIC" {
            // Parse the MADT
            assert!(apics.is_none(), "Multiple MADT ACPI table entries");
            apics = Some(parse_madt(PhysAddr(table_ptr as u64)));
        } else if &signature == b"SRAT" {
            // Parse the SRAT
            assert!(apic_domains.is_none() && memory_domains.is_none(),
                "Multiple SRAT ACPI table entries");
            let (ad, md) = parse_srat(PhysAddr(table_ptr as u64));
            apic_domains   = Some(ad);
            memory_domains = Some(md);
        }
    }

    if let (Some(ad), Some(md)) = (apic_domains, memory_domains) {
        // Register APIC to domain mappings
        for (&apic, &node) in ad.iter() {
            APIC_TO_DOMAIN[apic as usize].store(node.try_into().unwrap(),
                Ordering::Relaxed);
        }

        // Notify the memory manager of the known APIC -> NUMA mappings
        crate::mm::register_numa_nodes(ad, md);
    }

    // Set the total core count based on the number of detected APICs on the
    // system. If no APICs were mentioned by ACPI, then we can simply say there
    // is only one core.
    TOTAL_CORES.store(apics.as_ref().map(|x| x.len() as u32).unwrap_or(1),
                      Ordering::SeqCst);

    // Initialize the state of all the known APICs
    if let Some(apics) = &apics {
        for &apic_id in apics {
            APICS[apic_id as usize].store(ApicState::Offline as u8,
                                          Ordering::SeqCst);
        }
    }

    // Set that our core is online
    APICS[core!().apic_id().unwrap() as usize]
        .store(ApicState::Online as u8, Ordering::SeqCst);

    // Launch all other cores
    if let Some(valid_apics) = apics {
        // Get exclusive access to the APIC for this core
        let mut apic = core!().apic().lock();
        let apic = apic.as_mut().unwrap();

        // Go through all APICs on the system
        for apic_id in valid_apics {
            // We don't want to start ourselves
            if core!().apic_id().unwrap() == apic_id { continue; }

            // Mark the core as launched
            set_core_state(apic_id, ApicState::Launched);

            // Launch the core
            apic.ipi(apic_id, 0x4500);
            apic.ipi(apic_id, 0x4608);
            apic.ipi(apic_id, 0x4608);

            // Wait for the core to come online
            while core_state(apic_id) != ApicState::Online {}
        }
    }*/
}
