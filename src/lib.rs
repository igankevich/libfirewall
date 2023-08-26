use std::ffi::CStr;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem::transmute;
use std::net::IpAddr;
use std::sync::OnceLock;

use libc::addrinfo;
use libc::c_char;
use libc::c_int;
use libc::c_void;
use libc::close;
use libc::dlsym;
use libc::size_t;
use libc::sockaddr;
use libc::socklen_t;
use libc::ssize_t;
use libc::EAI_NONAME;
use libc::EBADF;
use libc::RTLD_NEXT;
use os_socketaddr::OsSocketAddr;
use regex::Regex;

macro_rules! log {
    ($($args:expr),*) => {
        eprintln!("lockdown: {}", format!($($args),*))
    };
}

type Connect = unsafe extern "C" fn(c_int, *const sockaddr, socklen_t) -> c_int;
type SendTo = unsafe extern "C" fn(
    c_int,
    *const c_void,
    size_t,
    c_int,
    *const sockaddr,
    socklen_t,
) -> ssize_t;
type GetAddrInfo = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    *const addrinfo,
    *mut *mut addrinfo,
) -> c_int;

static CONTEXT: OnceLock<Context> = OnceLock::new();

struct OriginalFunctions {
    getaddrinfo: GetAddrInfo,
    connect: Connect,
    sendto: SendTo,
}

impl OriginalFunctions {
    fn new() -> Self {
        Self {
            getaddrinfo: unsafe {
                transmute(dlsym(
                    RTLD_NEXT as *mut c_void,
                    b"getaddrinfo\0".as_ptr() as *const i8,
                ))
            },
            connect: unsafe {
                transmute(dlsym(
                    RTLD_NEXT as *mut c_void,
                    b"connect\0".as_ptr() as *const i8,
                ))
            },
            sendto: unsafe {
                transmute(dlsym(
                    RTLD_NEXT as *mut c_void,
                    b"sendto\0".as_ptr() as *const i8,
                ))
            },
        }
    }
}

fn get_original_functions() -> &'static OriginalFunctions {
    &Context::get().functions
}

struct AllowedDnsNames {
    patterns: Vec<Regex>,
}

fn glob_to_regex(glob: &str) -> Result<Regex, regex::Error> {
    let mut regex = String::with_capacity(glob.len() * 6);
    regex.push('^');
    for ch in glob.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            ch => write!(regex, "\\u{{{:x}}}", ch as u32).unwrap(),
        }
    }
    regex.push('$');
    Regex::new(regex.as_str())
}

impl AllowedDnsNames {
    fn new() -> Self {
        let mut patterns: Vec<Regex> = Vec::new();
        if let Ok(string) = std::env::var("LOCKDOWN_ALLOW") {
            for pattern in string.split(' ') {
                match glob_to_regex(pattern.trim()) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => {
                        log!("failed to parse glob pattern `{}`: {}", pattern, e);
                    }
                }
            }
        }
        Self { patterns }
    }

    fn contain(&self, name: &str) -> bool {
        for pattern in self.patterns.iter() {
            if pattern.is_match(name) {
                return true;
            }
        }
        false
    }
}

struct AllowedDnsServers {
    servers: Vec<IpAddr>,
}

impl AllowedDnsServers {
    fn new() -> Self {
        let mut servers: Vec<IpAddr> = Vec::new();
        let filepath = "/etc/resolv.conf";
        match File::open(filepath) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            let mut iter = line.trim().split_ascii_whitespace();
                            if let Some(word) = iter.next() {
                                if word == "nameserver" {
                                    if let Some(word) = iter.next() {
                                        match word.parse::<IpAddr>() {
                                            Ok(server) => servers.push(server),
                                            Err(e) => {
                                                log!(
                                                    "failed to parse ip address `{}` from `{}`: {}",
                                                    word,
                                                    filepath,
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log!("failed to read `{}`: {}", filepath, e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                log!("failed to open `{}`: {}", filepath, e);
                log!("please specify allowed dns servers manually");
            }
        };
        Self { servers }
    }

    fn contain(&self, server: IpAddr) -> bool {
        self.servers.contains(&server)
    }
}

struct Context {
    functions: OriginalFunctions,
    allowed_names: AllowedDnsNames,
    allowed_servers: AllowedDnsServers,
}

impl Context {
    fn new() -> Self {
        Self {
            functions: OriginalFunctions::new(),
            allowed_names: AllowedDnsNames::new(),
            allowed_servers: AllowedDnsServers::new(),
        }
    }

    fn get() -> &'static Self {
        CONTEXT.get_or_init(Self::new)
    }
}

fn is_dns_server_allowed(socket: c_int, address: *const sockaddr, len: socklen_t) -> bool {
    if let Some(addr) = unsafe { OsSocketAddr::copy_from_raw(address, len) }.into_addr() {
        if addr.port() == 53 {
            if Context::get().allowed_servers.contain(addr.ip()) {
                log!("allow {}", addr.ip());
            } else {
                log!("BLOCK {}", addr.ip());
                unsafe { close(socket) };
                return false;
            }
        }
    }
    true
}

#[no_mangle]
extern "C" fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int {
    if is_dns_server_allowed(socket, address, len) {
        unsafe { (get_original_functions().connect)(socket, address, len) }
    } else {
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
    if is_dns_server_allowed(socket, addr, addrlen) {
        unsafe { (get_original_functions().sendto)(socket, buf, len, flags, addr, addrlen) }
    } else {
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
                return unsafe {
                    (get_original_functions().getaddrinfo)(node, service, hints, res)
                };
            } else {
                log!("block {}", name);
            }
        }
        Err(e) => {
            log!("BLOCK unknown dns name: {}", e);
        }
    };
    EAI_NONAME
}
