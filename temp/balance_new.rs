use anyhow::{Context, Result};
use ethers_core::types::Address;
use ethers_providers::{Http, Provider}; //,Middleware};
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {

    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/27abc398cf46416c82784bcfe85ed98f").context("Failed to open Http Provider")?;

    let addr = "0x999E77c988C4C1451d3B1c104a6cca7813A9946E".parse::<Address>()?;

    let balance = provider.get_balance(addr, None).await?;
    
    println!("Got balance: {}", balance);
    Ok(())
}
