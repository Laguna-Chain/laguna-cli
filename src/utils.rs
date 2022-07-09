use primitives::{AccountId, AccountPublic};
use subxt::sp_runtime::{traits::IdentifyAccount, MultiSignature, MultiSigner};
use subxt::{
    sp_core::{sr25519, Pair, Public},
    DefaultConfig,
};

type PairSigner = subxt::PairSigner<DefaultConfig, subxt::sp_core::sr25519::Pair>;

pub fn signer_from_seed(s: &str) -> PairSigner {
    let pair = sr25519::Pair::from_string(&format!("//{}", s), None).unwrap();

    PairSigner::new(pair)
}
