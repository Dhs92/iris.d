use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub trait Reset<T: Copy> {
    fn reset(&mut self);
}

impl Reset<u8> for Vec<u8> {
    fn reset(&mut self) {
        for i in self {
            *i = 0;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 512];

            // In a loop, read data from the socket and print.
            loop {
                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(_n) => {
                        match String::from_utf8(buf.clone()) {
                            Ok(message) => print!("{}", message),
                            Err(_) => {
                                let _ = socket.write("Invalid UTF-8 byte sequence".as_bytes());
                                return;
                            }
                        };
                        buf.reset();
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
