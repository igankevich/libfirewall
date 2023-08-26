use std::mem::transmute;

use libc::addrinfo;
use libc::c_char;
use libc::c_int;
use libc::c_void;
use libc::dlsym;
use libc::size_t;
use libc::sockaddr;
use libc::socklen_t;
use libc::ssize_t;
use libc::RTLD_NEXT;

macro_rules! get_symbol {
    ($name:expr) => {
        unsafe { transmute(dlsym(RTLD_NEXT as *mut c_void, $name.as_ptr() as *const i8)) }
    };
}

pub(crate) struct OldFunctions {
    pub(crate) getaddrinfo: unsafe extern "C" fn(
        *const c_char,
        *const c_char,
        *const addrinfo,
        *mut *mut addrinfo,
    ) -> c_int,
    pub(crate) connect: unsafe extern "C" fn(c_int, *const sockaddr, socklen_t) -> c_int,
    pub(crate) sendto: unsafe extern "C" fn(
        c_int,
        *const c_void,
        size_t,
        c_int,
        *const sockaddr,
        socklen_t,
    ) -> ssize_t,
}

impl OldFunctions {
    pub(crate) fn new() -> Self {
        Self {
            getaddrinfo: get_symbol!(b"getaddrinfo\0"),
            connect: get_symbol!(b"connect\0"),
            sendto: get_symbol!(b"sendto\0"),
        }
    }
}
