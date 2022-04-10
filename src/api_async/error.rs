use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::mem::transmute;
use crate::binds::error::{error_code, error_to_string, ErrorCode, MegaError};


#[derive(Debug)]
pub struct Error {
    #[deprecated] pub(crate) ptr: *mut MegaError,
    code: ErrorCode,
    msg: String,
}

impl Error {
    pub fn get_code(&self) -> ErrorCode {
        self.code
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.msg
        )
    }
}

impl From<*mut MegaError> for Error {
    fn from(ptr: *mut MegaError) -> Self {
        let msg = unsafe { CStr::from_ptr(error_to_string(ptr)) };
        let code = unsafe { transmute(error_code(ptr)) };

        Self {
            ptr,
            code,
            msg: format!("{}", msg.to_string_lossy()),
        }
    }
}

impl std::error::Error for Error {

}
