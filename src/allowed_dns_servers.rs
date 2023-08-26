use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::net::IpAddr;

use crate::log;

pub(crate) struct AllowedDnsServers {
    servers: Vec<IpAddr>,
}

impl AllowedDnsServers {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn contain(&self, server: IpAddr) -> bool {
        self.servers.contains(&server)
    }
}
