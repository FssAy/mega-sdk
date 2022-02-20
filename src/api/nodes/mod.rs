pub mod list;

use crate::binds;
use crate::binds::nodes::MegaNode;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};

pub struct Node {
    pub(crate) ptr: *mut MegaNode,
}

impl Node {
    pub fn is_file(&self) -> bool {
        unsafe { binds::nodes::node_is_file(self.ptr) }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let name = binds::nodes::node_get_name(self.ptr);
            format!("{}", CStr::from_ptr(name).to_string_lossy())
        }
    }
}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        self
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
