use super::error::MegaError;
use super::MegaApi;
use libc::{c_char, c_void, size_t};

// Unused
pub(crate) type MegaTransfer = c_void;

pub(crate) type MegaTransferListener = c_void;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum TransferState {
    None = 0,
    Queued = 1,
    Active = 2,
    Paused = 3,
    Retrying = 4,
    Completing = 5,
    Completed = 6,
    Cancelled = 7,
    Failed = 8,
}

impl From<i32> for TransferState {
    fn from(value: i32) -> Self {
        if value > TransferState::Failed as i32 {
            return TransferState::None;
        }
        unsafe { return std::mem::transmute(value); }
    }
}

impl TransferState {
    pub fn is_finished(&self) -> bool {
        if self >= &Self::Completed {
            true
        } else {
            false
        }
    }
}

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn transfer_init_listener(
        slf: *mut (),
        ots: size_t,
        otf: size_t,
        otu: size_t,
        ote: size_t,
        otd: size_t,
    ) -> *mut MegaTransferListener;

    pub(crate) fn transfer_get_start_time(ptr: *mut MegaTransfer) -> i64;
    pub(crate) fn transfer_get_trans_bytes(ptr: *mut MegaTransfer) -> i64;
    pub(crate) fn transfer_get_total_bytes(ptr: *mut MegaTransfer) -> i64;
    pub(crate) fn transfer_get_speed(ptr: *mut MegaTransfer) -> i64;
    pub(crate) fn transfer_get_state(ptr: *mut MegaTransfer) -> TransferState;
}

pub trait TransferListenerTrigger {
    unsafe extern "C" fn on_transfer_start(_: *mut (), _: *mut MegaApi, _: *mut MegaTransfer) {}

    unsafe extern "C" fn on_transfer_finish(
        _: *mut (),
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut MegaError,
    ) {
    }

    unsafe extern "C" fn on_transfer_update(_: *mut (), _: *mut MegaApi, _: *mut MegaTransfer) {}

    unsafe extern "C" fn on_transfer_temp_error(
        _: *mut (),
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut MegaError,
    ) {
    }

    unsafe extern "C" fn on_transfer_data(
        _: *mut (),
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut c_char,
        _: usize,
    ) -> bool {
        return true;
    }
}
