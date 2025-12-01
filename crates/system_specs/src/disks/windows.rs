#![allow(non_upper_case_globals)]
use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::ptr;
use std::mem::size_of;

pub struct DiskInfo {
    pub label: String,
    pub storage_type: String,
    pub partitions: Vec<PartitionInfo>,
}

pub struct PartitionInfo {
    pub mount_point: String,
    pub filesystem: String,
    pub total_bytes: u64,
    
    pub free_bytes: u64,
}

// ─────────────────────────────────────────────────────────────
// Win32 Headers (manually defined, std-only)
// ─────────────────────────────────────────────────────────────

type HANDLE = *mut core::ffi::c_void;
type DWORD = u32;
type BOOL = i32;
type LPCWSTR = *const u16;
type LPWSTR = *mut u16;

const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const IOCTL_STORAGE_QUERY_PROPERTY: DWORD = 0x2D1400;

#[repr(C)]
struct STORAGE_PROPERTY_QUERY {
    property_id: DWORD,
    query_type: DWORD,
    additional_parameters: [u8; 1],
}

#[repr(C)]
#[derive(Default, Debug)]
struct STORAGE_DEVICE_DESCRIPTOR {
    version: DWORD,
    size: DWORD,
    device_type: u8,
    device_type_modifier: u8,
    removable_media: BOOL,
    command_queueing: BOOL,
    vendor_id_offset: DWORD,
    product_id_offset: DWORD,
    product_revision_offset: DWORD,
    serial_number_offset: DWORD,
    bus_type: DWORD,
    raw_properties_length: DWORD,
    raw_device_properties: [u8; 1],
}

const StorageDeviceProperty: DWORD = 0;
const PropertyStandardQuery: DWORD = 0;

const BusTypeNvme: DWORD = 17;
const BusTypeUsb: DWORD = 7;
const BusTypeSd: DWORD = 12;
const BusTypeMmc: DWORD = 13;
const BusTypeAta: DWORD = 3;
const BusTypeAtapi: DWORD = 2;
const BusTypeSata: DWORD = 11;
const BusTypeScsi: DWORD = 1;
const BusTypeSas: DWORD = 10;
const BusTypeFibre: DWORD = 6;
const BusTypeVirtual: DWORD = 14;
const BusTypeFileBackedVirtual: DWORD = 15;
const BusTypeSpaces: DWORD = 16;
const BusTypeUfs: DWORD = 19;

unsafe extern "system" {
    fn GetDriveTypeW(lpRootPathName: LPCWSTR) -> DWORD;
    fn GetDiskFreeSpaceExW(
        lpDirectoryName: LPCWSTR,
        free_avail: *mut u64,
        total: *mut u64,
        total_free: *mut u64,
    ) -> BOOL;

    fn GetVolumeInformationW(
        root: LPCWSTR,
        volume_name: LPWSTR,
        volume_name_size: DWORD,
        serial: *mut DWORD,
        max_comp_len: *mut DWORD,
        flags: *mut DWORD,
        fs_name: LPWSTR,
        fs_name_size: DWORD,
    ) -> BOOL;

    fn CreateFileW(
        name: LPCWSTR,
        access: DWORD,
        share: DWORD,
        sec: *const core::ffi::c_void,
        create: DWORD,
        flags: DWORD,
        template: HANDLE,
    ) -> HANDLE;

    fn DeviceIoControl(
        h: HANDLE,
        code: DWORD,
        in_buf: *mut core::ffi::c_void,
        in_size: DWORD,
        out_buf: *mut core::ffi::c_void,
        out_size: DWORD,
        out_returned: *mut DWORD,
        overlapped: *mut core::ffi::c_void,
    ) -> BOOL;

    fn CloseHandle(h: HANDLE);
}

// ─────────────────────────────────────────────────────────────
// Detect SSD / HDD / NVMe / USB / SD / MMC / EMMC / etc.
// ─────────────────────────────────────────────────────────────

unsafe fn detect_storage_type(root_letter: char) -> String {
    let path = format!(r"\\.\{}:", root_letter);
    let wide: Vec<u16> =
        OsString::from(path).encode_wide().chain(std::iter::once(0)).collect();

    let h = unsafe {
        CreateFileW(
            wide.as_ptr(),
            0,
            0,
            ptr::null(),
            3, // OPEN_EXISTING
            0,
            ptr::null_mut(),
        )
    };
    if h == INVALID_HANDLE_VALUE {
        return "Unknown".into();
    }

    let query = STORAGE_PROPERTY_QUERY {
        property_id: StorageDeviceProperty,
        query_type: PropertyStandardQuery,
        additional_parameters: [0],
    };

    let mut buf = [0u8; 1024];
    let mut ret = 0u32;
    let ok = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            &query as *const _ as *mut _,
            size_of::<STORAGE_PROPERTY_QUERY>() as DWORD,
            buf.as_mut_ptr() as *mut _,
            buf.len() as DWORD,
            &mut ret,
            ptr::null_mut(),
        )
    };

    unsafe {CloseHandle(h)};
    if ok == 0 || ret < size_of::<STORAGE_DEVICE_DESCRIPTOR>() as DWORD {
        return "Unknown".into();
    }

    #[cfg(debug_assertions)]
    {
        println!("Raw bytes returned ({} bytes):", ret);
        for (i, byte) in buf.iter().take(ret as usize).enumerate() {
            if i % 16 == 0 {
                print!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!("\n");
    }
    let desc = unsafe {&*(buf.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR)};
    unsafe fn read_string(buf: &[u8], offset: u32) -> Option<String> {
        if offset == 0 || (offset as usize) >= buf.len() { return None; }
        let mut end = offset as usize;
        while end < buf.len() && buf[end] != 0 { end += 1; }
        Some(String::from_utf8_lossy(&buf[offset as usize..end]).trim().to_string())
    }
    #[cfg(debug_assertions)]
    {
        println!("{:?}", desc);
        println!("=== Debug info for drive {} ===", root_letter);
        println!("Size: {}", desc.size);
        println!("DeviceType: 0x{:X}", desc.device_type);
        println!("DeviceTypeModifier: 0x{:X}", desc.device_type_modifier);
        println!("RemovableMedia: {}", desc.removable_media);
        println!("CommandQueueing: {}", desc.command_queueing);
        println!("VendorIdOffset: {}", desc.vendor_id_offset);
        println!("ProductIdOffset: {}", desc.product_id_offset);
        println!("ProductRevisionOffset: {}", desc.product_revision_offset);
        println!("SerialNumberOffset: {}", desc.serial_number_offset);
        println!("BusType: {}", desc.bus_type);
        println!("Raw bytes returned: {}", ret);
        println!("Debug info: {}", desc.bus_type);

        let vendor = unsafe { read_string(&buf, desc.vendor_id_offset)};
        let product = unsafe { read_string(&buf, desc.product_id_offset)};
        let revision = unsafe { read_string(&buf, desc.product_revision_offset)};
        let serial = unsafe { read_string(&buf, desc.serial_number_offset)
            .or_else(|| product.clone())};
        println!("Vendor: {:?}", vendor);
        println!("Product: {:?}", product);
        println!("Revision: {:?}", revision);
        println!("Serial: {:?}", serial);
        println!("==============================");
    }
    match desc.bus_type {
        BusTypeNvme => "NVMe".into(),
        BusTypeUsb => "USB".into(),
        BusTypeSd => "SD".into(),
        BusTypeMmc => "MMC".into(),
        BusTypeUfs => "EMMC".into(),
        BusTypeAta | BusTypeAtapi | BusTypeSata => "HDD".into(),
        BusTypeScsi | BusTypeSas | BusTypeFibre => "SSD".into(),
        BusTypeSpaces | BusTypeVirtual | BusTypeFileBackedVirtual => "SSD".into(),
        _ => {
            if desc.removable_media != 0 {
                return "USB".into();
            }

            // Check DeviceType
            match desc.device_type {
                0x0 => "SSD / HDD".into(), // Direct access device
                0x5 => "CD-ROM".into(),
                0x7 => "Tape".into(),
                _ => "SSD/HDD Unknown".into(),
            }
        },
    }
    
}

// ─────────────────────────────────────────────────────────────
// Enumerate all partitions and group into disks
// ─────────────────────────────────────────────────────────────
impl DiskInfo {
    pub fn get_disks() -> Vec<DiskInfo> {
        let mut partitions_per_letter = Vec::new();

        // Enumerate partitions (drive letters A: -> Z:)
        for letter in b'A'..=b'Z' {
            let mut root = [0u16; 4];
            root[0] = letter as u16;
            root[1] = ':' as u16;
            root[2] = '\\' as u16;

            unsafe {
                if GetDriveTypeW(root.as_ptr()) <= 1 {
                    continue;
                }

                let mut total = 0u64;
                let mut free = 0u64;
                let mut tmp = 0u64;

                if GetDiskFreeSpaceExW(root.as_ptr(), &mut free, &mut total, &mut tmp) == 0 {
                    continue;
                }

                let mut vol = [0u16; 260];
                let mut fs = [0u16; 260];

                GetVolumeInformationW(
                    root.as_ptr(),
                    vol.as_mut_ptr(),
                    260,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    fs.as_mut_ptr(),
                    260,
                );

                let filesystem = {
                    let len = fs.iter().position(|&c| c == 0).unwrap_or(0);
                    OsString::from_wide(&fs[..len]).to_string_lossy().into_owned()
                };

                partitions_per_letter.push((
                    letter as char,
                    PartitionInfo {
                        mount_point: format!("{}:\\", letter as char),
                        filesystem,
                        total_bytes: total,
                        free_bytes: free,
                    },
                ));
            }
        }

        // Group by physical disk:
        // In Windows, each drive letter IS a partition/volume,
        // and using the device path \\.\X: gives the parent storage device.
        //
        // So each letter = one Partition, one Disk containing only that Partition.
        // If you later want real multi-partition disks, we can add
        // Win32 Volume Management APIs (VDS, IVdsPack).

        let mut disks = Vec::new();

        for (letter, part) in partitions_per_letter {
            unsafe {
                let storage_type = detect_storage_type(letter);

                // Volume label
                let root_str = format!("{}:\\", letter);
                let wide: Vec<u16> = OsString::from(&root_str)
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();

                let mut volname = [0u16; 260];
                GetVolumeInformationW(
                    wide.as_ptr(),
                    volname.as_mut_ptr(),
                    260,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    0,
                );

                let label = {
                    let len = volname.iter().position(|&c| c == 0).unwrap_or(0);
                    OsString::from_wide(&volname[..len]).to_string_lossy().into_owned()
                };

                disks.push(DiskInfo {
                    label,
                    storage_type,
                    partitions: vec![part],
                });
            }
        }

        disks
    }
}