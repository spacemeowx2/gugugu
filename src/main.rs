use async_std::net::UdpSocket;
use std::io::Result;

#[async_std::main]
async fn main() -> Result<()> {
    println!("adfasdf");
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = vec![0u8; 1024];

    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        socket.send_to(&buf[..n], &peer).await?;
    }
    Ok(())
}
