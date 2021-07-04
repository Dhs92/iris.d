use std::net::IpAddr;

use bitflags::bitflags;
use dns_lookup::lookup_addr;

bitflags! {
    struct Modes: u8 {
        const INVISIBLE = 0b0000_0001;
        const NOTICES = 0b0000_0010;
        const WALL_OP = 0b0000_0100;
        const OP = 0b0000_1000;
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LocalClient {
    nick: String,
    addr: IpAddr,
    host: String,
    mode: Modes,
}

impl From<LocalClient> for User {
    fn from(client: LocalClient) -> Self {
        User {
            nick: client.nick,
            addr: client.addr,
            host: client.host,
            mode: client.mode,
            registered_nicks: Vec::new(),
        }
    }
}

impl LocalClient {
    pub fn new(nick: &str, addr: IpAddr) -> Self {
        let host = lookup_addr(&addr).unwrap(); // TODO handle error
        let nick = nick.into();

        Self {
            nick,
            addr,
            host,
            mode: Modes::empty(),
        }
    }

    pub fn promote(self) -> User {
        self.into()
    }
}

pub struct User {
    nick: String,
    addr: IpAddr,
    host: String,
    mode: Modes,
    registered_nicks: Vec<String>,
}

pub struct LocalServer {
    addr: IpAddr,
    host: String,
}
