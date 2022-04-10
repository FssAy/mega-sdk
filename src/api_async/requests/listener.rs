use std::borrow::Borrow;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::null_mut;
use std::sync::Arc;
use crate::binds::{MegaApi, RequestListenerTrigger};
use lazy_static::lazy_static;
use libc::bind;
use tokio::sync::{mpsc, oneshot};
use crate::api_async::requests::{Request, RequestOutput};
use crate::{addr_of_func, binds};
use crate::binds::error::{ErrorCode, MegaError};
use crate::binds::listeners::MegaRequestListener;
use crate::binds::request::MegaRequest;
use bucket::Bucket;
use tokio::io::AsyncRead;
use crate::api_async::error::Error;

pub struct RequestListener {
    pub(crate) ptr: *mut MegaRequestListener,
    pub(crate) sender: Pin<Box<Bucket<oneshot::Sender<RequestOutput>>>>,
    pub(crate) receiver: oneshot::Receiver<RequestOutput>,
}

impl RequestListener {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::oneshot::channel();

        let bucket = Box::new(Bucket::new(sender));
        let bucket_ptr = (&*bucket) as *const Bucket<oneshot::Sender<RequestOutput>>;

        let mut slf = Self {
            ptr: null_mut(),
            sender: Pin::new(bucket),
            receiver
        };

        unsafe {
            slf.ptr = binds::request::request_init_listener(
                // (&mut slf) as *mut Self as *mut _,
                bucket_ptr as *mut _,
                0,
                0,
                addr_of_func!(Self::on_request_finish),
                0
            );
        }

        slf
    }

    pub async fn receive(mut self) -> RequestOutput {
        (self.receiver).await.unwrap()
    }
}

impl RequestListenerTrigger for RequestListener {
    unsafe extern "C" fn on_request_finish(slf: *mut Bucket<oneshot::Sender<RequestOutput>>, _: *mut MegaRequestListener, _: *mut MegaApi, request: *mut MegaRequest, error: *mut MegaError) {
        let error = Error::from(error);
        let sender = (*slf).vacate().unwrap();

        if error.get_code() == ErrorCode::ApiOk {
            sender.send(Ok(Request::from(request)));
        } else {
            sender.send(Err(error));
        }
    }
}
