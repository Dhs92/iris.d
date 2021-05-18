use std::net::IpAddr;

pub struct LocalClient {
    nick: String,
    addr: IpAddr,
    host: String,
}

impl Into<User> for LocalClient {
    fn into(self) -> User {
        User {
            nick: self.nick,
            addr: self.addr,
            host: self.host,
            registered_nicks: Vec::new(),
        }
    }
}

impl LocalClient {
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
