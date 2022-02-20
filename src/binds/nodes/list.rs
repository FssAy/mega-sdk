use super::MegaNode;
use libc::{c_int, c_void};

pub(crate) type MegaNodeList = c_void;

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn list_size(list: *mut MegaNodeList) -> c_int;

    pub(crate) fn list_get(list: *mut MegaNodeList, i: c_int) -> *mut MegaNode;
}
