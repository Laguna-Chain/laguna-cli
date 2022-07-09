use crate::ClientWrapper;
use pallet_contracts_primitives::Code;
// use pallet_contracts_primitives::Code;
// use jsonrpsee::core::client::Client;
// use old_contracts_primitives::Code;
use pallet_contracts_rpc::{self, CallRequest, ContractsApiClient, InstantiateRequest};
use primitives::{AccountId, Balance, BlockNumber, Hash};

use laguna_runtime::opaque::Block;
use subxt::sp_core::Bytes;

// use old_sp_core::Bytes;
use old_sp_rpc::number::NumberOrHex;
use old_sp_runtime::DispatchError;

static MAX_GAS: NumberOrHex = NumberOrHex::Number(200_000_000_000);

impl ClientWrapper {
    pub async fn try_instantiate(
        &self,
        origin: AccountId,
        value: Balance,
        code: Code<Hash>,
        data: Bytes,
        salt: Bytes,
    ) -> crate::Result<()> {
        let client = self.0.client.rpc().client.as_ref();

        let req = InstantiateRequest {
            origin,
            value: NumberOrHex::Number(value as u64),
            gas_limit: MAX_GAS,
            storage_deposit_limit: None,
            code,
            data,
            salt,
        };

        let resp = ContractsApiClient::<Block, BlockNumber, AccountId, Balance, Hash>::instantiate(
            client, req, None,
        )
        .await?;

        if let Err(DispatchError::Module(m)) = resp.result {
            let meta_handle = self.0.client.metadata().clone();
            let meta = meta_handle.read();

            let err = meta.error(m.index, m.error[0])?;

            dbg!(err);
        } else {
            dbg!(resp);
        }

        Ok(())
    }

    pub async fn check_balance(&self, account: AccountId) {}
}
