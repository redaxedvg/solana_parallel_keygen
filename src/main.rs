use solana_sdk::signature::{Keypair, Signer};
use base58::ToBase58;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    // Enter your desired prefix here (e.g., "ExamplePrefix")
    let prefix = "ExamplePrefix";  // Replace "ExamplePrefix" with the desired prefix
    
    let mut generated_count = 0;
    let mut keypair_found = false;

    while !keypair_found {
        let result = (0..100_000)
            .into_par_iter()
            .find_any(|_i| {
                let keypair = Keypair::new();
                let pubkey_base58 = keypair.pubkey().to_bytes().to_base58();
                
                if pubkey_base58.starts_with(prefix) {
                    let private_key_base58 = keypair.secret().to_bytes().to_base58();
                    println!("Found matching public key: {}", pubkey_base58);
                    println!("Private key: {}", private_key_base58);
                    return true;
                }
                false
            });

        if result.is_some() {
            keypair_found = true;
        }

        generated_count += 100_000;

        if generated_count % 100_000 == 0 {
            let elapsed_time = start_time.elapsed().as_secs();
            println!("Generated {} keypairs, Time elapsed: {} seconds", generated_count, elapsed_time);
        }
    }
}
