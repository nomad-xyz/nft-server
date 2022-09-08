use std::path::PathBuf;

use async_trait::async_trait;
use ethers::types::U256;
use eyre::Result;
use serde::de::DeserializeOwned;

use crate::{open_sea::ContractMetadata, MetadataGenerator, NftMetadata};

/// A `MetadataGenerator` that consults stored JSON files in the local
/// filesystem.
///
/// ## Notes
///
/// Files are stored at `contract.json` for contract-level metadata, and
/// `{token-id}.json` for tokens, where `token-id` is the string representation
/// of the decimal token id. e.g. `0.json`, `384510.json`, etc
#[derive(Debug, Clone)]
pub struct LocalJson {
    location: PathBuf,
}

impl LocalJson {
    /// Instantiate a `LocalJson` metadata generator. Creates directories up to
    /// the specified path
    ///
    /// # Errors
    ///
    /// - If the location exists and is not a directory
    /// - If the directory can't be created
    pub fn new(location: PathBuf) -> Result<Self> {
        eyre::ensure!(
            !location.exists() || location.is_dir(),
            "location exists and is not a directory"
        );
        std::fs::create_dir_all(&location)?;
        Ok(Self { location })
    }

    /// Load JSON from a specific file
    async fn load_json<T, S>(&self, file_name: S) -> Result<T>
    where
        T: DeserializeOwned,
        S: AsRef<str>,
    {
        let path = self.location.with_file_name(file_name.as_ref());
        let raw = tokio::fs::read(path).await?;
        Ok(serde_json::from_slice(&raw)?)
    }
}

#[async_trait]
impl MetadataGenerator for LocalJson {
    async fn metadata_for(&self, token_id: U256) -> Option<NftMetadata> {
        self.load_json(format!("{}.json", token_id)).await.ok()
    }

    async fn contract_metadata(&self) -> Option<ContractMetadata> {
        self.load_json("contract.json").await.ok()
    }
}
