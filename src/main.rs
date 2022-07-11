use color_eyre::Result;
use pallet_contracts_primitives::{Code, ContractInstantiateResult, ContractResult};

use runtime::laguna::*;
use std::str::FromStr;
use subxt::{sp_core::Bytes, ClientBuilder, DefaultConfig, SubstrateExtrinsicParams};

pub mod utils;

pub mod services;
use codec::{Decode, Encode};

type ApiClient = RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>;

pub struct ClientWrapper(ApiClient);

impl AsRef<ApiClient> for ClientWrapper {
    fn as_ref(&self) -> &ApiClient {
        &self.0
    }
}

pub async fn create_api_client() -> Result<ClientWrapper> {
    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<ApiClient>();
    Ok(ClientWrapper(api))
}

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let api = create_api_client().await?;

    let code = std::fs::read("./examples/contracts-data/ink/basic/dist/basic.wasm")?;

    let mut selector = Bytes::from_str("0xed4b9d1b").map(|v| v.to_vec())?;
    selector.append(&mut true.encode());

    let signer = utils::signer_from_seed("Alice");

    api.try_instantiate(
        signer.account_id().clone(),
        0,
        Code::Upload(Bytes::from(code)),
        selector.into(),
        vec![].into(),
    )
    .await?;

    Ok(())
}
