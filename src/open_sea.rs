use ethers::types::Address;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::NftImage;

/// OpenSea display types for [`OpenSeaAttributeValue`] types.
///
/// See [OpenSea documentation](https://docs.opensea.io/docs/metadata-standards)
/// for examples
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenSeaDisplayType {
    /// Date
    Date,
    /// Number
    Number,
    /// BoostPercentage
    BoostPercentage,
    /// BoostNumber
    BoostNumber,
}

/// OpenSea attribute value types.
///
/// See [OpenSea documentation](https://docs.opensea.io/docs/metadata-standards)
/// for examples
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenSeaAttributeValue {
    /// A `String` attribute value
    String {
        /// The value
        value: String,
    },
    /// An `Integer` with display type and max value
    Integer {
        /// The value
        value: i64,
        /// The display type on OpenSea
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display_type: Option<OpenSeaDisplayType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The maximum value for display on OpenSea
        max_value: Option<i64>,
    },
    /// A `Float` with display type and max value
    Float {
        /// The value
        value: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The display type on OpenSea
        display_type: Option<OpenSeaDisplayType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The maximum value for display on OpenSea
        max_value: Option<f64>,
    },
}

impl<T> From<T> for OpenSeaAttributeValue
where
    T: AsRef<str>,
{
    fn from(t: T) -> Self {
        OpenSeaAttributeValue::String {
            value: t.as_ref().to_string(),
        }
    }
}

/// An OpenSea-style NFT attribute. Optionally includes an attribute name (the
/// `trait_type`), as well as the [`OpenSeaAttributeValue`]/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenSeaAttribute {
    /// The attribute name (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trait_type: Option<String>,
    /// The attribute value
    #[serde(flatten)]
    pub value: OpenSeaAttributeValue,
}

impl OpenSeaAttribute {
    /// Shortcut to instantiate a string attribute
    pub fn string<S, T>(trait_type: Option<S>, value: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Self {
            trait_type: trait_type.map(|s| s.as_ref().to_string()),
            value: value.into(),
        }
    }

    /// Shortcut to instantiate a float attribute
    pub fn float<S>(trait_type: Option<S>, value: f64, max_value: Option<f64>) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            trait_type: trait_type.map(|s| s.as_ref().to_string()),
            value: OpenSeaAttributeValue::Float {
                value,
                display_type: None,
                max_value,
            },
        }
    }

    /// Shortcut to instantiate an integer attribute
    pub fn integer<S>(trait_type: Option<S>, value: i64, max_value: Option<i64>) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            trait_type: trait_type.map(|s| s.as_ref().to_string()),
            value: OpenSeaAttributeValue::Integer {
                value,
                display_type: None,
                max_value,
            },
        }
    }
}

/// OpenSea-style contract-level metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractMetadata {
    /// The collection name
    pub name: String,
    /// The collection description
    pub description: String,
    /// The collection image
    #[serde(flatten)]
    pub image: NftImage,
    /// An external link for the NFT collection
    pub external_link: Url,
    /// The seller fee, in bps
    pub seller_fee_basis_points: usize,
    /// The recipient of the seller fee
    pub fee_recipient: Address,
}
