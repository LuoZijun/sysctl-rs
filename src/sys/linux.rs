use libc;

use std::io;
use std::ptr;

// largest number of components supported
pub const CTL_MAXNAME: usize = 10;

// Top-level names
pub const CTL_KERN: libc::c_int = 1; // General kernel info and control
pub const CTL_VM: libc::c_int = 2; // VM management
pub const CTL_NET: libc::c_int = 3; // Networking
pub const CTL_PROC: libc::c_int = 4; // removal breaks strace(1) compilation
pub const CTL_FS: libc::c_int = 5; // Filesystems
pub const CTL_DEBUG: libc::c_int = 6; // Debugging
pub const CTL_DEV: libc::c_int = 7; // Devices
pub const CTL_BUS: libc::c_int = 8; // Busses
pub const CTL_ABI: libc::c_int = 9; // Binary emulation
pub const CTL_CPU: libc::c_int = 10; // CPU stuff (speed scaling, etc)
pub const CTL_ARLAN: libc::c_int = 254; // arlan wireless driver
pub const CTL_S390DBF: libc::c_int = 5677; // s390 debug
pub const CTL_SUNRPC: libc::c_int = 7249; // sunrpc debug
pub const CTL_PM: libc::c_int = 9899; // frv power management
pub const CTL_FRV: libc::c_int = 9898; // frv specific sysctls

// TODO: 
// 