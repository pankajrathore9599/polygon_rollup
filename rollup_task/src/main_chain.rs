use crate::shared::{Rollup, MainChain};
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use bincode;

pub async fn start_main_chain() -> Result<(), Box<dyn std::error::Error>> {
    println!("Main chain started.");
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let mut main_chain = MainChain { rollups: Vec::new() };

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer = vec![0; 2048];
        let n = socket.read(&mut buffer).await?;
        let rollup: Rollup = bincode::deserialize(&buffer[..n])?;

        if rollup.hash == rollup.compute_hash() {
            main_chain.rollups.push(rollup);
            println!("Rollup added to the main chain. Total rollups: {}", main_chain.rollups.len());
        } else {
            println!("Rollup hash mismatch.");
        }
    }
}
