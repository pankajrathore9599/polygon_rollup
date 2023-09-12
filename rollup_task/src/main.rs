pub mod aggregator;
pub mod client;
pub mod shared;
pub mod main_chain;  

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify 'aggregator', 'client', or 'main_chain' to run.");
        return Ok(());
    }

    match args[1].as_str() {
        "aggregator" => aggregator::start_aggregator().await,
        "client" => client::start_client().await,
        "main_chain" => main_chain::start_main_chain().await,
        _ => {
            println!("Invalid argument. Please specify 'aggregator', 'client', or 'main_chain'.");
            Ok(())
        }
    }
}
