use crate::api::listeners::RequestListener;
use crate::api::Api;
use crate::binds;

use std::ffi::CString;
use std::ptr::null;

macro_rules! set_add {
    ( $( $attrib:tt ),* ) => {
        $(
            paste::item! {
                /// Panics if argument implementing `ToString` trait contains byte 0x00.
                pub fn [< set_ $attrib >](mut self, $attrib: impl ToString) -> Self {
                    self.$attrib = Some(CString::new($attrib.to_string()).unwrap());
                    self
                }

                /// Panics if argument implementing `ToString` trait contains byte 0x00.
                pub fn [< add_ $attrib >](&mut self, $attrib: impl ToString) {
                    self.$attrib = Some(CString::new($attrib.to_string()).unwrap());
                }
            }
        )*
    };
}

pub struct ApiBuilder {
    app_key: CString,
    base_path: Option<CString>,
    user_agent: Option<CString>,
}

impl ApiBuilder {
    pub fn new(app_key: impl ToString) -> Self {
        Self {
            app_key: CString::new(app_key.to_string()).unwrap(),
            base_path: None,
            user_agent: None,
        }
    }

    set_add!(base_path, user_agent);

    pub fn login(
        self,
        email: impl AsRef<str>,
        password: impl AsRef<str>,
        request_listener: &RequestListener,
    ) -> Api {
        let email = CString::new(email.as_ref()).unwrap();
        let password = CString::new(password.as_ref()).unwrap();
        let base_path = self
            .base_path
            .as_ref()
            .map_or(null(), |base_path| base_path.as_ptr());
        let user_agent = self
            .user_agent
            .as_ref()
            .map_or(null(), |user_agent| user_agent.as_ptr());

        Api {
            ptr: unsafe {
                binds::login(
                    request_listener.ptr,
                    self.app_key.as_ptr(),
                    email.as_ptr(),
                    password.as_ptr(),
                    base_path,
                    user_agent,
                )
            },
        }
    }
}
