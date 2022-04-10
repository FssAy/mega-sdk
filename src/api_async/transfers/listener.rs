use std::alloc::{dealloc, Layout};
use std::mem;
use std::pin::Pin;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use libc::{c_char, malloc};
use winapi::shared::minwindef::FALSE;
use winapi::um::winbase::IsBadReadPtr;
use crate::{addr_of_func, binds};
use crate::api_async::transfers::reader::{TransferListenerMem, TransferReader};
use crate::api_async::transfers::Transfer;
use crate::binds::listeners::MegaTransferListener;
use crate::binds::{MegaApi, TransferListenerTrigger};
use crate::binds::error::MegaError;
use crate::binds::transfer::MegaTransfer;

type Sender = mpsc::Sender<Transfer>;
type Receiver = mpsc::Receiver<Transfer>;
type TransferListenerBytes = [u8; std::mem::size_of::<TransferListenerMem>()];
const TRANSFER_LISTENER_EMPTY: TransferListenerBytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub struct TransferListener {
    pub(crate) reader_active: AtomicBool,
    pub(crate) sender: Sender,
    pub(crate) receiver: Receiver,
    pub(crate) ptr: *mut MegaTransferListener
}

impl TransferListener {
    pub fn new(buffer: usize) -> TransferReader {
        let (s, r) = mpsc::channel(buffer);

        let listener = unsafe {
            let l = malloc(mem::size_of::<Self>()) as *mut Self;

            std::ptr::write(l, Self {
                reader_active: AtomicBool::new(true),
                sender: s,
                receiver: r,
                ptr: null_mut()
            });

            l
        };

        // let self_ptr = (&*listener) as *const TransferListener;

        let ptr: *mut MegaTransferListener = unsafe {
            binds::transfer::transfer_init_listener(
                listener as *mut _,
                0,
                addr_of_func!(Self::on_transfer_finish),
                addr_of_func!(Self::on_transfer_update),
                0,
                0,
            )
        };

        unsafe { (*listener).ptr = ptr }
        TransferReader::new(listener)
    }

    pub(crate) fn is_zeroed<T>(ptr: *mut T) -> bool {
        if ptr.is_null() {
            return true;
        }

        unsafe { std::ptr::read(ptr as *mut TransferListenerBytes) == TRANSFER_LISTENER_EMPTY }
    }
}

impl TransferListenerTrigger for TransferListener {
    // single
    /// !!!!!BLOCKS!!!!!
    unsafe extern "C" fn on_transfer_finish(self_ptr: *mut (), _: *mut MegaApi, mega_transfer: *mut MegaTransfer, _: *mut MegaError) {
        let self_ptr = self_ptr as usize;
        let mega_transfer = mega_transfer as usize;

            let transfer = Transfer::new(mega_transfer as *mut _);
            // println!("Finish {:?}", transfer);
            let self_ptr = self_ptr as *mut TransferListener;
            if (*self_ptr).sender.capacity() > 0 {
                (*self_ptr).sender.blocking_send(transfer).ok();
            }
            (*self_ptr).receiver.close();

        dealloc(self_ptr as *mut _, Layout::from_size_align_unchecked(mem::size_of::<TransferListener>(), mem::align_of::<TransferListener>()));
    }

    // multiple
    unsafe extern "C" fn on_transfer_update(self_ptr: *mut (), _: *mut MegaApi, mega_transfer: *mut MegaTransfer) {
        // println!("ADDR: {:X?}", self_ptr);
        // println!("size: {}", std::mem::size_of::<TransferListenerMem>());
        // println!("val: {:?}", std::ptr::read(self_ptr as *mut [u8; std::mem::size_of::<TransferListenerMem>()]));

        let self_ptr = self_ptr as usize;
        let mega_transfer = mega_transfer as usize;

        std::thread::spawn(move || {
            let transfer = Transfer::new(mega_transfer as *mut _);
            // println!("{:?}", transfer);
            let self_ptr = self_ptr as *mut TransferListener;
            if (*self_ptr).sender.capacity() > 1 {
                (*self_ptr).sender.blocking_send(transfer).ok();
            }
        });
    }
}

impl Drop for TransferListener {
    fn drop(&mut self) {
        println!("Listener dropped!");
    }
}

unsafe impl Send for TransferListener {}
