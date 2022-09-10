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

/// `LocalJson` errors
#[derive(thiserror::Error, Debug)]
pub enum LocalJsonError {
    /// Serde
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    /// Filesystem
    #[error("{0}")]
    Filesystem(#[from] std::io::Error),
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
    async fn load_json<T, S>(&self, file_name: S) -> Result<Option<T>, LocalJsonError>
    where
        T: DeserializeOwned,
        S: AsRef<str>,
    {
        let path = self.location.with_file_name(file_name.as_ref());
        let raw = tokio::fs::read(path).await;
        match raw {
            Ok(raw) => Ok(serde_json::from_slice(&raw)?),
            Err(e) => {
                if e.kind() == tokio::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[async_trait]
impl MetadataGenerator for LocalJson {
    type Error = LocalJsonError;

    async fn metadata_for(&self, token_id: U256) -> Result<Option<NftMetadata>, Self::Error> {
        self.load_json(format!("{}.json", token_id)).await
    }

    async fn contract_metadata(&self) -> Option<ContractMetadata> {
        self.load_json("contract.json").await.ok().flatten()
    }
}
