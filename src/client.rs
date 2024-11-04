use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

// Get Client
pub async fn get_client() -> Result<RpcClient, ClientError> {
    let rpc: String = env::var("RPC").expect("RPC must be set"); // Get RPC from .env
    let client: RpcClient = RpcClient::new_with_commitment(rpc, CommitmentConfig::confirmed()); // Create RPC Client

    Ok(client)
}
