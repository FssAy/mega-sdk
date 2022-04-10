use std::ops::{Deref, DerefMut};
use super::Node;

use crate::binds::nodes::list::*;
use libc::c_int;

pub struct NodeList {
    pub(crate) ptr: *mut MegaNodeList,
    buffer: Vec<Node>,
}

impl NodeList {
    pub(crate) fn new(ptr: *mut MegaNodeList) -> Self {
        let size = (unsafe { list_size(ptr) }) as usize;
        let mut buffer = Vec::with_capacity(size);

        for i in 0..size {
            match NodeList::get_node(ptr, i) {
                None => break,
                Some(node) => buffer.push(node),
            }
        }

        Self {
            ptr,
            buffer,
        }
    }

    pub fn size(&self) -> usize {
        (unsafe { list_size(self.ptr) }) as usize
    }

    fn get_node(ptr: *mut MegaNodeList, i: usize) -> Option<Node> {
        let node_ptr = unsafe { list_get(ptr, i as c_int) };
        if node_ptr.is_null() {
            None
        } else {
            Some(Node::from(node_ptr))
        }
    }
}

impl Deref for NodeList {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for NodeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl Into<Vec<Node>> for NodeList {
    fn into(self) -> Vec<Node> {
        self.buffer
    }
}
