use std::ops::Deref;

use color_eyre::Result;
// use old_contracts_primitives::Code;
// use old_sp_core::Bytes;
// use old_sp_runtime::{
//     traits::{IdentifyAccount, Verify},
//     CryptoType,
// };
use runtime::*;
use subxt::{ClientBuilder, DefaultConfig, SubstrateExtrinsicParams};

pub mod utils;

pub mod services;

type ApiClient = RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>;

pub struct ClientWrapper(ApiClient);

impl AsRef<ApiClient> for ClientWrapper {
    fn as_ref(&self) -> &ApiClient {
        &self.0
    }
}

pub async fn create_api_client() -> Result<ClientWrapper> {
    let api = ClientBuilder::new()
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

    let signer = utils::signer_from_seed("Alice");

    api.try_instantiate(
        signer.account_id().clone(),
        0,
        Code::Upload(Bytes::from(code)),
        vec![].into(),
        vec![].into(),
    )
    .await?;

    let inner: &ApiClient = api.as_ref();

    Ok(())
}
