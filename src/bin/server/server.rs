use tokio::net::{TcpListener, TcpStream};
use vessels::core::config::settings;
use vessels::core::error::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to a peer
    let listener = TcpListener::bind("127.0.0.1:8089").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let conf = settings {
            version: 1,
            password: "12345678901234567890123".to_string(),
            timeout: 1000,
        };
        vessels::core::stream::handle_server(stream, addr, &conf).await;
    }
}
