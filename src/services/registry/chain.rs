use std::collections::HashMap;

use crate::domain::{ChainId, ChainInfo};
use crate::utils::errors::RegistryError;

#[derive(Default)]
pub struct ChainRegistry {
    chain_info: HashMap<ChainId, ChainInfo>,
}

impl ChainRegistry {
    pub fn new() -> Self {
        let chain_info = HashMap::new();

        ChainRegistry { chain_info }
    }

    pub fn register_chain(
        &mut self,
        chain_id: ChainId,
        chain_info: ChainInfo,
    ) -> Result<(), RegistryError> {
        if self.chain_info.contains_key(&chain_id) {
            return Err(RegistryError::ChainAlreadyRegistered(chain_id));
        }

        self.chain_info.insert(chain_id, chain_info);
        Ok(())
    }

    pub fn get_chain_ids(&self) -> Vec<ChainId> {
        self.chain_info.keys().cloned().collect()
    }

    /// Checks whether the given chain ID is recognized in our registry.
    pub fn validate_chain_id(&self, chain_id: ChainId) -> bool {
        self.chain_info.contains_key(&chain_id)
    }

    /// Checks if the given seller address is registered for the specified chain.
    pub fn is_valid_seller(&self, chain_id: ChainId, seller: &str) -> bool {
        if let Some(info) = self.chain_info.get(&chain_id) {
            info.registered_sellers.contains(&seller.to_string())
        } else {
            false
        }
    }

    /// Retrieves the maximum gas limit for the specified chain.
    pub fn get_max_gas_limit(&self, chain_id: ChainId) -> Option<u64> {
        self.chain_info.get(&chain_id).map(|info| info.gas_limit)
    }
}
