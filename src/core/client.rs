use std::{io::Error as IoError, net::IpAddr};

use bitflags::bitflags;
use dns_lookup::lookup_addr;
use tokio::net::TcpStream;

bitflags! {
    pub struct Modes: u8 {
        const INVISIBLE = 0b0000_0001;
        const NOTICES = 0b0000_0010;
        const WALL_OP = 0b0000_0100;
        const OP = 0b0000_1000;
    }
}

#[derive(Debug)]
pub struct LocalClient {
    nick: String,
    addr: IpAddr,
    host: String,
    mode: Modes,
    connection: TcpStream,
}

impl From<LocalClient> for User {
    fn from(client: LocalClient) -> Self {
        User {
            nick: client.nick,
            addr: client.addr,
            host: client.host,
            mode: client.mode,
            registered_nicks: Vec::new(),
            connection: client.connection,
        }
    }
}

impl LocalClient {
    pub fn with(nick: &str, modes: u8, connection: TcpStream) -> Result<Self, IoError> {
        let addr = connection.peer_addr()?.ip();
        let host = lookup_addr(&addr)?;
        let nick = nick.into();
        let mode = match Modes::from_bits(modes) {
            Some(mode) => mode,
            None => {
                // change to logging in the future
                eprintln!("Invalid mode: {:#X}, using default: 0x0", modes);
                Modes::empty()
            }
        };

        Ok(Self {
            nick,
            addr,
            host,
            mode,
            connection,
        })
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
    connection: TcpStream,
}

pub struct LocalServer {
    addr: IpAddr,
    host: String,
}
