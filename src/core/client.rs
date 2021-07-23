use std::{fmt::{Display, Formatter, Result as FmtResult}, io::Result as IoResult, net::IpAddr};

use bitflags::bitflags;
use dns_lookup::lookup_addr;
use tokio::net::TcpStream;
use log::{error, debug};
use serde::{Serialize, Deserialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Modes: u8 {
        const INVISIBLE = 0b0000_0001;
        const NOTICES = 0b0000_0010;
        const WALL_OP = 0b0000_0100;
        const OP = 0b0000_1000;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalClient {
    nick: String,
    user: String,
    real_name: String,
    addr: IpAddr,
    host: String,
    mode: Modes,

    // if connection is `None` user will be in invalid state
    #[serde(skip)]
    connection: Option<TcpStream>,
}

impl Display for LocalClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}!{}@{}", self.nick, self.user, self.host)
    }
}

impl From<LocalClient> for User {
    fn from(client: LocalClient) -> Self {
        User {
            client,
            registered_nicks: Vec::new(),
        }
    }
}

impl LocalClient {
    pub fn with(nick: &str, user: &str, real_name: &str, modes: u8, connection: TcpStream) -> IoResult<Self> {
        let addr = connection.peer_addr()?.ip();
        let host = lookup_addr(&addr)?;
        let nick = nick.into();
        let user = user.into();
        let real_name = real_name.into();
        let connection = Some(connection);
        let mode = match Modes::from_bits(modes) {
            Some(mode) => mode,
            None => {
                // change to logging in the future
                error!("Invalid mode: {:#X}, using default: 0x0", modes);
                Modes::empty()
            }
        };

        let local_client = Self {
            nick,
            user,
            real_name,
            addr,
            host,
            mode,
            connection,
        };

        debug!("Creating LocalClient: {}", local_client);

        Ok(local_client)
    }

    pub fn promote(self) -> User {
        debug!("Promoting from LocalUser to User: {}", self);
        self.into()
    }
}

pub struct User {
    client: LocalClient,
    registered_nicks: Vec<String>
}
