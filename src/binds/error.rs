use libc::{c_char, c_int, c_void};

// Unused
pub type MegaError = c_void;

/// mega::MegaError
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorCode {
    ApiOk = 0,
    ApiEinternal = -1,
    ApiEargs = -2,
    ApiEagain = -3,
    ApiEratelimit = -4,
    ApiEfailed = -5,
    ApiEtoomany = -6,
    ApiErange = -7,
    ApiEexpired = -8,
    ApiEnoent = -9,
    ApiEcircular = -10,
    ApiEaccess = -11,
    ApiEexist = -12,
    ApiEincomplete = -13,
    ApiEkey = -14,
    ApiEsid = -15,
    ApiEblocked = -16,
    ApiEoverquota = -17,
    ApiEtempunavail = -18,
    ApiEtoomanyconnections = -19,
    ApiEwrite = -20,
    ApiEread = -21,
    ApiEappkey = -22,
    ApiEssl = -23,

    PaymentEcard = -101,
    PaymentEbilling = -102,
    PaymentEfraud = -103,
    PaymentEtoomany = -104,
    PaymentEbalance = -105,
    PaymentEgeneric = -106
}

#[link(name = "mega-dll", kind = "dylib")]
extern "C" {
    pub(crate) fn error_to_string(error: *mut MegaError) -> *const c_char;

    pub(crate) fn error_code(error: *mut MegaError) -> c_int;
}
