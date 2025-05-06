use std::time::Instant;
use kaspa_wrpc_client::{KaspaRpcClient, Resolver, WrpcEncoding};
use kaspa_wrpc_client::prelude::{NetworkId, NetworkType, RpcApi};
use colored::Colorize;
use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(author, version, about = "Kaspa Node Benchmarking Tool", long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Kaspa node WebSocket endpoint
    #[arg(short, long, global = true)]
    endpoint: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check node health and sync status
    Healthcheck,
    /// Test connection latency
    Latencycheck,
}

async fn run_healthcheck(endpoint: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\nInitializing connection test...".blue());
    println!("Endpoint: {}", endpoint.unwrap_or("Using resolver"));

    let start = Instant::now();
    
    let client = KaspaRpcClient::new(
        WrpcEncoding::Borsh,
        endpoint,
        Some(Resolver::default()),
        Some(NetworkId::new(NetworkType::Mainnet)),
        None,
    )?;

    println!("Client creation took: {:.3}s", start.elapsed().as_secs_f64());
    
    let connect_start = Instant::now();
    match timeout(Duration::from_secs(5), client.connect(None)).await {
        Ok(connect_result) => {
            connect_result?;
            let connect_time = connect_start.elapsed().as_secs_f64();

            println!("{}", "\nConnection Results:".yellow());
            println!("Connection establishment time: {:.3}s", connect_time);
            println!("Total initialization time: {:.3}s", start.elapsed().as_secs_f64());

            println!("{}", "\nFetching sync status...".yellow());
            let sync_status = client.get_sync_status().await?;
            
            println!("{}", "\nSync Status:".green());
            println!("{:#?}", sync_status);

            Ok(())
        }
        Err(_) => {
            eprintln!("{}", "\nConnection timeout after 5 seconds!".red());
            std::process::exit(1);
        }
    }
}

async fn run_latencycheck(endpoint: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\nInitializing connection test...".blue());
    println!("Endpoint: {}", endpoint.unwrap_or("Using resolver"));

    let start = Instant::now();
    
    let client = KaspaRpcClient::new(
        WrpcEncoding::Borsh,
        endpoint,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Healthcheck => {
            run_healthcheck(args.endpoint.as_deref()).await?;
        }
        Commands::Latencycheck => {
            run_latencycheck(args.endpoint.as_deref()).await?;
        }
    }

    Ok(())
}