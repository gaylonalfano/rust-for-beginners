// NOTE Using NOWNodes API: GET https://btcbook.nownodes.io/api/
// Going to use deserialize the response JSON into a matching struct!
// TL;DR: Convert HTTP Response JSON into Rust structs

// NOTE We can comment out/omit fields from JSON and Rust will still deserialize
// NOTE #[derive(Deserialize)] annotation to pipe a JSON response from a request
// into a Rust struct
// NOTE #[serde(rename_all = "camelCase")] annotation to recognize JSON naming convention
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    // pub host: String,
    // pub version: String,
    // pub git_commit: String,
    // pub build_time: String,
    // pub sync_mode: bool,
    // pub initial_sync: bool,
    // pub in_sync: bool,
    // pub best_height: i64,
    // pub last_block_time: String,
    // pub in_sync_mempool: bool,
    // pub last_mempool_time: String,
    // pub mempool_size: i64,
    // pub decimals: i64,
    // pub db_size: i64,
    // pub about: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,
    // pub blocks: i64,
    // pub headers: i64,
    // pub best_block_hash: String,
    // pub difficulty: String,
    // pub size_on_disk: i64,
    // pub version: String,
    // pub subversion: String,
    // pub protocol_version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainStatus {
    pub blockbook: Blockbook,
    pub backend: Backend,
}
