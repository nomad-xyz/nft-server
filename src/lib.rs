//! A simple NFT Json Metadata library, with easy axum-based servers

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

use std::{convert::Infallible, str::FromStr};

use async_trait::async_trait;

use ethers::types::U256;
use serde::{Deserialize, Serialize};
use url::Url;

/// Provided generator implementations
pub mod generators;
/// OpenSea-specific data structures
pub mod open_sea;

#[cfg(feature = "axum")]
/// Servers
pub mod server;

use open_sea::{ContractMetadata, OpenSeaAttribute};

/// Prelude
pub mod prelude {
    #[cfg(feature = "axum")]
    pub use crate::server::{serve_generator, serve_router};
    pub use crate::{open_sea::*, MetadataGenerator, NftImage, NftMetadata};
    pub use ethers::types::U256;
    pub use url::Url;
}

/// An `NftImage` is a URL to an image, or the image data as a string.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NftImage {
    /// Image URL
    Url {
        /// The URL
        image: url::Url,
    },
    /// Image Data
    Data {
        /// The image file as a string
        image_data: String,
    },
}

impl FromStr for NftImage {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().map(Into::into).or_else(|_| Ok(s.into()))
    }
}

impl From<Url> for NftImage {
    fn from(image: Url) -> Self {
        NftImage::Url { image }
    }
}

impl From<&str> for NftImage {
    fn from(image_data: &str) -> Self {
        NftImage::Data {
            image_data: image_data.to_string(),
        }
    }
}

/// Top-level NFT Metadata supporting basic ERC-721 schema, with OpenSea
/// extensions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NftMetadata {
    /// The NFT name
    pub name: String,
    /// The NFT description
    pub description: String,
    /// An external URL related to the NFT
    pub external_url: Url,
    /// The NFT image link or data
    #[serde(flatten)]
    pub image: NftImage,
    /// NFT Attributes, in the OpenSea format
    #[serde(default)]
    pub attributes: Vec<OpenSeaAttribute>,
    /// The background color to be displayed on OpenSea
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// The animation URL to be displayed on OpenSea
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<Url>,
    /// The Youtube link to be displayed on OpenSea
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<Url>,
}

/// A `MetadataGenerator` asynchronously generates token and contract Metadata.
/// Tokens are referenced by the `uint256` tokenId used to identify them in the
/// ERC-721 contract.
///
/// A `MetadataGenerator` may query an outside API, a DB, the local filesystem,
/// or any other potential data source. Projects seeking to use this library
/// should make their own metadata generator
#[async_trait]
pub trait MetadataGenerator {
    /// Generate metadata for a specific token
    async fn metadata_for(&self, token_id: U256) -> Option<NftMetadata>;

    /// Generate contract-level metadata (in the OpenSea format). See
    /// [`ContractMetadata`]
    async fn contract_metadata(&self) -> Option<ContractMetadata>;
}

#[cfg(test)]
mod tests {
    use ethers::types::U256;

    #[test]
    fn it_works() {
        let num = "0x3e10".parse::<U256>().unwrap();
        dbg!(num);
        dbg!(num.to_string());
        dbg!(format!("{}", num));
    }
}
