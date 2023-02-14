use tokio::net::TcpListener;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut read, mut write) = socket.split();
            tokio::io::copy(&mut read, &mut write).await.unwrap();
        });
    }
}
