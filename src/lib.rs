use error::Error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::ops::Deref;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::raw::HANDLE;
use std::ptr::null_mut;
use uuid::Uuid;
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_SUCCESS, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::GetLogicalDriveStringsW;
use windows_sys::Win32::Storage::Vhd::{
    AttachVirtualDisk, DetachVirtualDisk, GetVirtualDiskInformation, OpenVirtualDisk,
    ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME, ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY,
    DETACH_VIRTUAL_DISK_FLAG_NONE, GET_VIRTUAL_DISK_INFO, GET_VIRTUAL_DISK_INFO_0,
    GET_VIRTUAL_DISK_INFO_0_3, GET_VIRTUAL_DISK_INFO_IDENTIFIER, GET_VIRTUAL_DISK_INFO_SIZE,
    VIRTUAL_DISK_ACCESS_ATTACH_RO, VIRTUAL_DISK_ACCESS_ATTACH_RW, VIRTUAL_DISK_ACCESS_DETACH,
    VIRTUAL_DISK_ACCESS_GET_INFO, VIRTUAL_STORAGE_TYPE, VIRTUAL_STORAGE_TYPE_DEVICE_VHD,
    VIRTUAL_STORAGE_TYPE_DEVICE_VHDX, VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT,
};

pub use error::Result;

mod error;

#[derive(Debug)]
pub struct Vhd {
    handle: HANDLE,
    mode: OpenMode,
}

impl Drop for Vhd {
    fn drop(&mut self) {
        if !self.handle.is_null() && self.handle != INVALID_HANDLE_VALUE {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiskInfo {
    pub virtual_size: u64,
    pub physical_size: u64,
    pub block_size: u32,
    pub sector_size: u32,
}

impl From<GET_VIRTUAL_DISK_INFO_0_3> for DiskInfo {
    fn from(value: GET_VIRTUAL_DISK_INFO_0_3) -> Self {
        Self {
            virtual_size: value.VirtualSize,
            physical_size: value.PhysicalSize,
            block_size: value.BlockSize,
            sector_size: value.SectorSize,
        }
    }
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

#[derive(Debug)]
pub struct VhdIdentifier(Uuid);

impl Deref for VhdIdentifier {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for VhdIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<GUID> for VhdIdentifier {
    fn from(guid: GUID) -> Self {
        let uuid = Uuid::from_fields(guid.data1, guid.data2, guid.data3, &guid.data4);
        VhdIdentifier(uuid)
    }
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
    pub fn new<P: AsRef<OsStr>>(
        path: P,
        open_mode: OpenMode,
        force_type: Option<VhdType>,
    ) -> Result<Self> {
        let mut access_flags = match open_mode {
            OpenMode::ReadOnly => VIRTUAL_DISK_ACCESS_ATTACH_RO,
            OpenMode::ReadWrite => VIRTUAL_DISK_ACCESS_ATTACH_RW,
        };

        access_flags |= VIRTUAL_DISK_ACCESS_GET_INFO;

        Self::open(path, open_mode, force_type, access_flags)
    }

    fn open<P: AsRef<OsStr>>(
        path: P,
        open_mode: OpenMode,
        force_type: Option<VhdType>,
        access_flags: i32,
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

        let open_result = unsafe {
            OpenVirtualDisk(
                &storage_type,
                wide_path.as_ptr(),
                access_flags,
                0,
                std::ptr::null(),
                &mut handle,
            )
        };

        if open_result != ERROR_SUCCESS {
            return Err(open_result.into());
        };

        Ok(Vhd {
            handle,
            mode: open_mode,
        })
    }

    /// Mounts the given [`Vhd`] to a Windows device.
    ///
    /// If `persistent` is set to `true`, the [`Vhd`] will remain mounted until it is explicitly
    /// unmounted or the Windows system is shut down. If `persistent` is `false`, the mount will
    /// last until the [`Vhd`] is dropped.
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

    /// Detaches a VHD specified by `path`.
    ///
    /// This function will return an `ERROR_NOT_READY` if an attempt is made to detach a VHD that
    /// has not been attached. Manual detachment is only necessary if the VHD was attached in
    /// persistent mode. Otherwise, the [`Vhd`] will be automatically detached when it is dropped.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing the error encountered during detachment.
    pub fn detach<P: AsRef<OsStr>>(path: P) -> Result<()> {
        let inner_vhd = Self::open(path, OpenMode::ReadOnly, None, VIRTUAL_DISK_ACCESS_DETACH)?;
        let result =
            unsafe { DetachVirtualDisk(inner_vhd.handle, DETACH_VIRTUAL_DISK_FLAG_NONE, 0) };
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
    pub fn get_size(&mut self) -> Result<DiskInfo> {
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

        unsafe { Ok(info.Anonymous.Size.into()) }
    }

    /// Retrieves the unique identifier (`VhdIdentifier`) of the attached [`Vhd`].
    ///
    /// This method returns a `VhdIdentifier` that uniquely identifies the virtual disk.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `VhdIdentifier` of the virtual disk on success, or an error if the identifier could not be retrieved.
    ///
    pub fn get_identifier(&mut self) -> Result<VhdIdentifier> {
        let mut info = GET_VIRTUAL_DISK_INFO {
            Version: GET_VIRTUAL_DISK_INFO_IDENTIFIER,
            Anonymous: GET_VIRTUAL_DISK_INFO_0 {
                Identifier: GUID {
                    data1: 0,
                    data2: 0,
                    data3: 0,
                    data4: [0; 8],
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

        let identifier: VhdIdentifier = unsafe { info.Anonymous.Identifier.into() };
        Ok(identifier)
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
    use std::thread::sleep;
    use std::time::Duration;

    // NOTE: Adding an initial sleep to ensure Windows has enough time to stabilize before running
    // the tests, as they execute too quickly, even in single-threaded mode.

    #[test]
    fn get_size() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let info = vhd.get_size();
        dbg!(&info);
    }

    #[test]
    fn get_identifier() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let info = vhd.get_identifier().unwrap();
        dbg!(&info);
    }

    #[test]
    fn mount_vhd_read_only_temporary() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let letter = vhd.attach(false).unwrap();
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
        drop(vhd);
        assert!(!Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn mount_vhd_read_write_temporary() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadWrite, None).unwrap();
        let letter = vhd.attach(false).unwrap();
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
        drop(vhd);
        assert!(!Path::new(&format!(r"{letter}:\")).is_dir());
    }

    #[test]
    fn mount_vhd_read_only_permanent() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let letter = vhd.attach(true).unwrap();
        drop(vhd);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
        Vhd::detach("file.vhd").unwrap();
    }

    #[test]
    fn mount_vhd_read_write_permanent() {
        sleep(Duration::from_secs(1));
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadWrite, None).unwrap();
        let letter = vhd.attach(true).unwrap();
        drop(vhd);
        assert!(Path::new(&format!(r"{letter}:\")).is_dir());
        Vhd::detach("file.vhd").unwrap();
    }

    // for manual testing
    #[ignore]
    #[test]
    fn attach() {
        let mut vhd = Vhd::new("file.vhd", OpenMode::ReadOnly, None).unwrap();
        let attach_result = vhd.attach(true);
        dbg!(&attach_result);
        assert!(attach_result.is_ok());
    }

    // for manual testing
    #[ignore]
    #[test]
    fn detach() {
        let result = Vhd::detach("file.vhd");
        dbg!(&result);
        assert!(result.is_ok());
    }
}
