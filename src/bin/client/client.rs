use tokio::net::{TcpListener, TcpStream};
use vessels::core::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to a peer
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        match vessels::core::stream::handle_client(stream, addr).await {
            Err(err) => {
                println!("handle_client: {}", err)
            }
            _ => {}
        }
    }
}
