pub(crate) mod error;
pub(crate) mod nodes;
pub(crate) mod request;
pub(crate) mod transfer;
pub(crate) mod listeners {
    pub use super::request::RequestListenerTrigger;
    pub(crate) use super::request::{init_listener, MegaRequestListener};
    pub use super::transfer::TransferListenerTrigger;
    pub(crate) use super::transfer::{transfer_init_listener, MegaTransferListener};
}

pub use listeners::{RequestListenerTrigger, TransferListenerTrigger};

use libc::{c_char, c_int, c_void};
use listeners::{MegaRequestListener, MegaTransferListener};
use nodes::{list::MegaNodeList, MegaNode};

pub(crate) type MegaApi = c_void;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LoggedStatus {
    NotLoggedIn = 0,
    EphemeralAccount = 1,
    ConfirmedAccount = 2,
    FullAccount = 3,
    EphemeralAccountPlusPlus = 4,
}

#[allow(dead_code)]
#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    #[deprecated]
    pub(crate) fn test();

    pub(crate) fn login(
        listener: *mut MegaRequestListener,
        app_key: *const c_char,
        email: *const c_char,
        password: *const c_char,
        base_path: *const c_char,
        user_agent: *const c_char,
    ) -> *mut MegaApi;

    pub(crate) fn is_logged(api: *mut MegaApi) -> c_int;

    pub(crate) fn get_handle(api: *mut MegaApi) -> *mut c_char;

    pub(crate) fn get_root_node(api: *mut MegaApi) -> *mut MegaNode;

    pub(crate) fn get_children(api: *mut MegaApi, root: *mut MegaNode) -> *mut MegaNodeList;

    pub(crate) fn get_node(
        api: *mut MegaApi,
        path: *const c_char,
        n: *mut MegaNode,
    ) -> *mut MegaNode;

    pub(crate) fn fetch_nodes(api: *mut MegaApi, listener: *mut MegaRequestListener);

    pub(crate) fn api_create_folder(
        api: *mut MegaApi,
        name: *const c_char,
        parent: *mut MegaNode,
        listener: *mut MegaRequestListener,
    );

    pub(crate) fn api_upload(
        api: *mut MegaApi,
        local_path: *const c_char,
        parent: *mut MegaNode,
        file_name: *const c_char,
        listener: *mut MegaTransferListener,
    );

    pub(crate) fn api_download(
        api: *mut MegaApi,
        node: *mut MegaNode,
        local_path: *const c_char,
        app_data: *const c_char,
        listener: *mut MegaTransferListener,
    );
}
