use crate::binds;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};

pub struct Request {
    pub(crate) ptr: *mut binds::request::MegaRequest,
}

impl Request {
    pub fn get_type(&self) -> binds::request::RequestType {
        unsafe { binds::request::req_get_type(self.ptr) }
    }
}

impl From<*mut binds::request::MegaRequest> for Request {
    fn from(pointer: *mut binds::request::MegaRequest) -> Self {
        Self { ptr: pointer }
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
