use std::pin::Pin;
use std::ptr::null_mut;
use std::sync::atomic::Ordering;

use super::Transfer;
use super::listener::TransferListener;

pub(crate) type TransferListenerMem = TransferListener;

pub struct TransferReader {
    pub(crate) listener: *mut TransferListenerMem,
}

impl TransferReader {
    pub(crate) fn new(listener: *mut TransferListenerMem) -> Self {
        Self {
            listener,
        }
    }

    pub async fn recv(&mut self) -> Option<Transfer> {
        unsafe { (*self.listener).receiver.recv().await }
    }

    pub async fn wait(mut self) -> Option<Transfer> {
        unsafe {
            let listener = &mut (*self.listener);

            while let Some(transfer) = listener.receiver.recv().await {
                if transfer.state.is_finished() {
                    return Some(transfer);
                }
            }

            return None;
        }
    }

    pub fn ignore(mut self) {
        drop(self);
    }
}

// impl Drop for TransferReader {
//     fn drop(&mut self) { unsafe {
//         println!("Dropping reader");
//         if !TransferListener::is_zeroed(self.listener) {
//             println!("Dropping reader listener not zeroed");
//             // (*listener).reader_active = false;
//             // std::ptr::write_bytes(self.listener, 0x00, std::mem::size_of::<TransferListenerMem>());
//             (*self.listener).reader_active.store(false, Ordering::SeqCst);
//         }
//         println!("Dropping reader done!");
//
//
//         // if let Some(mut listener) = self.listener.vacate() {
//         //     let listener = unsafe { &mut *listener };
//         //
//         //     if listener.sender.is_closed() {
//         //         return;
//         //     }
//         //
//         //     std::thread::spawn(move || {
//         //         while let Some(transfer) = listener.receiver.blocking_recv() {
//         //             if transfer.state.is_finished() {
//         //                 break;
//         //             }
//         //         }
//         //
//         //         println!("Ignoring reader finished!");
//         //     });
//         // }
//     }  }
// }

unsafe impl Send for TransferReader {}
