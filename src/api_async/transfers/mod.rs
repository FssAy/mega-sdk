pub mod listener;
pub mod reader;

use std::fmt::{Debug, Formatter};
use crate::binds::transfer::{MegaTransfer, transfer_get_speed, transfer_get_start_time, transfer_get_state, transfer_get_total_bytes, transfer_get_trans_bytes, TransferState};


#[derive(Debug, Copy, Clone)]
pub struct Transfer {
    // pub(crate) ptr: *mut MegaTransfer,
    pub(crate) start_time: i64,
    pub(crate) bytes_transferred: i64,
    pub(crate) bytes_total: i64,
    pub(crate) speed: i64,
    pub(crate) state: TransferState,
}

impl Transfer {
    pub fn new(ptr: *mut MegaTransfer) -> Self {
        unsafe {
            Self {
                start_time: transfer_get_start_time(ptr),
                bytes_transferred: transfer_get_trans_bytes(ptr),
                bytes_total: transfer_get_total_bytes(ptr),
                speed: transfer_get_speed(ptr),
                state: transfer_get_state(ptr)
            }
        }
    }
}

