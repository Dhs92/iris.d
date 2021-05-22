use std::net::IpAddr;

use dns_lookup::lookup_addr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LocalClient {
    nick: String,
    addr: IpAddr,
    host: String,
}

impl From<LocalClient> for User {
    fn from(client: LocalClient) -> Self {
        User {
            nick: client.nick,
            addr: client.addr,
            host: client.host,
            registered_nicks: Vec::new(),
        }
    }
}

impl LocalClient {
    pub fn new(nick: &str, addr: IpAddr) -> Self {
        let host = lookup_addr(&addr).unwrap(); // TODO handle error
        let nick = nick.into();

        Self { nick, addr, host }
    }

    pub fn promote(self) -> User {
        self.into()
    }
}

pub struct User {
    nick: String,
    addr: IpAddr,
    host: String,
    registered_nicks: Vec<String>,
}

pub struct LocalServer {
    addr: IpAddr,
    host: String,
}
