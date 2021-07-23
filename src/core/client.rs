use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::Result as IoResult,
    net::IpAddr,
};

use bitflags::bitflags;
use dns_lookup::lookup_addr;
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

bitflags! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct Modes: u8 {
        const INVISIBLE = 0b0000_0001;
        const NOTICES = 0b0000_0010;
        const WALL_OP = 0b0000_0100;
        const OP = 0b0000_1000;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Client {
    Unregistered(UserData),
    RegisteredUser(UserData, Vec<String>),
}

impl Client {
    pub fn promote(self) -> Self {
        match self {
            Self::Unregistered(data) => Self::RegisteredUser(data, Vec::new()),
            _ => {
                warn!("Promote called on registered user");
                self
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    user: String,
    #[serde(skip)]
    nick: String,
    #[serde(skip)]
    real_name: String,
    #[serde(skip)]
    hostname: String,
    #[serde(skip)]
    mode: Modes,
}

impl Display for UserData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}!{}@{}", self.nick, self.user, self.hostname)
    }
}

impl UserData {
    pub fn with(
        nick: &str,
        user: &str,
        real_name: &str,
        modes: u8,
        addr: IpAddr,
    ) -> IoResult<Self> {
        let hostname = lookup_addr(&addr)?;
        let nick = nick.into();
        let user = user.into();
        let real_name = real_name.into();
        let mode = match Modes::from_bits(modes) {
            Some(mode) => mode,
            None => {
                // change to logging in the future
                error!("Invalid mode: {:#X}, using default: 0x0", modes);
                Modes::empty()
            }
        };

        let user_data = Self {
            nick,
            user,
            real_name,
            hostname,
            mode,
        };

        debug!("Creating LocalClient: {}", user_data);

        Ok(user_data)
    }
}

#[derive(Debug)]
pub struct Connection {
    user: Client,
    conn: TcpStream,
}
