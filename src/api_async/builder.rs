use std::ffi::CString;
use std::ptr::null;
use crate::api_async::Api;
use crate::api_async::error::Error;
use crate::binds;


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

    pub fn set_base_path(mut self, var: impl ToString) -> Self {
        self.base_path = Some(CString::new(var.to_string()).unwrap());
        self
    }

    pub fn set_user_agent(mut self, var: impl ToString) -> Self {
        self.user_agent = Some(CString::new(var.to_string()).unwrap());
        self
    }

    pub async fn login(self, email: impl AsRef<str>, password: impl AsRef<str>,) -> Result<super::Api, Error> {
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

        let mut listener = super::requests::listener::RequestListener::new();

        let ptr = unsafe {
            binds::login(listener.ptr, self.app_key.as_ptr(),
                         email.as_ptr(),
                         password.as_ptr(),
                         base_path,
                         user_agent)
        };

        let output = listener.receive().await;

        output.map(|_| {
            Api {
                ptr,
            }
        })
    }
}