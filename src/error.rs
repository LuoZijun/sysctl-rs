#[derive(Debug, Fail)]
pub enum SysctlError {
    #[fail(display = "no such sysctl: {}", _0)]
    NotFound(String),

    #[fail(display = "no matching type for value")]
    #[cfg(not(target_os = "macos"))]
    UnknownType,

    #[fail(display = "Error extracting value")]
    ExtractionError,

    #[fail(display = "IO Error: {}", _0)]
    IoError(#[cause] std::io::Error),

    #[fail(display = "Error parsing UTF-8 data: {}", _0)]
    Utf8Error(#[cause] std::str::Utf8Error),

    #[fail(display = "Value is not readable")]
    NoReadAccess,

    #[fail(display = "Value is not writeable")]
    NoWriteAccess,

    #[fail(
        display = "sysctl returned a short read: read {} bytes, while a size of {} was reported",
        read, reported
    )]
    ShortRead { read: usize, reported: usize },

    #[fail(display = "Error reading C String: String was not NUL-terminated.")]
    InvalidCStr(#[cause] std::ffi::FromBytesWithNulError),
}

impl From<std::io::Error> for SysctlError {
    fn from(error: std::io::Error) -> Self {
        SysctlError::IoError(error)
    }
}

impl From<std::str::Utf8Error> for SysctlError {
    fn from(error: std::str::Utf8Error) -> Self {
        SysctlError::Utf8Error(error)
    }
}

impl From<std::ffi::FromBytesWithNulError> for SysctlError {
    fn from(error: std::ffi::FromBytesWithNulError) -> Self {
        SysctlError::InvalidCStr(error)
    }
}
