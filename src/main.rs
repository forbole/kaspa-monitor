use clap::Parser;
use csv::Reader;
use futures::stream::{self, StreamExt};
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    prelude::{NetworkId, NetworkType},
    KaspaRpcClient, Resolver, WrpcEncoding,
};
use std::{
    path::PathBuf,
    time::{Instant, Duration},
    process::ExitCode,
};
use std::path::Prefix;
use kaspa_addresses::Address;
use kaspa_wrpc_client::prelude::{RpcAddress, RpcApi};
use tracing::{info, warn, error};
use statistical::{mean, standard_deviation};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to CSV file containing Kaspa addresses
    #[arg(long)]
    csv_path: PathBuf,

    /// Kaspa node wRPC endpoint (optional, will use resolver if not provided)
    #[arg(short, long)]
    endpoint: Option<String>,

    /// Number of concurrent requests
    #[arg(short, long, default_value_t = 10)]
    concurrent_requests: usize,
}

#[derive(Debug)]
struct RequestMetrics {
    duration: Duration,
    address: String,
}

#[tokio::main]
async fn main() -> ExitCode {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match run_benchmark(args).await {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error!("Benchmark failed: {}", error);
            ExitCode::FAILURE
        }
    }
}

async fn run_benchmark(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Read addresses from CSV
    let mut rdr = Reader::from_path(&args.csv_path)?;
    let addresses: Vec<String> = rdr
        .records()
        .filter_map(|record| {
            record.ok().and_then(|rec| rec.get(0).map(String::from))
        })
        .collect();

    info!("Loaded {} addresses from CSV", addresses.len());

    // Setup client
    let encoding = WrpcEncoding::Borsh;
    let url = args.endpoint;
    let resolver = Some(Resolver::default());
    let network_type = NetworkType::Mainnet;
    let selected_network = Some(NetworkId::new(network_type));
    let subscription_context = None;

    // Create client
    let client = KaspaRpcClient::new(
        encoding,
        url.as_deref(),
        resolver,
        selected_network,
        subscription_context,
    )?;

    // Connect with options
    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(5_000)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    client.connect(Some(options)).await?;

    // Process addresses in chunks with concurrent requests
    let mut metrics = Vec::new();

    let chunks = addresses.chunks(args.concurrent_requests);
    for chunk in chunks {
        let futures = chunk.iter().map(|address| {
            let client = client.clone();
            let address = address.clone();

            async move {
                let start = Instant::now();
                let result = client.get_balance_by_address(Address::try_from(address.as_str()).unwrap()).await;

                match result {
                    Ok(balance) => {
                        let duration = start.elapsed();
                        info!("Address: {}, Balance: {}", address, balance);
                        Some(RequestMetrics {
                            duration,
                            address,
                        })
                    }
                    Err(e) => {
                        error!("Error getting balance for {}: {}", address, e);
                        None
                    }
                }
            }
        });

        let results: Vec<_> = stream::iter(futures)
            .buffer_unordered(args.concurrent_requests)
            .filter_map(|x| async move { x })
            .collect()
            .await;

        metrics.extend(results);
    }

    // Calculate statistics
    let durations: Vec<f64> = metrics
        .iter()
        .map(|m| m.duration.as_secs_f64())
        .collect();

    let mean_duration = mean(&durations);
    let std_dev = standard_deviation(&durations, Some(mean_duration));
    let min_duration = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_duration = durations.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    println!("\nBenchmark Results:");
    println!("Total Requests: {}", metrics.len());
    println!("Mean Duration: {:.3}s", mean_duration);
    println!("Std Deviation: {:.3}s", std_dev);
    println!("Min Duration: {:.3}s", min_duration);
    println!("Max Duration: {:.3}s", max_duration);

    // Disconnect client
    client.disconnect().await?;

    Ok(())
}

