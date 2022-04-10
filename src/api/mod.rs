mod builder;
mod listeners;
mod nodes;
mod request;
mod user_handle;

use crate::api::nodes::list::NodeList;
use crate::api::nodes::Node;
use crate::binds;
use crate::binds::{LoggedStatus, MegaApi};
pub use builder::ApiBuilder;
use libc::c_char;
pub use listeners::{RequestListener, TransferListener};
pub use request::Request;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr::null_mut;

/// Wrapper over MegaApi class
pub struct Api {
    ptr: *mut binds::MegaApi,
}

#[allow(dead_code)]
impl Api {
    /// Checks if a pointer to the MegaApi is valid
    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }

    /// mega::MegaApi::isLoggedIn
    pub fn login_status(&self) -> LoggedStatus {
        unsafe { transmute::<_, LoggedStatus>(binds::is_logged(self.ptr)) }
    }

    /// mega::MegaApi::fetchNodes
    pub fn fetch_nodes(&self, request_listener: &RequestListener) {
        unsafe { binds::fetch_nodes(self.ptr, request_listener.ptr) }
    }

    /// mega::MegaApi::getMyUserHandle
    fn get_handle(&self) -> Option<*mut c_char> {
        let p_handle = unsafe { binds::get_handle(self.ptr) };

        if p_handle.is_null() {
            None
        } else {
            Some(p_handle)
        }
    }

    /// mega::MegaApi::getNodeByPath
    pub fn get_node(&self, path: impl ToString, base_node: Option<&Node>) -> Node {
        let path = CString::new(path.to_string()).unwrap();
        let base_node = base_node.map_or(self.get_root_node().ptr, |node| node.ptr);

        Node {
            ptr: unsafe { binds::get_node(self.ptr, path.as_ptr(), base_node) },
        }
    }

    /// mega::MegaApi::getRootNode
    pub fn get_root_node(&self) -> Node {
        Node {
            ptr: unsafe { binds::get_root_node(self.ptr) },
        }
    }

    /// mega::MegaApi::getChildren
    pub fn get_children(&self, root: &Node) -> NodeList {
        NodeList {
            ptr: unsafe { binds::get_children(self.ptr, root.ptr) },
        }
    }

    /// mega::MegaApi::createFolder
    pub fn create_folder(
        &self,
        name: impl AsRef<str>,
        parent: impl AsRef<Node>,
        request_listener: impl AsRef<RequestListener>,
    ) {
        let name = CString::new(name.as_ref()).unwrap();

        unsafe {
            binds::api_create_folder(
                self.ptr,
                name.as_ptr(),
                parent.as_ref().ptr,
                request_listener.as_ref().ptr,
            );
        }
    }

    /// mega::MegaApi::startUpload
    pub fn upload(
        &self,
        path: impl AsRef<str>,
        name: impl AsRef<str>,
        parent: &Node,
        transfer_listener: &TransferListener,
    ) {
        let path = CString::new(path.as_ref()).unwrap();
        let name = CString::new(name.as_ref()).unwrap();

        unsafe {
            binds::api_upload(
                self.ptr,
                path.as_ptr(),
                parent.ptr,
                name.as_ptr(),
                transfer_listener.ptr,
            );
        }
    }

    /// mega::MegaApi::startDownloadWithData
    pub fn download(
        &self,
        path: impl AsRef<str>,
        app_data: impl AsRef<str>,
        node: &Node,
        transfer_listener: &TransferListener,
    ) {
        let path = CString::new(path.as_ref()).unwrap();
        let app_data = CString::new(app_data.as_ref()).unwrap();

        unsafe {
            binds::api_download(
                self.ptr,
                node.ptr,
                path.as_ptr(),
                app_data.as_ptr(),
                transfer_listener.ptr,
            );
        }
    }
}

impl From<*mut binds::MegaApi> for Api {
    fn from(ptr: *mut MegaApi) -> Self {
        Self { ptr }
    }
}

/// TEMPORARY
unsafe impl Sync for Api {}
