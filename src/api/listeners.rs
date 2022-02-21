use crate::binds::listeners::{RequestListenerTrigger, TransferListenerTrigger};

macro_rules! addr_of {
    ($target:expr) => {
        ($target as *const ()) as usize
    };
}

macro_rules! create_api_listener {
    ($name:ident, $initialization:tt) => {
        paste::item! {

            pub struct $name {
                pub(crate) ptr: *mut crate::binds::listeners::[< Mega $name >],
            }

            impl $name {
                pub fn new<T: [< $name Trigger >] + 'static>() -> Self {
                    Self {
                        ptr: unsafe {
                            $initialization
                        }
                    }
                }
            }

            impl [< $name Trigger >] for $name {}

            impl From<*mut crate::binds::listeners::[< Mega $name >]> for $name {
                fn from(ptr: *mut crate::binds::listeners::[< Mega $name >]) -> Self {
                    Self {
                        ptr,
                    }
                }
            }

            impl Into<usize> for $name {
                fn into(self) -> usize {
                    self.ptr as usize
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    // ToDo: use Self::new with $name type, but it results in a strange error
                    Self {
                        ptr: std::ptr::null_mut(),
                    }
                }
            }

            impl AsRef<$name> for $name {
                fn as_ref(&self) -> &$name {
                    self
                }
            }
        }
    };
}

create_api_listener!(RequestListener, {
    crate::binds::listeners::init_listener(
        addr_of!(T::on_request_start),
        addr_of!(T::on_request_update),
        addr_of!(T::on_request_finish),
        addr_of!(T::on_request_err),
    )
});

create_api_listener!(TransferListener, {
    crate::binds::listeners::transfer_init_listener(
        addr_of!(T::on_transfer_start),
        addr_of!(T::on_transfer_finish),
        addr_of!(T::on_transfer_update),
        addr_of!(T::on_transfer_temp_error),
        addr_of!(T::on_transfer_data),
    )
});
