//! Faucet controller entrypoint.

use clap::Parser;

/// Args to run the faucet controller.
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// Name of the faucet controller.
    #[arg(long)]
    name: String,

    /// Path to the data folder.
    #[arg(long)]
    data_path: String,

    /// Url to the main server.
    #[arg(long)]
    main_server_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    println!("faucet controller is run ");
    println!("Name:\t{}", args.name);
    println!("Data Path:\t{}", args.data_path);
    println!("Main Server Utl:\t{}", args.main_server_url);

    Ok(())
}
