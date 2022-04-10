pub mod listener;

use crate::binds;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use crate::api_async::error::Error;
use crate::binds::request::{MegaRequest, RequestType};

pub type RequestOutput = Result<Request, Error>;

#[derive(Debug)]
pub struct Request {
    #[deprecated] pub(crate) ptr: *mut MegaRequest,
    typ: RequestType,
    node_handle: u64,
}

impl Request {
    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }

    pub fn get_type(&self) -> binds::request::RequestType {
        self.typ
    }

    pub fn get_node_handle(&self) -> u64 {
        self.node_handle
    }
}

impl From<*mut binds::request::MegaRequest> for Request {
    fn from(pointer: *mut binds::request::MegaRequest) -> Self {
        unsafe { Self {
            ptr: pointer,
            typ: binds::request::req_get_type(pointer),
            node_handle: binds::request::request_get_node_handle(pointer),
        } }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            let cstr = CStr::from_ptr(binds::request::req_to_string(self.ptr));
            cstr.to_string_lossy()
        })
    }
}
