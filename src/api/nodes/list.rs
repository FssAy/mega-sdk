use crate::api::nodes::Node;

use crate::binds::nodes::list::*;
use libc::c_int;

pub struct NodeList {
    pub(crate) ptr: *mut MegaNodeList,
}

impl NodeList {
    pub fn size(&self) -> usize {
        (unsafe { list_size(self.ptr) }) as usize
    }

    pub fn get(&self, i: usize) -> Option<Node> {
        let node_ptr = unsafe { list_get(self.ptr, i as c_int) };
        if node_ptr.is_null() {
            None
        } else {
            Some(Node { ptr: node_ptr })
        }
    }
}
