pub mod auction;
pub mod chain;

pub use auction::AuctionRegistry;
pub use chain::ChainRegistry;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{AuctionInfo, ChainId, ChainInfo};
use crate::utils::errors::RegistryError;

pub struct RegistryService {
    auction_registry: Arc<RwLock<AuctionRegistry>>,
    chain_registry: Arc<RwLock<ChainRegistry>>,
}

impl RegistryService {
    pub fn new(
        auction_registry: Arc<RwLock<AuctionRegistry>>,
        chain_registry: Arc<RwLock<ChainRegistry>>,
    ) -> Self {
        RegistryService {
            auction_registry,
            chain_registry,
        }
    }

    pub async fn create_registry() -> (Arc<RwLock<AuctionRegistry>>, Arc<RwLock<ChainRegistry>>) {
        let chain_registry = Arc::new(RwLock::new(ChainRegistry::new()));
        let auction_registry = Arc::new(RwLock::new(AuctionRegistry::new(&chain_registry).await));

        (auction_registry, chain_registry)
    }

    pub fn get_auction_registry(&self) -> Arc<RwLock<AuctionRegistry>> {
        self.auction_registry.clone()
    }

    pub async fn get_chain_ids(&self) -> Vec<ChainId> {
        let chain_registry = self.chain_registry.read().await;
        chain_registry.get_chain_ids()
    }

    pub async fn register_chain(
        &self,
        chain_id: ChainId,
        chain_info: ChainInfo,
    ) -> Result<(), RegistryError> {
        {
            let mut guard = self.chain_registry.write().await;
            guard.register_chain(chain_id, chain_info)?;
        }

        {
            let mut auction_registry = self.auction_registry.write().await;
            auction_registry.register_chain(chain_id)
        }
    }

    pub async fn submit_auction_info(
        &self,
        auction_info: AuctionInfo,
    ) -> Result<(), RegistryError> {
        // Validate
        self.validate_auction_info(&auction_info).await?;

        // Store
        let mut auction_registry = self.auction_registry.write().await;
        auction_registry.store_auction_info(auction_info)
    }

    pub async fn validate_auction_info(
        &self,
        auction_info: &AuctionInfo,
    ) -> Result<(), RegistryError> {
        let chain_registry = self.chain_registry.read().await;

        if !chain_registry.validate_chain_id(auction_info.chain_id) {
            return Err(RegistryError::InvalidChainId(auction_info.chain_id));
        }

        if !chain_registry.is_valid_seller(auction_info.chain_id, &auction_info.seller_address) {
            return Err(RegistryError::SellerNotRegistered(
                auction_info.seller_address.clone(),
            ));
        }

        Ok(())
    }
}
