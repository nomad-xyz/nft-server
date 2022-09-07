use ethers::types::Address;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::NftImage;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenSeaDisplayType {
    // TODO: OpenSea display types
    Date,
    Number,
    BoostPercentage,
    BoostNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenSeaAttributeValue {
    String(String),
    Integer {
        value: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display_type: Option<OpenSeaDisplayType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_value: Option<i64>,
    },
    Float {
        value: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display_type: Option<OpenSeaDisplayType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_value: Option<f64>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenSeaAttribute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trait_type: Option<String>,
    #[serde(flatten)]
    value: OpenSeaAttributeValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub name: String,
    pub description: String,
    pub image: NftImage,
    pub external_link: Url,
    pub seller_fee_basis_points: usize,
    pub fee_recipient: Address,
}
