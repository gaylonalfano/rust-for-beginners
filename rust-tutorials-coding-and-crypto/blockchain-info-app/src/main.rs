// REF: https://github.com/Coding-and-Crypto/Rust-Beginner-Tutorial/tree/main/blockchain-info-app
// NOTE Use #[macro_use] to bring in EXTERNAL crate (serde_json in bc_info.rs)

use crate::bc_info::blockchain_transaction_request;
#[macro_use]
extern crate serde_derive; // Now we can use these macros to do deserialization

// NOTE Use 'mod' to bring in MY other modules
// This tells 'main' to include all these other files
mod bc_address;
mod bc_info;
mod bc_status;
mod bc_transaction;

// Tell main to access/use our structs
use {
    crate::bc_address::BlockchainAddress,
    crate::bc_status::BlockchainStatus,
    crate::bc_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time}, // For sleep, etc.
};


fn process_blockchain_info(address: &str) {
    let blockchain_status: BlockchainStatus = bc_info::blockhain_status_request();
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    );
    
    let blockchain_address: BlockchainAddress =
        bc_info::blockhain_address_request(address);
    println!("\n\nAnalyzing txs for Bitcoin address: {} ", &blockchain_address.address);

    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);

    println!("\nYou have a total of {} txs.", &blockchain_address.txids.len());

    println!("\nDo you want to query these txs? (y/n)\n");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Invalid command.");

    if command.trim().eq("y") {

        println!("Querying txs...");
        thread::sleep(sleep_time);

        // This represents the TOTAL balance
        let mut balance: i32 = 0;

        // NOTE Can print Vectors using {:#?} print syntax
        // println!("{:#?}", &blockchain_address.txids);
        for tx_id in &blockchain_address.txids {

            // NOTE vin (satoshis OUT) and vout (satoshis IN). It's swapped!
            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

            // NOTE A single tx has MULTIPLE items. Need to filter on wallet address!
            println!("Fetching txid: {}", &tx_id);
            let blockchain_transaction: BlockchainTransaction = blockchain_transaction_request(&tx_id);
            // println!("\nVin {:#?}", &blockchain_transaction.vin);
            // println!("\nVout {:#?}", &blockchain_transaction.vout);
            
            // Match on wallet argument value
            // NOTE Since this is the last use of 'address', can pass ownership
            // and essentially drop its memory usage (destroy it)
            // However, it won't get destroyed until this loop finishes!
            let match_address = String::from(address);

            // Now loop through the single tx_id's VIN items
            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_address) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }

            // Now loop through the single tx_id's VOUT items
            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }

            balance += &subtotal_vout - &subtotal_vin;

            println!("-----------------------------------------------------");
            println!("TX ID:           {}", &blockchain_transaction.txid);
            println!("SATOSHIS IN:     {}", &subtotal_vout);
            println!("SATOSHIS OUT:    {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("-----------------------------------------------------");
        };


        println!("CURRENT BALANCE IN SATOSHIS:     {}", &balance);
        println!("                     IN BTC:     {}\n\n", balance as f32 * 0.00000001);

    }

}

fn main() {
    let wallet_address = dotenv::var("BTC_WALLET").expect("Address not found!");
    // println!("Wallet address: {}", &wallet_address);
    process_blockchain_info(&wallet_address);


}
