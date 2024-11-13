use std::collections::HashMap;

#[derive(Debug)]
pub struct Network {
    pub chain_id: u8,
    pub name: String,
    pub explorer: String,
    pub rpc: String,
    pub supported_assets: HashMap<String, String>
}

impl Network {
    fn new(name: String, rpc: String, chain_id: u64, explorer: String) -> Self {
        Network {
            name,
            rpc,
            chain_id,
            explorer,
            supported_assets: HashMap::new()
        }
    }

    fn add_asset(&mut self, ticker: String, contract_address: String) {
        self.supported_assets.insert(ticker.to_uppercase(), contract_address);
    }

    fn get_asset_address(&self, ticker: &str) -> Option<&String> {
        self.supported_assets.get(&ticker.to_uppercase())
    }

    pub fn get_supported_assets(&self) -> Vec<String> {
        self.supported_assets.keys().cloned().collect()
    }
}