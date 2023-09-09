pub mod server;
pub mod client;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify 'server' or 'client' to run.");
        return Ok(());
    }

    match args[1].as_str() {
        "server" => server::start_server().await,
        "client" => client::start_client().await,
        _ => {
            println!("Invalid argument. Please specify 'server' or 'client'.");
            Ok(())
        }
    }
}
