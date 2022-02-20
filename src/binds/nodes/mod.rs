use libc::{c_char, c_void};

pub(crate) mod list;

pub(crate) type MegaNode = c_void;

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn node_is_file(node: *mut MegaNode) -> bool;

    pub(crate) fn node_get_name(node: *mut MegaNode) -> *const c_char;
}
