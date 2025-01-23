use std::io::{self, Write};
use std::{env, fs::File, str::FromStr};

use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

// Get Client
pub async fn get_client() -> Result<RpcClient, ClientError> {
    let rpc: String = env::var("RPC").expect("RPC must be set"); // Get RPC from .env
    let client: RpcClient = RpcClient::new_with_commitment(rpc, CommitmentConfig::confirmed()); // Create RPC Client

    Ok(client)
}
// Classify pubkeys
pub fn classify_pubkeys(addresses: Vec<&str>) -> (Vec<Pubkey>, Vec<Pubkey>) {
    let mut on_curve: Vec<Pubkey> = Vec::new();
    let mut off_curve: Vec<Pubkey> = Vec::new();

    for address in addresses {
        if let Ok(pubkey) = Pubkey::from_str(address) {
            if pubkey.is_on_curve() {
                on_curve.push(pubkey);
            } else {
                off_curve.push(pubkey);
            }
        } else {
            println!("Invalid address: {}", address);
        }
    }

    (on_curve, off_curve)
}
#[tokio::main]
async fn main() {
    let data = "
        4BjHsdV37kUrNxSwDrXoBXhzX2fhrfD6PSxHypQygJiS
        DkRbPvzBhP67LnLtvXoXQuMXH75D56v3aDyUPzRo6163
        ARSCiowxuYnt9asAf6et5kb1FyoE7TE1umcJMacGGfdn
        F861fU2a9qRpgcZmLJ2mqtQP4ejUqmmWbACUXJAS2jWN
        ANE681Y15MMpsxLijKhGtLHg4GmSG9uZ6bXQVaL2RY8K
        DhtN7dTMidPbFJpmU5teu5KwJjEr7cRwJR7hDcBCRhKh
        E6ayDCEyMZuvfxW8SqY4fgZvMEpZvCTct2ojyad6rvDV
        6uT3NwZkcLYoAue3q2dT5DAiAz4TehsvoucLLNHct5nB
        ARSCioMr19K2NbgxPSjravvq9M7TuK82URCXiv6FQboT
        Hpk1u8bsgZYgpzuZEdrrsMuj29dUfMLPemmvaoZLG1RB
        GGbeHjs9uGAUf4YzTuNxCaB6V1CQY5SCLb7aY98mYGyc
        Cr1Yj9z5aDmAhUn9BzMa6zJfLEBycguXfyuw1Lf37mUr
        3GZgQGUNUUJznWRvUi1mUK4A3ihLVbfmSJeuoFMRysb9
        ZNax1hscmBmUcB3KkFywSLNFprmqXxzt2JLd1XVZmBe
        ";

    // Transform the data
    let transformed_data: Vec<&str> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect();

    // Classify addresses
    let (on_curve, off_curve) = classify_pubkeys(transformed_data);

    // Write the classified data to a text file
    let mut file = File::create("classified.txt").expect("Unable to create file");

    writeln!(file, "On Curve:").expect("Unable to write data");
    for line in &on_curve {
        writeln!(file, "{}", line).expect("Unable to write data");
    }

    writeln!(file, "\nOff Curve:").expect("Unable to write data");
    for line in &off_curve {
        writeln!(file, "{}", line).expect("Unable to write data");
    }
}
