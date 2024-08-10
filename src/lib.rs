#![allow(clippy::missing_safety_doc)]

#[cfg(test)]
mod tests {
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::raw::HANDLE;
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::Storage::Vhd::{
        AttachVirtualDisk, OpenVirtualDisk, ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME,
        ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY, VIRTUAL_DISK_ACCESS_ATTACH_RO, VIRTUAL_STORAGE_TYPE,
        VIRTUAL_STORAGE_TYPE_DEVICE_VHD, VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
    };

    #[test]
    fn attach_vhd() {
        let mut handle: HANDLE = std::ptr::null_mut();

        let storage_type = VIRTUAL_STORAGE_TYPE {
            DeviceId: VIRTUAL_STORAGE_TYPE_DEVICE_VHD,
            VendorId: VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
        };

        let path = std::ffi::OsStr::new(r"C:\Users\sam\dev\vhdrs\file.vhd")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>();

        let open_result = unsafe {
            OpenVirtualDisk(
                &storage_type,
                path.as_ptr(),
                VIRTUAL_DISK_ACCESS_ATTACH_RO,
                0,
                std::ptr::null(),
                &mut handle,
            )
        };

        assert_eq!(open_result, ERROR_SUCCESS);

        let mut flags = 0;
        flags |= ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME;
        flags |= ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY;

        let attach_result = unsafe {
            AttachVirtualDisk(
                handle,
                std::ptr::null_mut(),
                flags,
                0,
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        assert_eq!(attach_result, ERROR_SUCCESS);
    }
}
