use libc;

use std::fmt;
use std::str::FromStr;

#[cfg(any(target_os = "android", target_os = "linux"))]
mod linux;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use self::linux::*;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
mod unix;
#[cfg(any(target_os = "macos", target_os = "freebsd"))]
pub use self::unix::*;

use crate::Value;

#[derive(Debug, Clone, Copy)]
pub struct Mib {
    pub(crate) inner: [libc::c_int; CTL_MAXNAME],
    pub(crate) len: usize,
}

impl Mib {
    #[inline]
    pub fn new() -> Self {
        Mib {
            inner: [0; CTL_MAXNAME],
            len: CTL_MAXNAME,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn bytes_len(&self) -> usize {
        self.len * std::mem::size_of::<libc::c_int>()
    }

    #[inline]
    pub fn components(&self) -> &[libc::c_int] {
        &self.inner[..self.len]
    }

    #[inline]
    pub fn name(&self) -> Result<String, std::io::Error> {
        mib2name(self)
    }

    /// Get Value
    #[inline]
    pub fn value(&self) -> Result<Value, std::io::Error> {
        get(self)
    }

    /// Set Value
    #[inline]
    pub fn set_value(&self, val: Value) -> Result<Value, std::io::Error> {
        update(self, val)
    }

    #[inline]
    pub fn metadata(&self) -> Result<Metadata, std::io::Error> {
        mib2metadata(self)
    }

    /// Only available on FreeBSD system.
    #[inline]
    pub fn description(&self) -> Result<String, std::io::Error> {
        mib2desc(self)
    }

    #[inline]
    pub fn push(&mut self, component: libc::c_int) {
        if self.len >= CTL_MAXNAME {
            return ();
        }

        self.inner[self.len] = component;
        self.len += 1;
    }

    #[inline]
    pub fn replace(&mut self, offset: usize, val: libc::c_int) {
        if offset < self.len {
            self.inner[offset] = val;
        }
    }

    #[inline]
    pub fn extend(&mut self, other: &Self) {
        &mut self.inner[self.len..self.len + other.len()].copy_from_slice(other.components());
        self.len += other.len();
    }

    #[inline]
    pub fn as_ptr(&self) -> *const libc::c_int {
        (&self.inner[..self.len]).as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut libc::c_int {
        (&mut self.inner[..self.len]).as_mut_ptr()
    }
}

impl From<[libc::c_int; CTL_MAXNAME]> for Mib {
    fn from(buffer: [libc::c_int; CTL_MAXNAME]) -> Self {
        let mut len = CTL_MAXNAME;
        while len > 0 {
            if buffer[len - 1] == 0 {
                len -= 1;
            } else {
                break;
            }
        }
        Mib {
            inner: buffer,
            len: len,
        }
    }
}

impl From<&[libc::c_int]> for Mib {
    fn from(val: &[libc::c_int]) -> Self {
        let mut len = val.len();
        assert!(len <= CTL_MAXNAME);

        let mut buffer: [libc::c_int; CTL_MAXNAME] = [0; CTL_MAXNAME];

        while len > 0 {
            if val[len - 1] == 0 {
                len -= 1;
            } else {
                break;
            }
        }

        for idx in 0..len {
            buffer[idx] = val[idx];
        }

        Mib {
            inner: buffer,
            len: len,
        }
    }
}

impl FromStr for Mib {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        name2mib(s)
    }
}

impl fmt::Display for Mib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.components())
    }
}

impl Iterator for Mib {
    type Item = Result<Mib, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match next_mib(self) {
            Ok(mib) => {
                self.inner = mib.inner;
                self.len = mib.len;

                Some(Ok(mib))
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    None
                } else {
                    Some(Err(e))
                }
            }
        }
    }
}

impl Default for Mib {
    fn default() -> Self {
        let mut buffer: [libc::c_int; CTL_MAXNAME] = [0; CTL_MAXNAME];
        buffer[0] = CTL_KERN;
        let len = 1;

        Mib {
            inner: buffer,
            len: len,
        }
    }
}
