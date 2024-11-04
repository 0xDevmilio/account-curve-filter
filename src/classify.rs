use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

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
