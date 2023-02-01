#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    // pub page: i64,
    // pub total_pages: i64,
    // pub items_on_page: i64,
    pub address: String,
    // pub balance: String,
    // pub total_received: String,
    // pub total_sent: String,
    // pub unconfirmed_balance: String,
    // pub unconfirmed_txs: i64,
    // pub txs: i64,
    pub txids: Vec<String>
}
