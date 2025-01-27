use std::time::Instant;
use kaspa_wrpc_client::{KaspaRpcClient, Resolver, WrpcEncoding};
use kaspa_wrpc_client::prelude::{NetworkId, NetworkType};
use colored::Colorize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Test Kaspa WebSocket connection time")]
struct Args {
    #[arg(short, long)]
    endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{}", "\nInitializing connection test...".blue());
    println!("Endpoint: {}", args.endpoint.as_deref().unwrap_or("Using resolver"));

    let start = Instant::now();
    
    let client = KaspaRpcClient::new(
        WrpcEncoding::Borsh,
        args.endpoint.as_deref(),
        Some(Resolver::default()),
        Some(NetworkId::new(NetworkType::Mainnet)),
        None,
    )?;

    println!("Client creation took: {:.3}s", start.elapsed().as_secs_f64());
    
    let connect_start = Instant::now();
    client.connect(None).await?;
    let connect_time = connect_start.elapsed().as_secs_f64();

    println!("{}", "\nConnection Results:".yellow());
    println!("Connection establishment time: {:.3}s", connect_time);
    println!("Total initialization time: {:.3}s", start.elapsed().as_secs_f64());

    client.disconnect().await?;
    Ok(())
}