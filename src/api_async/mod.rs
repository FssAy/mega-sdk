use std::ffi::CString;
use crate::api_async::error::Error;
use crate::api_async::nodes::list::NodeList;
use crate::api_async::nodes::Node;
use crate::api_async::requests::listener::RequestListener;
use crate::api_async::transfers::listener::TransferListener;
use crate::api_async::transfers::reader::TransferReader;
use crate::api_async::transfers::Transfer;
use crate::binds;
use crate::binds::MegaApi;

mod macros;
pub mod builder;
pub mod requests;
pub mod error;
pub mod nodes;
pub mod transfers;

pub struct Api {
    ptr: *mut MegaApi,
}

impl Api {
    fn get_node_by_handle(&self, handle: u64) -> Node {
        Node::from(unsafe { binds::get_node_by_handle(self.ptr, handle) })
    }

    pub fn get_root_node(&self) -> Node {
        Node::from(unsafe { binds::get_root_node(self.ptr) })
    }

    pub fn get_children(&self, root: impl AsRef<Node>) -> NodeList {
        NodeList::new(unsafe { binds::get_children(self.ptr, root.as_ref().ptr) })
    }


    pub async fn fetch_nodes(&self) -> Result<(), Error> {
        let listener = RequestListener::new();
        unsafe { binds::fetch_nodes(self.ptr, listener.ptr); }

        listener.receive().await.map(|_| ())
    }

    pub async fn create_folder(
        &self,
        name: impl AsRef<str>,
        parent: impl AsRef<Node>,
    ) -> Result<Node, Error> {
        let name = CString::new(name.as_ref()).unwrap();
        let listener = RequestListener::new();

        unsafe {
            binds::api_create_folder(
                self.ptr,
                name.as_ptr(),
                parent.as_ref().ptr,
                listener.ptr,
            );
        }

        let output = listener.receive().await;

        output.map(|request| {
            self.get_node_by_handle(request.get_node_handle())
        })
    }

    pub async fn upload(
        &self,
        path: impl AsRef<str>,
        name: impl AsRef<str>,
        parent: impl AsRef<Node>,
    ) -> TransferReader {
        let path = CString::new(path.as_ref()).unwrap();
        let name = CString::new(name.as_ref()).unwrap();

        let mut reader = TransferListener::new(32);

        unsafe {
            binds::api_upload(
                self.ptr,
                path.as_ptr(),
                parent.as_ref().ptr,
                name.as_ptr(),
                (*reader.listener).ptr,
            );
        }

        reader
    }

    pub async fn download(
        &self,
        path: impl AsRef<str>,
        app_data: impl AsRef<str>,
        node: impl AsRef<Node>,
    ) -> TransferReader {
        let path = CString::new(path.as_ref()).unwrap();
        let app_data = CString::new(app_data.as_ref()).unwrap();

        let mut reader = TransferListener::new(32);

        unsafe {
            binds::api_download(
                self.ptr,
                node.as_ref().ptr,
                path.as_ptr(),
                app_data.as_ptr(),
                (*reader.listener).ptr,
            );
        }

        reader
    }
}
