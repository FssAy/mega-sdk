use super::error::MegaError;
use super::MegaApi;
use libc::{c_char, c_void, size_t};

// Unused
pub(crate) type MegaTransfer = c_void;

pub(crate) type MegaTransferListener = c_void;

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn transfer_init_listener(
        ots: size_t,
        otf: size_t,
        otu: size_t,
        ote: size_t,
        otd: size_t,
    ) -> *mut MegaTransferListener;
}

pub trait TransferListenerTrigger {
    unsafe extern "C" fn on_transfer_start(_: *mut MegaApi, _: *mut MegaTransfer) {}

    unsafe extern "C" fn on_transfer_finish(
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut MegaError,
    ) {
    }

    unsafe extern "C" fn on_transfer_update(_: *mut MegaApi, _: *mut MegaTransfer) {}

    unsafe extern "C" fn on_transfer_temp_error(
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut MegaError,
    ) {
    }

    unsafe extern "C" fn on_transfer_data(
        _: *mut MegaApi,
        _: *mut MegaTransfer,
        _: *mut c_char,
        _: usize,
    ) -> bool {
        return true;
    }
}
