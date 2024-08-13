#![allow(clippy::enum_variant_names, non_camel_case_types)]

use thiserror::Error;
use windows_result::{Error as WindowsError, HRESULT};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to detect the mount drive.")]
    MountDriveDetection,

    #[error("Failed to detect file extension.")]
    UnknownFileExtension,

    #[error("{description} ({hex}) ({code})")]
    UnexpectedWindowsCallResult {
        description: String,
        hex: HRESULT,
        code: u32,
    },
}

// unfortunately I do not know which errors are relevant for `OpenVirtualDisk` and `AttachVirtualDisk`
// https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
//
// TODO: add more error codes. these are 0-499
impl From<u32> for Error {
    fn from(value: u32) -> Self {
        // even though not all syscalls support IErrorInfo interface to get a suitable message,
        // it works for all the codes documented here, https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes#system-error-codes
        // internally `windows-result` does step-3-4 on it's ErrorInfo struct in error.rs: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/retrieving-error-information#to-retrieve-error-information
        // two helpful paragraphs here: https://docs.rs/windows-result/0.2.0/windows_result/struct.Error.html#extended-error-info-and-the-windows_slim_errors-configuration-option
        // also! we convert from HRESULT to WindowsError via `.into()` because this calls `ErrorInfo::from_thread()` internally to
        // get the error message
        let err: WindowsError = HRESULT::from_win32(value).into();
        Self::UnexpectedWindowsCallResult {
            description: err.message(),
            hex: err.code(),
            code: value,
        }
    }
}
