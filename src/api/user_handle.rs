#![allow(dead_code)]

use libc::c_char;

/// Unused
pub struct UserHandle {
    ptr: *mut c_char,
}

impl UserHandle {
    pub fn new(p_handle: *mut c_char) -> Self {
        Self { ptr: p_handle }
    }
}
