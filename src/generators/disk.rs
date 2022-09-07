use std::path::PathBuf;

use async_trait::async_trait;
use ethers::types::U256;
use eyre::Result;
use serde::de::DeserializeOwned;

use crate::{open_sea::ContractMetadata, MetadataGenerator, NftMetadata};

pub struct LocalJson {
    location: PathBuf,
}

impl LocalJson {
    pub fn from_path(location: PathBuf) -> Result<Self> {
        eyre::ensure!(
            !location.exists() || location.is_dir(),
            "location exists and is not a directory"
        );
        std::fs::create_dir_all(&location)?;
        Ok(Self { location })
    }

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
