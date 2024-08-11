#![allow(dead_code)]

use error::Error;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::raw::HANDLE;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use windows_sys::Win32::Storage::FileSystem::GetLogicalDriveStringsW;
use windows_sys::Win32::Storage::Vhd::{
    AttachVirtualDisk, DetachVirtualDisk, GetVirtualDiskInformation, OpenVirtualDisk,
    ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME, ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY,
    DETACH_VIRTUAL_DISK_FLAG_NONE, GET_VIRTUAL_DISK_INFO, GET_VIRTUAL_DISK_INFO_0,
    GET_VIRTUAL_DISK_INFO_0_3, GET_VIRTUAL_DISK_INFO_SIZE, VIRTUAL_DISK_ACCESS_ATTACH_RO,
    VIRTUAL_DISK_ACCESS_ATTACH_RW, VIRTUAL_DISK_ACCESS_GET_INFO, VIRTUAL_STORAGE_TYPE,
    VIRTUAL_STORAGE_TYPE_DEVICE_VHD, VIRTUAL_STORAGE_TYPE_DEVICE_VHDX,
    VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
};

pub use error::Result;

mod error;

#[derive(Debug)]
pub struct Vhd {
    path: Vec<u16>,
    handle: HANDLE,
    vhd_type: VhdType,
    mode: OpenMode,
}

#[derive(Debug, Copy, Clone)]
pub enum OpenMode {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug)]
pub enum VhdType {
    Vhd,
    Vhdx,
}

impl Vhd {
    /// Opens a VHD/VHDX file in either `ReadOnly` or `ReadWrite` mode. This method does not
    /// automatically attach the file. The VHD type is inferred from the file extension unless
    /// `force_type` is explicitly specified.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the VHD/VHDX file.
    /// - `open_mode`: Specifies the mode in which to open the file (`ReadOnly` or `ReadWrite`).
    /// - `force_type`: An optional parameter to explicitly set the VHD type, overriding the inferred type.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Vhd` instance on success, or an error if the file
    /// could not be opened.
    pub fn open<P: AsRef<OsStr>>(
        path: P,
        open_mode: OpenMode,
        force_type: Option<VhdType>,
    ) -> Result<Self> {
        let wide_path: Vec<u16> = path.as_ref().encode_wide().chain(Some(0)).collect();

        let vhd_type = match force_type {
            Some(s) => Ok(s),
            None => {
                let ext = std::path::Path::new(&path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_lowercase());

                match ext.as_deref() {
                    Some("vhd") => Ok(VhdType::Vhd),
                    Some("vhdx") => Ok(VhdType::Vhdx),
                    _ => Err(Error::UnknownFileExtension),
                }
            }
        }?;

        let mut handle = null_mut();

        let storage_type = VIRTUAL_STORAGE_TYPE {
            DeviceId: match vhd_type {
                VhdType::Vhd => VIRTUAL_STORAGE_TYPE_DEVICE_VHD,
                VhdType::Vhdx => VIRTUAL_STORAGE_TYPE_DEVICE_VHDX,
            },
            VendorId: VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
        };

        let mut access = match open_mode {
            OpenMode::ReadOnly => VIRTUAL_DISK_ACCESS_ATTACH_RO,
            OpenMode::ReadWrite => VIRTUAL_DISK_ACCESS_ATTACH_RW,
        };

        access |= VIRTUAL_DISK_ACCESS_GET_INFO;

        let open_result = unsafe {
            OpenVirtualDisk(
                &storage_type,
                wide_path.as_ptr(),
                access,
                0,
                std::ptr::null(),
                &mut handle,
            )
        };

        if open_result != ERROR_SUCCESS {
            return Err(open_result.into());
        };

        Ok(Vhd {
            path: wide_path,
            handle,
            vhd_type,
            mode: open_mode,
        })
    }

    /// Mounts the given [`Vhd`] to a Windows device.
    ///
    /// If `persistent` is set to `true`, the [`Vhd`] will remain mounted until it is explicitly
    /// unmounted or the Windows system is shut down. If `persistent` is `false`, the mount will last
    /// only as long as the [`Vhd`] is not dropped.
    ///
    /// # Returns
    ///
    /// A `char` representing the device letter where the [`Vhd`] was successfully mounted.
    pub fn attach(&mut self, persistent: bool) -> Result<char> {
        let mut flags = 0;

        if matches!(self.mode, OpenMode::ReadOnly) {
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

    /// Detaches the previously attached [`Vhd`].
    ///
    /// This function will return an `ERROR_NOT_READY` if an attempt is made to detach a VHD that
    /// has not been attached. Manual detachment is only necessary if the VHD was attached in
    /// persistent mode. Otherwise, the [`Vhd`] will be automatically detached when it is dropped.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing the error encountered during detachment.
    pub fn detach(&mut self) -> Result<()> {
        let result = unsafe { DetachVirtualDisk(self.handle, DETACH_VIRTUAL_DISK_FLAG_NONE, 0) };

        if result != ERROR_SUCCESS {
            return Err(result.into());
        }

        Ok(())
    }

    /// Retrieves the size information of the [`Vhd`],
    /// including `VirtualSize` (u64), `PhysicalSize` (u64), `BlockSize` (u32), and `SectorSize` (u32).
    ///
    /// # Returns
    ///
    /// A `Result` containing a `GET_VIRTUAL_DISK_INFO_0_3` struct with the size details on success,
    /// or an error if the information could not be retrieved.
    pub fn get_size(&mut self) -> Result<GET_VIRTUAL_DISK_INFO_0_3> {
        let mut info = GET_VIRTUAL_DISK_INFO {
            Version: GET_VIRTUAL_DISK_INFO_SIZE,
            Anonymous: GET_VIRTUAL_DISK_INFO_0 {
                Size: GET_VIRTUAL_DISK_INFO_0_3 {
                    VirtualSize: 0,
                    PhysicalSize: 0,
                    BlockSize: 0,
                    SectorSize: 0,
                },
            },
        };

        let mut info_size = std::mem::size_of::<GET_VIRTUAL_DISK_INFO>() as u32;

        let result = unsafe {
            GetVirtualDiskInformation(self.handle, &mut info_size, &mut info, std::ptr::null_mut())
        };

        if result != ERROR_SUCCESS {
            return Err(result.into());
        }

        unsafe { Ok(info.Anonymous.Size) }
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

    #[ignore]
    #[test]
    fn attach() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let attach_result = vhd.attach(true);
        dbg!(&attach_result);
        assert!(attach_result.is_ok());
    }

    #[ignore]
    #[test]
    fn detach() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let result = vhd.detach();
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn info() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let info = vhd.get_size();
        match info {
            Ok(info) => {
                println!("Virtual Size: {}", info.VirtualSize);
                println!("Physical Size: {}", info.PhysicalSize);
                println!("Block Size: {}", info.BlockSize);
                println!("Sector Size: {}", info.SectorSize);
            }
            Err(e) => {
                dbg!(e);
                panic!();
            }
        }
    }

    #[test]
    fn test_mount_vhd_read_only() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let letter = vhd.attach(false).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_only_permanent() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let letter = vhd.attach(true).unwrap();
        dbg!(&letter);
        drop(vhd);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_write() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadWrite, None).unwrap();
        let letter = vhd.attach(false).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn test_mount_vhd_read_write_permanent() {
        let mut vhd = Vhd::open("file.vhd", OpenMode::ReadWrite, None).unwrap();
        let letter = vhd.attach(true).unwrap();
        dbg!(&letter);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
    }
}
