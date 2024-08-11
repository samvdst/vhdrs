use error::Error;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::raw::HANDLE;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use windows_sys::Win32::Storage::FileSystem::GetLogicalDriveStringsW;
use windows_sys::Win32::Storage::Vhd::{
    AttachVirtualDisk, OpenVirtualDisk, ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME,
    ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY, VIRTUAL_DISK_ACCESS_ATTACH_RO,
    VIRTUAL_DISK_ACCESS_ATTACH_RW, VIRTUAL_STORAGE_TYPE, VIRTUAL_STORAGE_TYPE_DEVICE_VHD,
    VIRTUAL_STORAGE_TYPE_DEVICE_VHDX, VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
};

pub use error::Result;

mod error;

#[derive(Debug)]
pub struct Vhd {
    path: Vec<u16>,
    handle: HANDLE,
    vhd_type: VhdType,
}

#[derive(Debug)]
pub enum MountMode {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug)]
pub enum VhdType {
    Vhd,
    Vhdx,
}

impl Vhd {
    pub fn new<P: AsRef<OsStr>>(path: P, vhd_type: VhdType) -> Self {
        let wide_path: Vec<u16> = path.as_ref().encode_wide().chain(Some(0)).collect();

        Vhd {
            path: wide_path,
            handle: std::ptr::null_mut(),
            vhd_type,
        }
    }

    /// Mount the [`Vhd`] to a Windows device. Persistent will mount the [`Vhd`] until explicitly
    /// `unmounted` or the Windows system shuts down. Otherwise the mount lives until [`Vhd`] is
    /// dropped (tied to the handle).
    pub fn mount(&mut self, mode: MountMode, persistent: bool) -> Result<char> {
        let storage_type = VIRTUAL_STORAGE_TYPE {
            DeviceId: match self.vhd_type {
                VhdType::Vhd => VIRTUAL_STORAGE_TYPE_DEVICE_VHD,
                VhdType::Vhdx => VIRTUAL_STORAGE_TYPE_DEVICE_VHDX,
            },
            VendorId: VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
        };

        let access = match mode {
            MountMode::ReadOnly => VIRTUAL_DISK_ACCESS_ATTACH_RO,
            MountMode::ReadWrite => VIRTUAL_DISK_ACCESS_ATTACH_RW,
        };

        let open_result = unsafe {
            OpenVirtualDisk(
                &storage_type,
                self.path.as_ptr(),
                access,
                0,
                std::ptr::null(),
                &mut self.handle,
            )
        };

        if open_result != ERROR_SUCCESS {
            return Err(open_result.into());
        };

        let mut flags = 0;

        if matches!(mode, MountMode::ReadOnly) {
            flags |= ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY;
        }

        if persistent {
            flags |= ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME;
        }

        let drives_before = get_drive_letters();

        let attach_result = unsafe {
            AttachVirtualDisk(
                self.handle,
                std::ptr::null_mut(),
                flags,
                0,
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        match attach_result {
            ERROR_SUCCESS => {
                let drives_after = get_drive_letters();

                let new_drive = get_new_drive_letter(&drives_before, &drives_after)
                    .ok_or(Error::MountDriveDetection)?;

                Ok(new_drive)
            }
            e => Err(e.into()),
        }
    }
}

fn get_drive_letters() -> Vec<char> {
    // NOTE: 512 is more than enough
    let mut buffer: [u16; 512] = [0; 512];
    let buffer_len = unsafe { GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr()) };

    // Pre-allocate vector for drive letters (estimate 1 char per 4 wide chars)
    let mut drive_letters = Vec::with_capacity((buffer_len / 4) as usize);

    let mut start = 0;
    for i in 0..buffer_len as usize {
        if buffer[i] == 0 {
            if start < i {
                // Get the first character of the wide string slice and convert it to char
                if let Some(first_char) = std::char::from_u32(buffer[start] as u32) {
                    drive_letters.push(first_char);
                }
            }
            start = i + 1;
        }
    }

    drive_letters
}

fn get_new_drive_letter(before: &[char], after: &[char]) -> Option<char> {
    after
        .iter()
        .find(|&&letter| !before.contains(&letter))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_mount_vhd_read_only() {
        let mut vhd = Vhd::new("file.vhd", VhdType::Vhd);
        let letter = vhd.mount(MountMode::ReadOnly, false).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_only_permanent() {
        let mut vhd = Vhd::new("file.vhd", VhdType::Vhd);
        let letter = vhd.mount(MountMode::ReadOnly, true).unwrap();
        dbg!(&letter);
        drop(vhd);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_write() {
        let mut vhd = Vhd::new("file.vhd", VhdType::Vhd);
        let letter = vhd.mount(MountMode::ReadWrite, false).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_write_permanent() {
        let mut vhd = Vhd::new("file.vhd", VhdType::Vhd);
        let letter = vhd.mount(MountMode::ReadWrite, true).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }
}
