//! Faucet controller entrypoint.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("I'm faucet!");

    Ok(())
}
