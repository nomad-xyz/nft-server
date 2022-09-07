use async_trait::async_trait;

use ethers::types::U256;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod generators;
pub mod open_sea;

#[cfg(feature = "axum")]
pub mod server;

use open_sea::{ContractMetadata, OpenSeaAttribute};

pub mod prelude {
    #[cfg(feature = "axum")]
    pub use crate::server::serve;
    pub use crate::{open_sea::*, MetadataGenerator, NftImage, NftMetadata};
    pub use ethers::types::U256;
    pub use url::Url;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NftImage {
    Url { image: url::Url },
    Data { image_data: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub external_url: Url,
    #[serde(flatten)]
    pub image: NftImage,
    #[serde(default)]
    pub attributes: Vec<OpenSeaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<Url>,
}

#[async_trait]
pub trait MetadataGenerator {
    async fn metadata_for(&self, token_id: U256) -> Option<NftMetadata>;

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
