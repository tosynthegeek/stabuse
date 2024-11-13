use std::collections::HashMap;

pub struct Network {
    pub chain_id: u8,
    pub name: String,
    pub explorer_url: String,
    pub rpc: String,
    supported_assets: HashMap<String, String>
}
