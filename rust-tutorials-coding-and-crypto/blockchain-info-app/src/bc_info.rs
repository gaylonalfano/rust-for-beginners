// This is where we'll do all the HTTP requests
use dotenv;
use reqwest;
use serde_json::Result; // Does the deserialization of JSON to struct
use tokio;

use crate::bc_status::BlockchainStatus;
use crate::bc_address::BlockchainAddress;
use crate::bc_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

// Create a helper fn for all requests
// NOTE Tokio will handle the request, so need its attribute
#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();

    client
        .get(url) // GET request
        .header(
            "api-key",
            dotenv::var("API_KEY").expect("Could not find key: API_KEY"),
        )
        .send() // Send the request
        .await
        .expect("Failed to get response!")
        .text() // Get the response body (payload) text (NOTE: We handle JSON later)
        .await
        .expect("Failed to get response body (payload) text!")
    // NOTE We don't add ';' at end since it returns automatically
    // i.e., no need to use 'return' keyword
}

pub fn blockhain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    // println!("Blockchain Status response: {}", response); 

    // NOTE Convert/deserialize payload into struct
    // If this passes, then it returns a struct! (don't put ';' at end!)
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockhain_address_request(address: &str) -> BlockchainAddress {
    // https://btcbook.nownodes.io/api/v2/address/bc1q592ee9fjw5wh9j7trcq8avzc0lxmv4q8nlm234
    
    // let response = send_request(&[HOST_ROOT, "v2/address/", &address].concat());
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    // println!("Blockchain Address response: {}", response); 

    // NOTE Convert/deserialize payload into struct
    // If this passes, then it returns a struct! (don't put ';' at end!)
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_transaction_request(txid: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &txid].concat());
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

