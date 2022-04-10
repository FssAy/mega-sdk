use super::error::MegaError;
use super::MegaApi;
use libc::{c_char, size_t, uint64_t};
use std::ffi::c_void;
use bucket::Bucket;
use tokio::sync::oneshot;
use crate::api_async::requests::RequestOutput;

pub(crate) type MegaRequestListener = c_void;

pub(crate) type MegaRequest = c_void;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RequestType {
    Login = 0,
    CreateFolder = 1,
    Move = 2,
    Copy = 3,
    Rename = 4,
    Remove = 5,
    Share = 6,
    ImportLink = 7,
    Export = 8,
    FetchNodes = 9,
    AccountDetails = 10,
    ChangePw = 11,
    Upload = 12,
    Logout = 13,
    GetPublicNode = 14,
    GetAttrFile = 15,
    SetAttrFile = 16,
    GetAttrUser = 17,
    SetAttrUser = 18,
    RetryPendingConnections = 19,
    RemoveContact = 20,
    CreateAccount = 21,
    ConfirmAccount = 22,
    QuerySignupLink = 23,
    AddSync = 24,
    RemoveSync = 25,
    DisableSync = 26,
    EnableSync = 27,
    CopySyncConfig = 28,
    CopyCachedStatus = 29,
    ImportSyncConfigs = 30,
    RemoveSyncs = 31,
    PauseTransfers = 32,
    CancelTransfer = 33,
    CancelTransfers = 34,
    Delete = 35,
    ReportEvent = 36,
    CancelAttrFile = 37,
    GetPricing = 38,
    GetPaymentId = 39,
    GetUserData = 40,
    LoadBalancing = 41,
    KillSession = 42,
    SubmitPurchaseReceipt = 43,
    CreditCardStore = 44,
    UpgradeAccount = 45,
    CreditCardQuerySubscriptions = 46,
    CreditCardCancelSubscriptions = 47,
    GetSessionTransferUrl = 48,
    GetPaymentMethods = 49,
    InviteContact = 50,
    ReplyContactRequest = 51,
    SubmitFeedback = 52,
    SendEvent = 53,
    CleanRubbishBin = 54,
    SetAttrNode = 55,
    ChatCreate = 56,
    ChatFetch = 57,
    ChatInvite = 58,
    ChatRemove = 59,
    ChatUrl = 60,
    ChatGrantAccess = 61,
    ChatRemoveAccess = 62,
    UseHttpsOnly = 63,
    SetProxy = 64,
    GetRecoveryLink = 65,
    QueryRecoveryLink = 66,
    ConfirmRecoveryLink = 67,
    GetCancelLink = 68,
    ConfirmCancelLink = 69,
    GetChangeEmailLink = 70,
    ConfirmChangeEmailLink = 71,
    ChatUpdatePermissions = 72,
    ChatTruncate = 73,
    ChatSetTitle = 74,
    SetMaxConnections = 75,
    PauseTransfer = 76,
    MoveTransfer = 77,
    ChatPresenceUrl = 78,
    RegisterPushNotification = 79,
    GetUserEmail = 80,
    AppVersion = 81,
    GetLocalSslCert = 82,
    SendSignupLink = 83,
    QueryDns = 84,
    #[deprecated]
    QueryGelb = 85, // (obsolete)
    ChatStats = 86,
    DownloadFile = 87,
    QueryTransferQuota = 88,
    PasswordLink = 89,
    GetAchievements = 90,
    Restore = 91,
    RemoveVersions = 92,
    ChatArchive = 93,
    WhyAmIBlocked = 94,
    ContactLinkCreate = 95,
    ContactLinkQuery = 96,
    ContactLinkDelete = 97,
    FolderInfo = 98,
    RichLink = 99,
    KeepMeAlive = 100,
    MultiFactorAuthCheck = 101,
    MultiFactorAuthGet = 102,
    MultiFactorAuthSet = 103,
    AddScheduledCopy = 104,
    RemoveScheduledCopy = 105,
    Timer = 106,
    AbortCurrentScheduledCopy = 107,
    GetPsa = 108,
    FetchTimezone = 109,
    UseralertAcknowledge = 110,
    ChatLinkHandle = 111,
    ChatLinkUrl = 112,
    SetPrivateMode = 113,
    AutojoinPublicChat = 114,
    Catchup = 115,
    PublicLinkInformation = 116,
    GetBackgroundUploadUrl = 117,
    CompleteBackgroundUpload = 118,
    GetCloudStorageUsed = 119,
    SendSmsVerificationcode = 120,
    CheckSmsVerificationcode = 121,
    GetRegisteredContacts = 122,
    GetCountryCallingCodes = 123,
    VerifyCredentials = 124,
    GetMiscFlags = 125,
    ResendVerificationEmail = 126,
    SupportTicket = 127,
    SetRetentionTime = 128,
    ResetSmsVerifiedNumber = 129,
    SendDevCommand = 130,
    GetBanners = 131,
    DismissBanner = 132,
    BackupPut = 133,
    BackupRemove = 134,
    BackupPutHeartBeat = 135,
    #[deprecated]
    FetchGoogleAds = 136, // deprecated
    #[deprecated]
    QueryGoogleAds = 137, // deprecated
    GetAttrNode = 138,
    LoadExternalDriveBackups = 139,
    CloseExternalDriveBackups = 140,
    GetDownloadUrls = 141,
    StartChatCall = 142,
    JoinChatCall = 143,
    EndChatCall = 144,
    TotalOfRequestTypes = 145,
}

pub trait RequestListenerTrigger {
    unsafe extern "C" fn on_request_start(
        _: *mut Self,
        _: *mut MegaRequestListener,
        _: *mut MegaApi,
        _: *mut MegaRequest,
    ) {
    }

    unsafe extern "C" fn on_request_finish(
        _: *mut Bucket<oneshot::Sender<RequestOutput>>,
        _: *mut MegaRequestListener,
        _: *mut MegaApi,
        _: *mut MegaRequest,
        _: *mut MegaError,
    ) {
    }

    unsafe extern "C" fn on_request_update(
        _: *mut Self,
        _: *mut MegaRequestListener,
        _: *mut MegaApi,
        _: *mut MegaRequest,
    ) {
    }

    unsafe extern "C" fn on_request_err(
        _: *mut Self,
        _: *mut MegaRequestListener,
        _: *mut MegaApi,
        _: *mut MegaRequest,
        _: *mut MegaError,
    ) {
    }
}

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn req_get_type(request: *mut MegaRequest) -> RequestType;

    pub(crate) fn req_to_string(request: *mut MegaRequest) -> *const c_char;

    pub(crate) fn request_get_node_handle(request: *mut MegaRequest) -> u64;

    pub(crate) fn request_init_listener(
        slf: *mut (),
        ors: size_t,
        oru: size_t,
        orf: size_t,
        ore: size_t,
    ) -> *mut MegaRequestListener;

    pub(crate) fn request_size() -> size_t;
}
