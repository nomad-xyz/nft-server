use ethers::types::{Address, U256};
use nft_server::prelude::*;

struct Generator;

#[async_trait::async_trait]
impl MetadataGenerator for Generator {
    async fn metadata_for(&self, token_id: U256) -> Option<NftMetadata> {
        if token_id == U256::zero() {
            let image = NftImage::Url {
                image: "https://peach.blender.org/wp-content/uploads/bbb-splash.thumbnail.png"
                    .parse()
                    .unwrap(),
            };
            Some(NftMetadata {
                name: "no".to_owned(),
                description: "hello".to_owned(),
                external_url: "http://example.com/".parse().unwrap(),
                image,
                attributes: vec![],
                background_color: None,
                animation_url: None,
                youtube_url: None,
            })
        } else {
            None
        }
    }

    async fn contract_metadata(&self) -> Option<ContractMetadata> {
        let image = NftImage::Url {
            image: "https://peach.blender.org/wp-content/uploads/bbb-splash.thumbnail.png"
                .parse()
                .unwrap(),
        };
        Some(ContractMetadata {
            name: "Toast".to_owned(),
            description: "Toast".to_owned(),
            image,
            external_link: "http://example.com/".parse().unwrap(),
            seller_fee_basis_points: 300,
            fee_recipient: Address::zero(),
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = serve_generator(Generator, ([0, 0, 0, 0], 8080)).await;
}
