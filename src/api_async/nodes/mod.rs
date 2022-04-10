pub mod list;

use crate::binds;
use crate::binds::nodes::MegaNode;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};

pub struct Node {
    pub(crate) ptr: *mut MegaNode,
    is_file: bool,
    name: String,
}

impl Node {
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
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

impl From<*mut MegaNode> for Node {
    fn from(pointer: *mut MegaNode) -> Self {
        unsafe {
            let name = CStr::from_ptr(binds::nodes::node_get_name(pointer));

            Self {
                ptr: pointer,
                is_file: binds::nodes::node_is_file(pointer),
                name: format!("{}", (name).to_string_lossy()),
            }
        }
    }
}
