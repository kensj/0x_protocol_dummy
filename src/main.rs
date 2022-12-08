use std::time::Duration;
use ethers::utils::Ganache;
use ethers_contract::ContractFactory;
use ethers_providers::Provider;
use ethers_solc::Solc;

use eyre::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create Ganache Instance & Provider
    let ganache = Ganache::new()
        .mnemonic("a b c d e f g h i j k l")
        .spawn();
    let provider = Provider::try_from(ganache.endpoint())
        .expect("Error creating provider")
        .interval(Duration::from_millis(10));

    // Compile & Get Contract
    let compiled = Solc::default()
        .compile_source("./contracts/ZeroProtocol.sol")
        .expect("Could not compile source");

    let contract = compiled
        .get("./contracts/ZeroProtocol.sol", "ZeroProtocol")
        .expect("could not find contract");


    // Deploy Contract
    let factory = ContractFactory::new(
        contract.abi.expect("Error fetching ABI").clone(),
        contract.bytecode().expect("Error fetching bytecode").clone(),
        std::sync::Arc::new(provider),
    )
        .deploy("initial value".to_string())?
        .confirmations(0usize)
        .send()
        .await?;

    Ok(())
}
