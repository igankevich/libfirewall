use std::ffi::CStr;

use libc::addrinfo;
use libc::c_char;
use libc::c_int;
use libc::c_void;
use libc::close;
use libc::size_t;
use libc::sockaddr;
use libc::socklen_t;
use libc::ssize_t;
use libc::EAI_NONAME;
use libc::EBADF;
use os_socketaddr::OsSocketAddr;

use crate::log;
use crate::Context;
use crate::OldFunctions;

#[no_mangle]
extern "C" fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int {
    if is_dns_server_allowed(address, len) {
        unsafe { (get_old_functions().connect)(socket, address, len) }
    } else {
        unsafe { close(socket) };
        EBADF
    }
}

#[no_mangle]
extern "C" fn sendto(
    socket: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t {
    if is_dns_server_allowed(addr, addrlen) {
        unsafe { (get_old_functions().sendto)(socket, buf, len, flags, addr, addrlen) }
    } else {
        unsafe { close(socket) };
        EBADF as ssize_t
    }
}

#[no_mangle]
extern "C" fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> c_int {
    match unsafe { CStr::from_ptr(node).to_str() } {
        Ok(name) => {
            if Context::get().allowed_names.contain(name) {
                log!("allow {}", name);
                return unsafe { (get_old_functions().getaddrinfo)(node, service, hints, res) };
            } else {
                log!("BLOCK {}", name);
            }
        }
        Err(e) => {
            log!("BLOCK unknown dns name: {}", e);
        }
    };
    EAI_NONAME
}

fn get_old_functions() -> &'static OldFunctions {
    &Context::get().old_functions
}

fn is_dns_server_allowed(address: *const sockaddr, len: socklen_t) -> bool {
    if let Some(addr) = unsafe { OsSocketAddr::copy_from_raw(address, len) }.into_addr() {
        if addr.port() == 53 {
            if Context::get().allowed_servers.contain(addr.ip()) {
                log!("allow {}", addr);
            } else {
                log!("BLOCK {}", addr);
                return false;
            }
        }
    }
    true
}
