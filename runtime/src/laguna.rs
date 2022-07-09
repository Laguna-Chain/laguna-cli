#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 14usize] = [
        "System",
        "Timestamp",
        "Sudo",
        "Scheduler",
        "Tokens",
        "Currencies",
        "ContractAssetsRegistry",
        "TransactionPayment",
        "FluentFee",
        "FeeEnablement",
        "Aura",
        "Grandpa",
        "Contracts",
        "RandomnessCollectiveFlip",
    ];
    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 2)]
        Sudo(sudo::Event),
        #[codec(index = 3)]
        Scheduler(scheduler::Event),
        #[codec(index = 4)]
        Tokens(tokens::Event),
        #[codec(index = 8)]
        FluentFee(fluent_fee::Event),
        #[codec(index = 11)]
        Grandpa(grandpa::Event),
        #[codec(index = 12)]
        Contracts(contracts::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        FillBlock,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<FillBlock>()?
                    };
                    if runtime_call_hash
                        == [
                            228u8, 117u8, 251u8, 95u8, 47u8, 56u8, 32u8, 177u8, 191u8, 72u8, 75u8,
                            23u8, 193u8, 175u8, 227u8, 218u8, 127u8, 94u8, 114u8, 110u8, 215u8,
                            61u8, 162u8, 102u8, 73u8, 89u8, 218u8, 148u8, 59u8, 73u8, 59u8, 149u8,
                        ]
                    {
                        let call = FillBlock { ratio };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Remark>()?
                    };
                    if runtime_call_hash
                        == [
                            186u8, 79u8, 33u8, 199u8, 216u8, 115u8, 19u8, 146u8, 220u8, 174u8,
                            98u8, 61u8, 179u8, 230u8, 40u8, 70u8, 22u8, 251u8, 77u8, 62u8, 133u8,
                            80u8, 186u8, 70u8, 135u8, 172u8, 178u8, 241u8, 69u8, 106u8, 235u8,
                            140u8,
                        ]
                    {
                        let call = Remark { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetHeapPages,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetHeapPages>()?
                    };
                    if runtime_call_hash
                        == [
                            77u8, 138u8, 122u8, 55u8, 179u8, 101u8, 60u8, 137u8, 173u8, 39u8, 28u8,
                            36u8, 237u8, 243u8, 232u8, 162u8, 76u8, 176u8, 135u8, 58u8, 60u8,
                            177u8, 105u8, 136u8, 94u8, 53u8, 26u8, 31u8, 41u8, 156u8, 228u8, 241u8,
                        ]
                    {
                        let call = SetHeapPages { pages };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetCode>()?
                    };
                    if runtime_call_hash
                        == [
                            35u8, 75u8, 103u8, 203u8, 91u8, 141u8, 77u8, 95u8, 37u8, 157u8, 107u8,
                            240u8, 54u8, 242u8, 245u8, 205u8, 104u8, 165u8, 177u8, 37u8, 86u8,
                            197u8, 28u8, 202u8, 121u8, 159u8, 18u8, 204u8, 237u8, 117u8, 141u8,
                            131u8,
                        ]
                    {
                        let call = SetCode { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCodeWithoutChecks,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetCodeWithoutChecks>()?
                    };
                    if runtime_call_hash
                        == [
                            150u8, 148u8, 119u8, 129u8, 77u8, 216u8, 135u8, 187u8, 127u8, 24u8,
                            238u8, 15u8, 227u8, 229u8, 191u8, 217u8, 106u8, 129u8, 149u8, 79u8,
                            154u8, 78u8, 53u8, 159u8, 89u8, 69u8, 103u8, 197u8, 93u8, 161u8, 134u8,
                            17u8,
                        ]
                    {
                        let call = SetCodeWithoutChecks { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetStorage>()?
                    };
                    if runtime_call_hash
                        == [
                            197u8, 12u8, 119u8, 205u8, 152u8, 103u8, 211u8, 170u8, 146u8, 253u8,
                            25u8, 56u8, 180u8, 146u8, 74u8, 75u8, 38u8, 108u8, 212u8, 154u8, 23u8,
                            22u8, 148u8, 175u8, 107u8, 186u8, 222u8, 13u8, 149u8, 132u8, 204u8,
                            217u8,
                        ]
                    {
                        let call = SetStorage { items };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<KillStorage>()?
                    };
                    if runtime_call_hash
                        == [
                            154u8, 115u8, 185u8, 20u8, 126u8, 90u8, 222u8, 131u8, 199u8, 57u8,
                            184u8, 226u8, 43u8, 245u8, 161u8, 176u8, 194u8, 123u8, 139u8, 97u8,
                            97u8, 94u8, 47u8, 64u8, 204u8, 96u8, 190u8, 94u8, 216u8, 237u8, 69u8,
                            51u8,
                        ]
                    {
                        let call = KillStorage { keys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillPrefix,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<KillPrefix>()?
                    };
                    if runtime_call_hash
                        == [
                            214u8, 101u8, 191u8, 241u8, 1u8, 241u8, 144u8, 116u8, 246u8, 199u8,
                            159u8, 249u8, 155u8, 164u8, 220u8, 221u8, 75u8, 33u8, 204u8, 3u8,
                            255u8, 201u8, 187u8, 238u8, 181u8, 213u8, 41u8, 105u8, 234u8, 120u8,
                            202u8, 115u8,
                        ]
                    {
                        let call = KillPrefix { prefix, subkeys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        RemarkWithEvent,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<RemarkWithEvent>()?
                    };
                    if runtime_call_hash
                        == [
                            171u8, 82u8, 75u8, 237u8, 69u8, 197u8, 223u8, 125u8, 123u8, 51u8,
                            241u8, 35u8, 202u8, 210u8, 227u8, 109u8, 1u8, 241u8, 255u8, 63u8, 33u8,
                            115u8, 156u8, 239u8, 97u8, 76u8, 193u8, 35u8, 74u8, 199u8, 43u8, 255u8,
                        ]
                    {
                        let call = RemarkWithEvent { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::laguna_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Account>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8,
                                130u8, 23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8,
                                88u8, 71u8, 168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8,
                                195u8, 46u8,
                            ]
                        {
                            let entry = Account(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Account<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Account>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8,
                                130u8, 23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8,
                                88u8, 71u8, 168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8,
                                195u8, 46u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::u32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ExtrinsicCount>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                                222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8,
                                136u8, 41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8,
                                163u8, 231u8,
                            ]
                        {
                            let entry = ExtrinsicCount;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_support::weights::PerDispatchClass<
                            ::core::primitive::u64,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<BlockWeight>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                2u8, 236u8, 190u8, 174u8, 244u8, 98u8, 194u8, 168u8, 89u8, 208u8,
                                7u8, 45u8, 175u8, 171u8, 177u8, 121u8, 215u8, 190u8, 184u8, 195u8,
                                49u8, 133u8, 44u8, 1u8, 181u8, 215u8, 89u8, 84u8, 255u8, 16u8,
                                57u8, 152u8,
                            ]
                        {
                            let entry = BlockWeight;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::u32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<AllExtrinsicsLen>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                                254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                                219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                                134u8, 60u8,
                            ]
                        {
                            let entry = AllExtrinsicsLen;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: &'a ::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<BlockHash>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8,
                                24u8, 236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8,
                                111u8, 141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8,
                                246u8, 120u8,
                            ]
                        {
                            let entry = BlockHash(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<BlockHash>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8,
                                24u8, 236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8,
                                111u8, 141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8,
                                246u8, 120u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: &'a ::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ExtrinsicData>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8,
                                211u8, 254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8,
                                23u8, 241u8, 59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8,
                                62u8, 50u8, 67u8,
                            ]
                        {
                            let entry = ExtrinsicData(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ExtrinsicData>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8,
                                211u8, 254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8,
                                23u8, 241u8, 59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8,
                                62u8, 50u8, 67u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Number>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8,
                                235u8, 246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8,
                                204u8, 36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8,
                                235u8, 164u8, 191u8,
                            ]
                        {
                            let entry = Number;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ParentHash>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                194u8, 221u8, 147u8, 22u8, 68u8, 141u8, 32u8, 6u8, 202u8, 39u8,
                                164u8, 184u8, 69u8, 126u8, 190u8, 101u8, 215u8, 27u8, 127u8, 157u8,
                                200u8, 69u8, 170u8, 139u8, 232u8, 27u8, 254u8, 181u8, 183u8, 105u8,
                                111u8, 177u8,
                            ]
                        {
                            let entry = ParentHash;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::sp_runtime::generic::digest::Digest,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Digest>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                10u8, 176u8, 13u8, 228u8, 226u8, 42u8, 210u8, 151u8, 107u8, 212u8,
                                136u8, 15u8, 38u8, 182u8, 225u8, 12u8, 250u8, 56u8, 193u8, 243u8,
                                219u8, 113u8, 95u8, 233u8, 21u8, 229u8, 125u8, 146u8, 92u8, 250u8,
                                32u8, 168u8,
                            ]
                        {
                            let entry = Digest;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                runtime_types::laguna_runtime::Event,
                                ::subxt::sp_core::H256,
                            >,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Events>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                214u8, 159u8, 248u8, 196u8, 247u8, 129u8, 245u8, 109u8, 171u8,
                                32u8, 1u8, 154u8, 101u8, 242u8, 168u8, 37u8, 24u8, 49u8, 86u8,
                                245u8, 146u8, 208u8, 48u8, 186u8, 123u8, 54u8, 103u8, 247u8, 98u8,
                                78u8, 86u8, 232u8,
                            ]
                        {
                            let entry = Events;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<EventCount>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                                203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                                161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8,
                                66u8, 112u8,
                            ]
                        {
                            let entry = EventCount;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: &'a ::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<EventTopics>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                                140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8,
                                93u8, 178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8,
                                245u8, 179u8,
                            ]
                        {
                            let entry = EventTopics(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, EventTopics<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<EventTopics>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                                140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8,
                                93u8, 178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8,
                                245u8, 179u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<LastRuntimeUpgrade>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                219u8, 153u8, 158u8, 38u8, 45u8, 65u8, 151u8, 137u8, 53u8, 76u8,
                                11u8, 181u8, 218u8, 248u8, 125u8, 190u8, 100u8, 240u8, 173u8, 75u8,
                                179u8, 137u8, 198u8, 197u8, 248u8, 185u8, 118u8, 58u8, 42u8, 165u8,
                                125u8, 119u8,
                            ]
                        {
                            let entry = LastRuntimeUpgrade;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<UpgradedToU32RefCount>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8,
                                175u8, 178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8,
                                3u8, 114u8, 83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8,
                                131u8, 204u8,
                            ]
                        {
                            let entry = UpgradedToU32RefCount;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<UpgradedToTripleRefCount>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8,
                                201u8, 210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8,
                                103u8, 155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8,
                                40u8, 185u8,
                            ]
                        {
                            let entry = UpgradedToTripleRefCount;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<runtime_types::frame_system::Phase>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ExecutionPhase>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                174u8, 13u8, 230u8, 220u8, 239u8, 161u8, 172u8, 122u8, 188u8, 95u8,
                                141u8, 118u8, 91u8, 158u8, 111u8, 145u8, 243u8, 173u8, 226u8,
                                212u8, 187u8, 118u8, 94u8, 132u8, 221u8, 244u8, 61u8, 148u8, 217u8,
                                30u8, 238u8, 225u8,
                            ]
                        {
                            let entry = ExecutionPhase;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "BlockWeights")?
                        == [
                            49u8, 105u8, 160u8, 185u8, 41u8, 100u8, 26u8, 18u8, 154u8, 196u8,
                            108u8, 96u8, 36u8, 148u8, 28u8, 162u8, 92u8, 234u8, 89u8, 152u8, 149u8,
                            176u8, 186u8, 20u8, 217u8, 167u8, 59u8, 167u8, 106u8, 9u8, 205u8,
                            106u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("BlockWeights")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "BlockLength")?
                        == [
                            120u8, 249u8, 182u8, 103u8, 246u8, 214u8, 149u8, 44u8, 42u8, 64u8, 2u8,
                            56u8, 157u8, 184u8, 43u8, 195u8, 214u8, 251u8, 207u8, 207u8, 249u8,
                            105u8, 203u8, 108u8, 179u8, 93u8, 93u8, 246u8, 40u8, 175u8, 160u8,
                            114u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("BlockLength")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "BlockHashCount")?
                        == [
                            123u8, 126u8, 182u8, 103u8, 71u8, 187u8, 233u8, 8u8, 47u8, 226u8,
                            159u8, 139u8, 0u8, 59u8, 190u8, 135u8, 189u8, 77u8, 190u8, 81u8, 39u8,
                            198u8, 224u8, 219u8, 70u8, 143u8, 6u8, 132u8, 196u8, 61u8, 117u8,
                            194u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("BlockHashCount")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "DbWeight")?
                        == [
                            203u8, 8u8, 106u8, 152u8, 74u8, 132u8, 2u8, 132u8, 244u8, 106u8, 147u8,
                            12u8, 93u8, 80u8, 61u8, 158u8, 172u8, 178u8, 228u8, 125u8, 213u8,
                            102u8, 75u8, 210u8, 64u8, 185u8, 204u8, 84u8, 10u8, 164u8, 204u8, 62u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("DbWeight")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "Version")?
                        == [
                            104u8, 169u8, 131u8, 128u8, 84u8, 245u8, 0u8, 179u8, 164u8, 28u8, 19u8,
                            140u8, 4u8, 127u8, 148u8, 125u8, 221u8, 64u8, 143u8, 173u8, 239u8,
                            114u8, 76u8, 225u8, 28u8, 149u8, 108u8, 194u8, 34u8, 154u8, 176u8,
                            34u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("Version")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("System", "SS58Prefix")?
                        == [
                            197u8, 217u8, 49u8, 68u8, 82u8, 238u8, 120u8, 50u8, 91u8, 58u8, 6u8,
                            156u8, 40u8, 1u8, 241u8, 213u8, 141u8, 74u8, 83u8, 115u8, 117u8, 41u8,
                            119u8, 50u8, 140u8, 136u8, 163u8, 185u8, 34u8, 190u8, 60u8, 97u8,
                        ]
                    {
                        let pallet = metadata.pallet("System")?;
                        let constant = pallet.constant("SS58Prefix")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Set>()?
                    };
                    if runtime_call_hash
                        == [
                            191u8, 73u8, 102u8, 150u8, 65u8, 157u8, 172u8, 194u8, 7u8, 72u8, 1u8,
                            35u8, 54u8, 99u8, 245u8, 139u8, 40u8, 136u8, 245u8, 53u8, 167u8, 100u8,
                            143u8, 244u8, 160u8, 5u8, 18u8, 130u8, 77u8, 160u8, 227u8, 51u8,
                        ]
                    {
                        let call = Set { now };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Now>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8,
                                144u8, 221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8,
                                202u8, 185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8,
                                2u8, 250u8,
                            ]
                        {
                            let entry = Now;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<DidUpdate>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8,
                                175u8, 13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8,
                                167u8, 85u8, 27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8,
                                72u8, 204u8,
                            ]
                        {
                            let entry = DidUpdate;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Timestamp", "MinimumPeriod")?
                        == [
                            141u8, 242u8, 40u8, 24u8, 83u8, 43u8, 33u8, 194u8, 156u8, 149u8, 219u8,
                            61u8, 10u8, 123u8, 120u8, 247u8, 228u8, 22u8, 25u8, 24u8, 214u8, 188u8,
                            54u8, 135u8, 240u8, 162u8, 41u8, 216u8, 3u8, 58u8, 238u8, 39u8,
                        ]
                    {
                        let pallet = metadata.pallet("Timestamp")?;
                        let constant = pallet.constant("MinimumPeriod")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(
                    &self,
                    call: runtime_types::laguna_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Sudo>()?
                    };
                    if runtime_call_hash
                        == [
                            127u8, 198u8, 61u8, 65u8, 53u8, 192u8, 81u8, 4u8, 81u8, 151u8, 43u8,
                            242u8, 105u8, 141u8, 93u8, 230u8, 81u8, 146u8, 163u8, 247u8, 191u8,
                            53u8, 128u8, 178u8, 7u8, 247u8, 235u8, 99u8, 184u8, 81u8, 66u8, 125u8,
                        ]
                    {
                        let call = Sudo {
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::laguna_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SudoUncheckedWeight,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SudoUncheckedWeight>()?
                    };
                    if runtime_call_hash
                        == [
                            156u8, 219u8, 206u8, 77u8, 77u8, 164u8, 30u8, 61u8, 15u8, 195u8, 91u8,
                            168u8, 75u8, 250u8, 77u8, 59u8, 105u8, 98u8, 136u8, 148u8, 147u8,
                            156u8, 230u8, 157u8, 30u8, 199u8, 57u8, 202u8, 125u8, 28u8, 167u8,
                            253u8,
                        ]
                    {
                        let call = SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetKey>()?
                    };
                    if runtime_call_hash
                        == [
                            77u8, 253u8, 211u8, 157u8, 74u8, 92u8, 1u8, 102u8, 178u8, 103u8, 126u8,
                            56u8, 156u8, 105u8, 45u8, 44u8, 64u8, 154u8, 163u8, 102u8, 93u8, 93u8,
                            212u8, 5u8, 148u8, 184u8, 22u8, 135u8, 110u8, 102u8, 44u8, 172u8,
                        ]
                    {
                        let call = SetKey { new };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::laguna_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SudoAs>()?
                    };
                    if runtime_call_hash
                        == [
                            68u8, 79u8, 124u8, 83u8, 194u8, 191u8, 217u8, 57u8, 102u8, 51u8, 241u8,
                            239u8, 190u8, 81u8, 250u8, 58u8, 246u8, 65u8, 139u8, 177u8, 63u8,
                            231u8, 80u8, 160u8, 103u8, 210u8, 202u8, 29u8, 165u8, 150u8, 7u8, 11u8,
                        ]
                    {
                        let call = SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Key>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                222u8, 90u8, 158u8, 233u8, 184u8, 23u8, 141u8, 135u8, 81u8, 187u8,
                                47u8, 100u8, 30u8, 81u8, 239u8, 197u8, 249u8, 253u8, 73u8, 207u8,
                                161u8, 141u8, 174u8, 59u8, 74u8, 181u8, 10u8, 90u8, 22u8, 109u8,
                                62u8, 27u8,
                            ]
                        {
                            let entry = Key;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
    }
    pub mod scheduler {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                >,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                >,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                >,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                >,
            }
            impl ::subxt::Call for ScheduleNamedAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Anonymously schedule a task."]
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Schedule,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Schedule>()?
                    };
                    if runtime_call_hash
                        == [
                            51u8, 177u8, 57u8, 54u8, 236u8, 178u8, 122u8, 243u8, 108u8, 21u8,
                            236u8, 156u8, 213u8, 36u8, 241u8, 93u8, 200u8, 62u8, 62u8, 51u8, 197u8,
                            112u8, 137u8, 226u8, 2u8, 242u8, 127u8, 104u8, 176u8, 16u8, 31u8, 43u8,
                        ]
                    {
                        let call = Schedule {
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Cancel an anonymously scheduled task."]
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Cancel, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Cancel>()?
                    };
                    if runtime_call_hash
                        == [
                            118u8, 0u8, 188u8, 218u8, 148u8, 86u8, 139u8, 15u8, 3u8, 161u8, 6u8,
                            150u8, 46u8, 32u8, 85u8, 179u8, 106u8, 113u8, 240u8, 115u8, 167u8,
                            114u8, 243u8, 69u8, 103u8, 60u8, 99u8, 135u8, 21u8, 8u8, 19u8, 225u8,
                        ]
                    {
                        let call = Cancel { when, index };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Schedule a named task."]
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ScheduleNamed,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ScheduleNamed>()?
                    };
                    if runtime_call_hash
                        == [
                            167u8, 62u8, 74u8, 31u8, 56u8, 3u8, 213u8, 236u8, 63u8, 204u8, 160u8,
                            225u8, 207u8, 21u8, 252u8, 133u8, 221u8, 215u8, 167u8, 227u8, 51u8,
                            213u8, 232u8, 81u8, 93u8, 77u8, 160u8, 56u8, 27u8, 42u8, 39u8, 36u8,
                        ]
                    {
                        let call = ScheduleNamed {
                            id,
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Cancel a named scheduled task."]
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CancelNamed,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<CancelNamed>()?
                    };
                    if runtime_call_hash
                        == [
                            118u8, 221u8, 232u8, 126u8, 67u8, 134u8, 33u8, 7u8, 224u8, 110u8,
                            181u8, 18u8, 57u8, 39u8, 15u8, 64u8, 90u8, 132u8, 2u8, 238u8, 19u8,
                            241u8, 194u8, 120u8, 5u8, 109u8, 74u8, 205u8, 42u8, 244u8, 99u8, 54u8,
                        ]
                    {
                        let call = CancelNamed { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Anonymously schedule a task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule`]."]
                #[doc = "# </weight>"]
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ScheduleAfter,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ScheduleAfter>()?
                    };
                    if runtime_call_hash
                        == [
                            147u8, 121u8, 131u8, 93u8, 4u8, 115u8, 148u8, 68u8, 102u8, 43u8, 201u8,
                            188u8, 122u8, 55u8, 141u8, 73u8, 143u8, 44u8, 178u8, 134u8, 2u8, 93u8,
                            231u8, 202u8, 209u8, 252u8, 212u8, 41u8, 17u8, 234u8, 128u8, 30u8,
                        ]
                    {
                        let call = ScheduleAfter {
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Schedule a named task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                #[doc = "# </weight>"]
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::laguna_runtime::Call,
                        ::subxt::sp_core::H256,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ScheduleNamedAfter,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ScheduleNamedAfter>()?
                    };
                    if runtime_call_hash
                        == [
                            242u8, 109u8, 227u8, 101u8, 68u8, 169u8, 101u8, 104u8, 109u8, 95u8,
                            57u8, 165u8, 205u8, 50u8, 60u8, 78u8, 84u8, 203u8, 14u8, 168u8, 187u8,
                            198u8, 62u8, 216u8, 161u8, 21u8, 22u8, 246u8, 224u8, 79u8, 53u8, 88u8,
                        ]
                    {
                        let call = ScheduleNamedAfter {
                            id,
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "Events type."]
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Scheduled some task."]
            pub struct Scheduled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Canceled some task."]
            pub struct Canceled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Dispatched some task."]
            pub struct Dispatched {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The call for the provided hash was not found so the task has been aborted."]
            pub struct CallLookupFailed {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub error: runtime_types::frame_support::traits::schedule::LookupError,
            }
            impl ::subxt::Event for CallLookupFailed {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "CallLookupFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda<'_> {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<
                        runtime_types::pallet_scheduler::ScheduledV3<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::laguna_runtime::Call,
                                ::subxt::sp_core::H256,
                            >,
                            ::core::primitive::u32,
                            runtime_types::laguna_runtime::OriginCaller,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Lookup<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for Lookup<'_> {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda(
                    &self,
                    _0: &'a ::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::std::vec::Vec<
                            ::core::option::Option<
                                runtime_types::pallet_scheduler::ScheduledV3<
                                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                                        runtime_types::laguna_runtime::Call,
                                        ::subxt::sp_core::H256,
                                    >,
                                    ::core::primitive::u32,
                                    runtime_types::laguna_runtime::OriginCaller,
                                    ::subxt::sp_core::crypto::AccountId32,
                                >,
                            >,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Agenda>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                57u8, 13u8, 118u8, 231u8, 165u8, 237u8, 69u8, 85u8, 103u8, 50u8,
                                16u8, 79u8, 228u8, 110u8, 64u8, 38u8, 217u8, 84u8, 111u8, 114u8,
                                4u8, 50u8, 174u8, 117u8, 84u8, 81u8, 209u8, 74u8, 117u8, 40u8,
                                202u8, 56u8,
                            ]
                        {
                            let entry = Agenda(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Agenda<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Agenda>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                57u8, 13u8, 118u8, 231u8, 165u8, 237u8, 69u8, 85u8, 103u8, 50u8,
                                16u8, 79u8, 228u8, 110u8, 64u8, 38u8, 217u8, 84u8, 111u8, 114u8,
                                4u8, 50u8, 174u8, 117u8, 84u8, 81u8, 209u8, 74u8, 117u8, 40u8,
                                202u8, 56u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Lookup from identity to the block number and index of the task."]
                pub fn lookup(
                    &self,
                    _0: &'a [::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Lookup>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
                                180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8,
                                36u8, 202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8,
                                62u8, 166u8,
                            ]
                        {
                            let entry = Lookup(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Lookup from identity to the block number and index of the task."]
                pub fn lookup_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Lookup<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Lookup>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
                                180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8,
                                36u8, 202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8,
                                62u8, 166u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The maximum weight that may be scheduled per block for any dispatchables of less"]
                #[doc = " priority than `schedule::HARD_DEADLINE`."]
                pub fn maximum_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Scheduler", "MaximumWeight")?
                        == [
                            235u8, 167u8, 74u8, 91u8, 5u8, 188u8, 76u8, 138u8, 208u8, 10u8, 100u8,
                            241u8, 65u8, 185u8, 195u8, 212u8, 38u8, 161u8, 27u8, 113u8, 220u8,
                            214u8, 28u8, 214u8, 67u8, 169u8, 21u8, 10u8, 230u8, 130u8, 251u8,
                            175u8,
                        ]
                    {
                        let pallet = metadata.pallet("Scheduler")?;
                        let constant = pallet.constant("MaximumWeight")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of scheduled calls in the queue for a single block."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_scheduled_per_block(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Scheduler", "MaxScheduledPerBlock")?
                        == [
                            64u8, 25u8, 128u8, 202u8, 165u8, 97u8, 30u8, 196u8, 174u8, 132u8,
                            139u8, 223u8, 88u8, 20u8, 228u8, 203u8, 253u8, 201u8, 83u8, 157u8,
                            161u8, 120u8, 187u8, 165u8, 4u8, 64u8, 184u8, 34u8, 28u8, 129u8, 136u8,
                            13u8,
                        ]
                    {
                        let pallet = metadata.pallet("Scheduler")?;
                        let constant = pallet.constant("MaxScheduledPerBlock")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod tokens {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "set_balance";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "It will decrease the total issuance of the system by the"]
                #[doc = "`TransferFee`. If the sender's account is below the existential"]
                #[doc = "deposit as a result of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the"]
                #[doc = "transactor."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `currency_id`: currency type."]
                #[doc = "- `amount`: free balance amount to tranfer."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Transfer>()?
                    };
                    if runtime_call_hash
                        == [
                            216u8, 91u8, 92u8, 153u8, 196u8, 239u8, 118u8, 106u8, 218u8, 8u8, 31u8,
                            243u8, 63u8, 64u8, 0u8, 172u8, 130u8, 169u8, 37u8, 218u8, 158u8, 167u8,
                            193u8, 12u8, 26u8, 159u8, 70u8, 116u8, 48u8, 155u8, 114u8, 103u8,
                        ]
                    {
                        let call = Transfer {
                            dest,
                            currency_id,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Transfer all remaining balance to the given account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_"]
                #[doc = "balances. This means that any locked, reserved, or existential"]
                #[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
                #[doc = "this function. To ensure that this function results in a killed"]
                #[doc = "account, you might need to prepare the account by removing any"]
                #[doc = "reference counters, storage deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the"]
                #[doc = "transactor."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `currency_id`: currency type."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
                #[doc = "  operation should send all of the funds the account has, causing"]
                #[doc = "  the sender account to be killed (false), or transfer everything"]
                #[doc = "  except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    keep_alive: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferAll,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<TransferAll>()?
                    };
                    if runtime_call_hash
                        == [
                            197u8, 54u8, 72u8, 99u8, 111u8, 108u8, 59u8, 134u8, 237u8, 168u8,
                            177u8, 214u8, 189u8, 85u8, 209u8, 167u8, 234u8, 90u8, 163u8, 253u8,
                            78u8, 89u8, 198u8, 14u8, 211u8, 65u8, 63u8, 167u8, 121u8, 127u8, 114u8,
                            118u8,
                        ]
                    {
                        let call = TransferAll {
                            dest,
                            currency_id,
                            keep_alive,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
                #[doc = "will not kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the"]
                #[doc = "transactor."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `currency_id`: currency type."]
                #[doc = "- `amount`: free balance amount to tranfer."]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferKeepAlive,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<TransferKeepAlive>()?
                    };
                    if runtime_call_hash
                        == [
                            151u8, 91u8, 66u8, 218u8, 246u8, 88u8, 141u8, 1u8, 250u8, 151u8, 43u8,
                            30u8, 202u8, 255u8, 216u8, 173u8, 75u8, 127u8, 179u8, 105u8, 145u8,
                            213u8, 173u8, 37u8, 11u8, 40u8, 115u8, 37u8, 83u8, 205u8, 190u8, 193u8,
                        ]
                    {
                        let call = TransferKeepAlive {
                            dest,
                            currency_id,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source"]
                #[doc = "account may be specified."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Root_."]
                #[doc = ""]
                #[doc = "- `source`: The sender of the transfer."]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `currency_id`: currency type."]
                #[doc = "- `amount`: free balance amount to tranfer."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ForceTransfer>()?
                    };
                    if runtime_call_hash
                        == [
                            237u8, 36u8, 148u8, 139u8, 134u8, 34u8, 170u8, 58u8, 135u8, 59u8,
                            219u8, 107u8, 203u8, 227u8, 155u8, 59u8, 156u8, 14u8, 68u8, 126u8,
                            109u8, 178u8, 255u8, 209u8, 246u8, 181u8, 4u8, 193u8, 151u8, 30u8,
                            121u8, 56u8,
                        ]
                    {
                        let call = ForceTransfer {
                            source,
                            dest,
                            currency_id,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
                #[doc = "will also decrease the total issuance of the system"]
                #[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
                #[doc = "existential deposit, it will reap the `AccountInfo`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetBalance,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SetBalance>()?
                    };
                    if runtime_call_hash
                        == [
                            10u8, 248u8, 44u8, 166u8, 75u8, 181u8, 81u8, 101u8, 67u8, 33u8, 169u8,
                            53u8, 221u8, 122u8, 39u8, 80u8, 112u8, 93u8, 250u8, 44u8, 179u8, 46u8,
                            97u8, 61u8, 44u8, 103u8, 35u8, 175u8, 4u8, 165u8, 34u8, 66u8,
                        ]
                    {
                        let call = SetBalance {
                            who,
                            currency_id,
                            new_free,
                            new_reserved,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::orml_tokens::module::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was removed whose balance was non-zero but below"]
            #[doc = "ExistentialDeposit, resulting in an outright loss."]
            pub struct DustLost {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some reserved balance was repatriated (moved from reserved to"]
            #[doc = "another account)."]
            pub struct ReserveRepatriated {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The total issuance of an currency has been set"]
            pub struct TotalIssuanceSet {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for TotalIssuanceSet {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "TotalIssuanceSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balances were withdrawn (e.g. pay for transaction fee)"]
            pub struct Withdrawn {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balances were slashed (e.g. due to mis-behavior)"]
            pub struct Slashed {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free_amount: ::core::primitive::u128,
                pub reserved_amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Deposited some balance into an account"]
            pub struct Deposited {
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposited {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Deposited";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some funds are locked"]
            pub struct LockSet {
                pub lock_id: [::core::primitive::u8; 8usize],
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for LockSet {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "LockSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some locked funds were unlocked"]
            pub struct LockRemoved {
                pub lock_id: [::core::primitive::u8; 8usize],
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for LockRemoved {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "LockRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance<'a>(pub &'a runtime_types::primitives::currency::CurrencyId);
            impl ::subxt::StorageEntry for TotalIssuance<'_> {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Locks<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a runtime_types::primitives::currency::CurrencyId,
            );
            impl ::subxt::StorageEntry for Locks<'_> {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Accounts<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a runtime_types::primitives::currency::CurrencyId,
            );
            impl ::subxt::StorageEntry for Accounts<'_> {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "Accounts";
                type Value = runtime_types::orml_tokens::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Reserves<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a runtime_types::primitives::currency::CurrencyId,
            );
            impl ::subxt::StorageEntry for Reserves<'_> {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::orml_tokens::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The total issuance of a token type."]
                pub fn total_issuance(
                    &self,
                    _0: &'a runtime_types::primitives::currency::CurrencyId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<TotalIssuance>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                78u8, 198u8, 9u8, 36u8, 221u8, 55u8, 154u8, 215u8, 110u8, 118u8,
                                95u8, 174u8, 228u8, 37u8, 40u8, 165u8, 169u8, 96u8, 36u8, 41u8,
                                163u8, 181u8, 110u8, 139u8, 83u8, 185u8, 16u8, 156u8, 186u8, 233u8,
                                93u8, 139u8,
                            ]
                        {
                            let entry = TotalIssuance(_0);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The total issuance of a token type."]
                pub fn total_issuance_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, TotalIssuance<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<TotalIssuance>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                78u8, 198u8, 9u8, 36u8, 221u8, 55u8, 154u8, 215u8, 110u8, 118u8,
                                95u8, 174u8, 228u8, 37u8, 40u8, 165u8, 169u8, 96u8, 36u8, 41u8,
                                163u8, 181u8, 110u8, 139u8, 83u8, 185u8, 16u8, 156u8, 186u8, 233u8,
                                93u8, 139u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Any liquidity locks of a token type under an account."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    _1: &'a runtime_types::primitives::currency::CurrencyId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Locks>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                178u8, 243u8, 164u8, 69u8, 185u8, 227u8, 3u8, 197u8, 175u8, 100u8,
                                141u8, 71u8, 79u8, 94u8, 155u8, 174u8, 219u8, 54u8, 30u8, 0u8,
                                163u8, 95u8, 235u8, 40u8, 253u8, 168u8, 35u8, 233u8, 8u8, 32u8,
                                196u8, 164u8,
                            ]
                        {
                            let entry = Locks(_0, _1);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Any liquidity locks of a token type under an account."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Locks<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Locks>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                178u8, 243u8, 164u8, 69u8, 185u8, 227u8, 3u8, 197u8, 175u8, 100u8,
                                141u8, 71u8, 79u8, 94u8, 155u8, 174u8, 219u8, 54u8, 30u8, 0u8,
                                163u8, 95u8, 235u8, 40u8, 253u8, 168u8, 35u8, 233u8, 8u8, 32u8,
                                196u8, 164u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The balance of a token type under an account."]
                #[doc = ""]
                #[doc = " NOTE: If the total is ever zero, decrease account ref account."]
                #[doc = ""]
                #[doc = " NOTE: This is only used in the case that this module is used to store"]
                #[doc = " balances."]
                pub fn accounts(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    _1: &'a runtime_types::primitives::currency::CurrencyId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Accounts>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                115u8, 119u8, 87u8, 129u8, 25u8, 115u8, 80u8, 122u8, 178u8, 202u8,
                                36u8, 66u8, 215u8, 212u8, 158u8, 204u8, 152u8, 71u8, 63u8, 63u8,
                                57u8, 190u8, 103u8, 105u8, 250u8, 249u8, 74u8, 194u8, 188u8, 8u8,
                                202u8, 120u8,
                            ]
                        {
                            let entry = Accounts(_0, _1);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The balance of a token type under an account."]
                #[doc = ""]
                #[doc = " NOTE: If the total is ever zero, decrease account ref account."]
                #[doc = ""]
                #[doc = " NOTE: This is only used in the case that this module is used to store"]
                #[doc = " balances."]
                pub fn accounts_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Accounts<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Accounts>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                115u8, 119u8, 87u8, 129u8, 25u8, 115u8, 80u8, 122u8, 178u8, 202u8,
                                36u8, 66u8, 215u8, 212u8, 158u8, 204u8, 152u8, 71u8, 63u8, 63u8,
                                57u8, 190u8, 103u8, 105u8, 250u8, 249u8, 74u8, 194u8, 188u8, 8u8,
                                202u8, 120u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    _1: &'a runtime_types::primitives::currency::CurrencyId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::orml_tokens::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Reserves>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                145u8, 60u8, 151u8, 131u8, 48u8, 247u8, 69u8, 5u8, 198u8, 32u8,
                                225u8, 250u8, 85u8, 47u8, 121u8, 142u8, 169u8, 131u8, 66u8, 78u8,
                                120u8, 165u8, 72u8, 46u8, 4u8, 17u8, 135u8, 165u8, 29u8, 103u8,
                                203u8, 125u8,
                            ]
                        {
                            let entry = Reserves(_0, _1);
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, Reserves<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Reserves>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                145u8, 60u8, 151u8, 131u8, 48u8, 247u8, 69u8, 5u8, 198u8, 32u8,
                                225u8, 250u8, 85u8, 47u8, 121u8, 142u8, 169u8, 131u8, 66u8, 78u8,
                                120u8, 165u8, 72u8, 46u8, 4u8, 17u8, 135u8, 165u8, 29u8, 103u8,
                                203u8, 125u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Tokens", "MaxLocks")?
                        == [
                            250u8, 58u8, 19u8, 15u8, 35u8, 113u8, 227u8, 89u8, 39u8, 75u8, 21u8,
                            108u8, 202u8, 32u8, 163u8, 167u8, 207u8, 233u8, 69u8, 151u8, 53u8,
                            164u8, 230u8, 16u8, 14u8, 22u8, 172u8, 46u8, 36u8, 216u8, 29u8, 1u8,
                        ]
                    {
                        let pallet = metadata.pallet("Tokens")?;
                        let constant = pallet.constant("MaxLocks")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Tokens", "MaxReserves")?
                        == [
                            117u8, 207u8, 208u8, 40u8, 3u8, 139u8, 184u8, 190u8, 227u8, 156u8,
                            207u8, 252u8, 250u8, 235u8, 170u8, 16u8, 211u8, 218u8, 21u8, 251u8,
                            32u8, 22u8, 8u8, 3u8, 224u8, 40u8, 165u8, 131u8, 30u8, 97u8, 205u8,
                            19u8,
                        ]
                    {
                        let pallet = metadata.pallet("Tokens")?;
                        let constant = pallet.constant("MaxReserves")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod currencies {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Transfer {
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub currency_id: runtime_types::primitives::currency::CurrencyId,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Currencies";
                const FUNCTION: &'static str = "transfer";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    to: ::subxt::sp_core::crypto::AccountId32,
                    currency_id: runtime_types::primitives::currency::CurrencyId,
                    balance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Transfer>()?
                    };
                    if runtime_call_hash
                        == [
                            203u8, 66u8, 83u8, 131u8, 150u8, 195u8, 194u8, 184u8, 39u8, 125u8,
                            12u8, 140u8, 174u8, 103u8, 110u8, 163u8, 169u8, 79u8, 7u8, 213u8,
                            224u8, 91u8, 225u8, 174u8, 79u8, 88u8, 194u8, 130u8, 58u8, 239u8, 25u8,
                            186u8,
                        ]
                    {
                        let call = Transfer {
                            to,
                            currency_id,
                            balance,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn native_currency_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::primitives::currency::CurrencyId,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Currencies", "NativeCurrencyId")?
                        == [
                            252u8, 171u8, 213u8, 21u8, 228u8, 207u8, 193u8, 86u8, 49u8, 86u8,
                            236u8, 105u8, 193u8, 92u8, 136u8, 67u8, 242u8, 132u8, 63u8, 112u8,
                            132u8, 251u8, 223u8, 254u8, 12u8, 199u8, 254u8, 132u8, 90u8, 0u8, 78u8,
                            10u8,
                        ]
                    {
                        let pallet = metadata.pallet("Currencies")?;
                        let constant = pallet.constant("NativeCurrencyId")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod contract_assets_registry {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RegisterAsset {
                pub asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                pub enabled: ::core::primitive::bool,
            }
            impl ::subxt::Call for RegisterAsset {
                const PALLET: &'static str = "ContractAssetsRegistry";
                const FUNCTION: &'static str = "register_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SuspendAsset {
                pub asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SuspendAsset {
                const PALLET: &'static str = "ContractAssetsRegistry";
                const FUNCTION: &'static str = "suspend_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct UnregisterAsset {
                pub asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for UnregisterAsset {
                const PALLET: &'static str = "ContractAssetsRegistry";
                const FUNCTION: &'static str = "unregister_asset";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn register_asset(
                    &self,
                    asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                    enabled: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        RegisterAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<RegisterAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            141u8, 48u8, 153u8, 105u8, 167u8, 242u8, 49u8, 62u8, 139u8, 79u8, 90u8,
                            242u8, 204u8, 198u8, 149u8, 90u8, 176u8, 34u8, 80u8, 63u8, 5u8, 204u8,
                            111u8, 219u8, 65u8, 238u8, 105u8, 235u8, 138u8, 89u8, 234u8, 68u8,
                        ]
                    {
                        let call = RegisterAsset {
                            asset_contract_address,
                            enabled,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn suspend_asset(
                    &self,
                    asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SuspendAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<SuspendAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            116u8, 81u8, 29u8, 117u8, 125u8, 213u8, 177u8, 8u8, 17u8, 65u8, 69u8,
                            46u8, 125u8, 162u8, 37u8, 110u8, 82u8, 139u8, 231u8, 143u8, 35u8,
                            163u8, 215u8, 116u8, 122u8, 225u8, 159u8, 193u8, 91u8, 221u8, 182u8,
                            14u8,
                        ]
                    {
                        let call = SuspendAsset {
                            asset_contract_address,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn unregister_asset(
                    &self,
                    asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        UnregisterAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<UnregisterAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            5u8, 112u8, 154u8, 170u8, 226u8, 81u8, 203u8, 2u8, 223u8, 182u8, 151u8,
                            77u8, 245u8, 41u8, 56u8, 108u8, 253u8, 48u8, 166u8, 32u8, 51u8, 76u8,
                            154u8, 194u8, 111u8, 168u8, 235u8, 37u8, 59u8, 159u8, 235u8, 16u8,
                        ]
                    {
                        let call = UnregisterAsset {
                            asset_contract_address,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RegisteredAsset<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RegisteredAsset<'_> {
                const PALLET: &'static str = "ContractAssetsRegistry";
                const STORAGE: &'static str = "RegisteredAsset";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn registered_asset(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::bool>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<RegisteredAsset>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                184u8, 248u8, 116u8, 50u8, 114u8, 168u8, 216u8, 135u8, 20u8, 96u8,
                                115u8, 52u8, 38u8, 5u8, 23u8, 133u8, 113u8, 217u8, 77u8, 181u8,
                                66u8, 62u8, 142u8, 180u8, 78u8, 224u8, 95u8, 169u8, 106u8, 175u8,
                                248u8, 131u8,
                            ]
                        {
                            let entry = RegisteredAsset(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                pub fn registered_asset_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, RegisteredAsset<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<RegisteredAsset>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                184u8, 248u8, 116u8, 50u8, 114u8, 168u8, 216u8, 135u8, 20u8, 96u8,
                                115u8, 52u8, 38u8, 5u8, 23u8, 133u8, 113u8, 217u8, 77u8, 181u8,
                                66u8, 62u8, 142u8, 180u8, 78u8, 224u8, 95u8, 169u8, 106u8, 175u8,
                                248u8, 131u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("ContractAssetsRegistry", "PalletId")?
                        == [
                            208u8, 216u8, 172u8, 158u8, 127u8, 141u8, 12u8, 245u8, 111u8, 99u8,
                            195u8, 230u8, 33u8, 81u8, 35u8, 195u8, 96u8, 149u8, 111u8, 237u8,
                            181u8, 73u8, 162u8, 250u8, 214u8, 113u8, 174u8, 179u8, 151u8, 31u8,
                            110u8, 190u8,
                        ]
                    {
                        let pallet = metadata.pallet("ContractAssetsRegistry")?;
                        let constant = pallet.constant("PalletId")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn max_gas(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("ContractAssetsRegistry", "MaxGas")?
                        == [
                            198u8, 72u8, 80u8, 202u8, 213u8, 250u8, 25u8, 249u8, 14u8, 71u8, 15u8,
                            35u8, 104u8, 104u8, 241u8, 239u8, 95u8, 93u8, 122u8, 119u8, 127u8,
                            45u8, 62u8, 221u8, 118u8, 140u8, 37u8, 179u8, 45u8, 192u8, 41u8, 188u8,
                        ]
                    {
                        let pallet = metadata.pallet("ContractAssetsRegistry")?;
                        let constant = pallet.constant("MaxGas")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn contract_debug_flag(
                    &self,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("ContractAssetsRegistry", "ContractDebugFlag")?
                        == [
                            56u8, 170u8, 116u8, 151u8, 31u8, 197u8, 227u8, 237u8, 179u8, 56u8,
                            116u8, 98u8, 6u8, 239u8, 124u8, 52u8, 231u8, 72u8, 5u8, 30u8, 201u8,
                            82u8, 224u8, 185u8, 27u8, 114u8, 246u8, 163u8, 33u8, 171u8, 216u8,
                            218u8,
                        ]
                    {
                        let pallet = metadata.pallet("ContractAssetsRegistry")?;
                        let constant = pallet.constant("ContractDebugFlag")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn next_fee_multiplier(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::sp_arithmetic::fixed_point::FixedU128,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<NextFeeMultiplier>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                232u8, 48u8, 68u8, 202u8, 209u8, 29u8, 249u8, 71u8, 0u8, 84u8,
                                229u8, 250u8, 176u8, 203u8, 27u8, 26u8, 34u8, 55u8, 83u8, 183u8,
                                224u8, 40u8, 62u8, 127u8, 131u8, 88u8, 128u8, 9u8, 56u8, 178u8,
                                31u8, 183u8,
                            ]
                        {
                            let entry = NextFeeMultiplier;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                pub fn storage_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::pallet_transaction_payment::Releases,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<StorageVersion>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8,
                                176u8, 200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8,
                                12u8, 198u8, 58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8,
                                154u8, 228u8,
                            ]
                        {
                            let entry = StorageVersion;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("TransactionPayment", "OperationalFeeMultiplier")?
                        == [
                            161u8, 232u8, 150u8, 43u8, 106u8, 83u8, 56u8, 248u8, 54u8, 123u8,
                            244u8, 73u8, 5u8, 49u8, 245u8, 150u8, 70u8, 92u8, 158u8, 207u8, 127u8,
                            115u8, 211u8, 21u8, 24u8, 136u8, 89u8, 44u8, 151u8, 211u8, 235u8,
                            196u8,
                        ]
                    {
                        let pallet = metadata.pallet("TransactionPayment")?;
                        let constant = pallet.constant("OperationalFeeMultiplier")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The polynomial that is applied in order to derive fee from weight."]
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("TransactionPayment", "WeightToFee")?
                        == [
                            236u8, 89u8, 172u8, 50u8, 101u8, 218u8, 151u8, 158u8, 128u8, 186u8,
                            120u8, 84u8, 103u8, 248u8, 220u8, 191u8, 9u8, 185u8, 114u8, 160u8,
                            104u8, 235u8, 167u8, 83u8, 228u8, 6u8, 56u8, 179u8, 160u8, 4u8, 230u8,
                            12u8,
                        ]
                    {
                        let pallet = metadata.pallet("TransactionPayment")?;
                        let constant = pallet.constant("WeightToFee")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The polynomial that is applied in order to derive fee from length."]
                pub fn length_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("TransactionPayment", "LengthToFee")?
                        == [
                            59u8, 61u8, 190u8, 218u8, 233u8, 30u8, 78u8, 123u8, 149u8, 68u8, 183u8,
                            192u8, 66u8, 208u8, 152u8, 43u8, 182u8, 204u8, 125u8, 116u8, 73u8,
                            60u8, 132u8, 228u8, 206u8, 146u8, 15u8, 119u8, 184u8, 172u8, 68u8,
                            229u8,
                        ]
                    {
                        let pallet = metadata.pallet("TransactionPayment")?;
                        let constant = pallet.constant("LengthToFee")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod fluent_fee {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_fluent_fee::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
    }
    pub mod fee_enablement {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct OnboardAsset {
                pub asset_id: runtime_types::primitives::currency::CurrencyId,
                pub enabled: ::core::primitive::bool,
            }
            impl ::subxt::Call for OnboardAsset {
                const PALLET: &'static str = "FeeEnablement";
                const FUNCTION: &'static str = "onboard_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EnableAsset {
                pub asset_id: runtime_types::primitives::currency::CurrencyId,
            }
            impl ::subxt::Call for EnableAsset {
                const PALLET: &'static str = "FeeEnablement";
                const FUNCTION: &'static str = "enable_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct DisableAsset {
                pub asset_id: runtime_types::primitives::currency::CurrencyId,
            }
            impl ::subxt::Call for DisableAsset {
                const PALLET: &'static str = "FeeEnablement";
                const FUNCTION: &'static str = "disable_asset";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn onboard_asset(
                    &self,
                    asset_id: runtime_types::primitives::currency::CurrencyId,
                    enabled: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        OnboardAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<OnboardAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            137u8, 1u8, 80u8, 220u8, 76u8, 65u8, 40u8, 158u8, 211u8, 58u8, 128u8,
                            178u8, 228u8, 253u8, 96u8, 93u8, 22u8, 210u8, 149u8, 17u8, 96u8, 167u8,
                            255u8, 135u8, 216u8, 117u8, 144u8, 95u8, 119u8, 107u8, 252u8, 42u8,
                        ]
                    {
                        let call = OnboardAsset { asset_id, enabled };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn enable_asset(
                    &self,
                    asset_id: runtime_types::primitives::currency::CurrencyId,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        EnableAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<EnableAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            4u8, 103u8, 98u8, 173u8, 145u8, 67u8, 248u8, 143u8, 41u8, 73u8, 80u8,
                            173u8, 148u8, 225u8, 223u8, 45u8, 144u8, 91u8, 166u8, 40u8, 87u8,
                            170u8, 12u8, 114u8, 10u8, 200u8, 75u8, 152u8, 27u8, 163u8, 126u8, 4u8,
                        ]
                    {
                        let call = EnableAsset { asset_id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn disable_asset(
                    &self,
                    asset_id: runtime_types::primitives::currency::CurrencyId,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DisableAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<DisableAsset>()?
                    };
                    if runtime_call_hash
                        == [
                            67u8, 166u8, 103u8, 108u8, 175u8, 187u8, 238u8, 31u8, 108u8, 207u8,
                            167u8, 242u8, 232u8, 120u8, 51u8, 6u8, 13u8, 47u8, 83u8, 111u8, 147u8,
                            226u8, 62u8, 251u8, 17u8, 118u8, 7u8, 200u8, 188u8, 170u8, 108u8,
                            255u8,
                        ]
                    {
                        let call = DisableAsset { asset_id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct FeeAssets<'a>(pub &'a runtime_types::primitives::currency::CurrencyId);
            impl ::subxt::StorageEntry for FeeAssets<'_> {
                const PALLET: &'static str = "FeeEnablement";
                const STORAGE: &'static str = "FeeAssets";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fee_assets(
                    &self,
                    _0: &'a runtime_types::primitives::currency::CurrencyId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::bool>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<FeeAssets>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                8u8, 253u8, 13u8, 247u8, 229u8, 180u8, 254u8, 125u8, 3u8, 203u8,
                                241u8, 219u8, 224u8, 140u8, 186u8, 172u8, 135u8, 11u8, 46u8, 86u8,
                                50u8, 25u8, 242u8, 98u8, 52u8, 240u8, 149u8, 91u8, 100u8, 125u8,
                                46u8, 233u8,
                            ]
                        {
                            let entry = FeeAssets(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                pub fn fee_assets_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, FeeAssets<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<FeeAssets>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                8u8, 253u8, 13u8, 247u8, 229u8, 180u8, 254u8, 125u8, 3u8, 203u8,
                                241u8, 219u8, 224u8, 140u8, 186u8, 172u8, 135u8, 11u8, 46u8, 86u8,
                                50u8, 25u8, 242u8, 98u8, 52u8, 240u8, 149u8, 91u8, 100u8, 125u8,
                                46u8, 233u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Authorities>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                168u8, 101u8, 224u8, 96u8, 254u8, 152u8, 213u8, 141u8, 46u8, 181u8,
                                131u8, 23u8, 218u8, 24u8, 145u8, 111u8, 161u8, 192u8, 253u8, 29u8,
                                128u8, 92u8, 125u8, 159u8, 242u8, 144u8, 253u8, 174u8, 50u8, 190u8,
                                148u8, 193u8,
                            ]
                        {
                            let entry = Authorities;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::sp_consensus_slots::Slot,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<CurrentSlot>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                233u8, 102u8, 77u8, 99u8, 103u8, 50u8, 151u8, 229u8, 46u8, 226u8,
                                181u8, 37u8, 117u8, 204u8, 234u8, 120u8, 116u8, 166u8, 80u8, 188u8,
                                92u8, 154u8, 137u8, 150u8, 79u8, 164u8, 29u8, 203u8, 2u8, 51u8,
                                123u8, 104u8,
                            ]
                        {
                            let entry = CurrentSlot;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocation,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ReportEquivocation>()?
                    };
                    if runtime_call_hash
                        == [
                            255u8, 59u8, 201u8, 1u8, 171u8, 157u8, 232u8, 62u8, 75u8, 212u8, 86u8,
                            247u8, 132u8, 32u8, 114u8, 38u8, 121u8, 205u8, 61u8, 241u8, 16u8,
                            241u8, 178u8, 191u8, 52u8, 33u8, 34u8, 110u8, 18u8, 6u8, 216u8, 130u8,
                        ]
                    {
                        let call = ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocationUnsigned,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<ReportEquivocationUnsigned>()?
                    };
                    if runtime_call_hash
                        == [
                            193u8, 179u8, 43u8, 34u8, 77u8, 194u8, 203u8, 216u8, 112u8, 101u8,
                            70u8, 127u8, 136u8, 123u8, 163u8, 143u8, 83u8, 137u8, 142u8, 226u8,
                            5u8, 100u8, 225u8, 32u8, 7u8, 195u8, 78u8, 76u8, 85u8, 114u8, 76u8,
                            109u8,
                        ]
                    {
                        let call = ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
                #[doc = "stalled. This will trigger a forced authority set change at the beginning"]
                #[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
                #[doc = "should be high enough to safely assume that the block signalling the"]
                #[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
                #[doc = "will start the new authority set using the given finalized block as base."]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        NoteStalled,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<NoteStalled>()?
                    };
                    if runtime_call_hash
                        == [
                            227u8, 98u8, 249u8, 158u8, 96u8, 124u8, 72u8, 188u8, 27u8, 215u8, 73u8,
                            62u8, 103u8, 79u8, 38u8, 48u8, 212u8, 88u8, 233u8, 187u8, 11u8, 95u8,
                            39u8, 247u8, 55u8, 184u8, 228u8, 102u8, 13u8, 251u8, 52u8, 206u8,
                        ]
                    {
                        let call = NoteStalled {
                            delay,
                            best_finalized_block_number,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession<'_> {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<State>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                159u8, 75u8, 78u8, 23u8, 98u8, 89u8, 239u8, 230u8, 192u8, 67u8,
                                139u8, 222u8, 151u8, 237u8, 216u8, 20u8, 235u8, 247u8, 180u8, 24u8,
                                64u8, 160u8, 58u8, 15u8, 205u8, 191u8, 120u8, 68u8, 32u8, 5u8,
                                161u8, 106u8,
                            ]
                        {
                            let entry = State;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<
                            runtime_types::pallet_grandpa::StoredPendingChange<
                                ::core::primitive::u32,
                            >,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<PendingChange>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                128u8, 176u8, 209u8, 41u8, 231u8, 111u8, 205u8, 198u8, 154u8, 44u8,
                                228u8, 231u8, 44u8, 110u8, 74u8, 9u8, 31u8, 86u8, 128u8, 244u8,
                                112u8, 21u8, 120u8, 176u8, 50u8, 213u8, 122u8, 46u8, 85u8, 255u8,
                                40u8, 173u8,
                            ]
                        {
                            let entry = PendingChange;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::u32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<NextForced>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8,
                                67u8, 6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8,
                                19u8, 169u8, 196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8,
                                80u8, 156u8,
                            ]
                        {
                            let entry = NextForced;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Stalled>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8,
                                186u8, 189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8,
                                231u8, 109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8,
                                24u8, 185u8,
                            ]
                        {
                            let entry = Stalled;
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<CurrentSetId>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8,
                                20u8, 178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8,
                                105u8, 52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8,
                                7u8, 108u8,
                            ]
                        {
                            let entry = CurrentSetId;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: &'a ::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::core::primitive::u32>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<SetIdSession>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                                166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                                108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8,
                                53u8, 134u8,
                            ]
                        {
                            let entry = SetIdSession(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<SetIdSession>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                                166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                                108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8,
                                53u8, 134u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Grandpa", "MaxAuthorities")?
                        == [
                            80u8, 201u8, 93u8, 114u8, 100u8, 205u8, 172u8, 38u8, 174u8, 71u8,
                            187u8, 161u8, 148u8, 83u8, 7u8, 73u8, 176u8, 100u8, 128u8, 71u8, 233u8,
                            163u8, 89u8, 171u8, 100u8, 247u8, 111u8, 44u8, 173u8, 82u8, 34u8,
                            159u8,
                        ]
                    {
                        let pallet = metadata.pallet("Grandpa")?;
                        let constant = pallet.constant("MaxAuthorities")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Call {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Call {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "call";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct InstantiateWithCode {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for InstantiateWithCode {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate_with_code";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Instantiate {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                pub code_hash: ::subxt::sp_core::H256,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Instantiate {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct UploadCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
            }
            impl ::subxt::Call for UploadCode {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "upload_code";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RemoveCode {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RemoveCode {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "remove_code";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Makes a call to an account, optionally transferring some balance."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `dest`: Address of the contract to call."]
                #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                #[doc = "  caller to pay for the storage consumed."]
                #[doc = "* `data`: The input data to pass to the contract."]
                #[doc = ""]
                #[doc = "* If the account is a smart-contract account, the associated code will be"]
                #[doc = "executed and any value will be transferred."]
                #[doc = "* If the account is a regular account, any value will be transferred."]
                #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                #[doc = "a regular account will be created and any value will be transferred."]
                pub fn call(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Call, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Call>()?
                    };
                    if runtime_call_hash
                        == [
                            106u8, 173u8, 84u8, 196u8, 53u8, 245u8, 62u8, 134u8, 187u8, 142u8,
                            111u8, 146u8, 6u8, 35u8, 112u8, 181u8, 14u8, 189u8, 228u8, 161u8, 14u8,
                            167u8, 17u8, 146u8, 229u8, 21u8, 46u8, 195u8, 182u8, 109u8, 134u8,
                            132u8,
                        ]
                    {
                        let call = Call {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                #[doc = "some balance."]
                #[doc = ""]
                #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                #[doc = "also check the documentation of [`Self::upload_code`]."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                #[doc = "  from the caller to pay for the storage consumed."]
                #[doc = "* `code`: The contract code to deploy in raw bytes."]
                #[doc = "* `data`: The input data to pass to the contract constructor."]
                #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                #[doc = ""]
                #[doc = "Instantiation is executed as follows:"]
                #[doc = ""]
                #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                #[doc = "  code."]
                #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                #[doc = "- The smart-contract account is created at the computed address."]
                #[doc = "- The `value` is transferred to the new account."]
                #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                pub fn instantiate_with_code(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        InstantiateWithCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<InstantiateWithCode>()?
                    };
                    if runtime_call_hash
                        == [
                            155u8, 201u8, 165u8, 13u8, 34u8, 70u8, 0u8, 24u8, 118u8, 39u8, 15u8,
                            9u8, 85u8, 36u8, 75u8, 100u8, 130u8, 71u8, 86u8, 162u8, 210u8, 69u8,
                            62u8, 212u8, 201u8, 185u8, 103u8, 167u8, 32u8, 97u8, 93u8, 156u8,
                        ]
                    {
                        let call = InstantiateWithCode {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                #[doc = ""]
                #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                #[doc = "must be supplied."]
                pub fn instantiate(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                    code_hash: ::subxt::sp_core::H256,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Instantiate,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<Instantiate>()?
                    };
                    if runtime_call_hash
                        == [
                            189u8, 13u8, 4u8, 51u8, 168u8, 118u8, 125u8, 60u8, 62u8, 150u8, 99u8,
                            55u8, 185u8, 243u8, 27u8, 235u8, 249u8, 214u8, 226u8, 230u8, 105u8,
                            176u8, 223u8, 188u8, 249u8, 204u8, 219u8, 103u8, 175u8, 124u8, 179u8,
                            39u8,
                        ]
                    {
                        let call = Instantiate {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Upload new `code` without instantiating a contract from it."]
                #[doc = ""]
                #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                #[doc = "depends on the instrumented size of the the supplied `code`."]
                #[doc = ""]
                #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                #[doc = "the in storage version to the current"]
                #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                #[doc = "through [`Self::instantiate_with_code`]."]
                pub fn upload_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        UploadCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<UploadCode>()?
                    };
                    if runtime_call_hash
                        == [
                            154u8, 75u8, 72u8, 46u8, 121u8, 200u8, 72u8, 230u8, 29u8, 231u8, 198u8,
                            4u8, 67u8, 35u8, 201u8, 181u8, 70u8, 66u8, 140u8, 194u8, 211u8, 157u8,
                            218u8, 201u8, 170u8, 48u8, 74u8, 105u8, 57u8, 136u8, 175u8, 55u8,
                        ]
                    {
                        let call = UploadCode {
                            code,
                            storage_deposit_limit,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                #[doc = ""]
                #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                #[doc = "not used by any contract."]
                pub fn remove_code(
                    &self,
                    code_hash: ::subxt::sp_core::H256,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        RemoveCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<RemoveCode>()?
                    };
                    if runtime_call_hash
                        == [
                            61u8, 95u8, 174u8, 58u8, 44u8, 132u8, 156u8, 127u8, 117u8, 68u8, 66u8,
                            91u8, 91u8, 132u8, 126u8, 110u8, 78u8, 177u8, 192u8, 13u8, 125u8, 90u8,
                            219u8, 140u8, 72u8, 101u8, 158u8, 149u8, 187u8, 140u8, 241u8, 52u8,
                        ]
                    {
                        let call = RemoveCode { code_hash };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Contract deployed by address at the specified address."]
            pub struct Instantiated {
                pub deployer: ::subxt::sp_core::crypto::AccountId32,
                pub contract: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Contract has been removed."]
            #[doc = ""]
            #[doc = "# Note"]
            #[doc = ""]
            #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
            #[doc = "`seal_terminate`."]
            pub struct Terminated {
                pub contract: ::subxt::sp_core::crypto::AccountId32,
                pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Code with the specified hash has been stored."]
            pub struct CodeStored {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A custom event emitted by the contract."]
            pub struct ContractEmitted {
                pub contract: ::subxt::sp_core::crypto::AccountId32,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A code with the specified hash was removed."]
            pub struct CodeRemoved {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A contract's code was updated."]
            pub struct ContractCodeUpdated {
                pub contract: ::subxt::sp_core::crypto::AccountId32,
                pub new_code_hash: ::subxt::sp_core::H256,
                pub old_code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for ContractCodeUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractCodeUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PristineCode<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for PristineCode<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "PristineCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeStorage<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for CodeStorage<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "CodeStorage";
                type Value = runtime_types::pallet_contracts::wasm::PrefabWasmModule;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct OwnerInfoOf<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for OwnerInfoOf<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "OwnerInfoOf";
                type Value = runtime_types::pallet_contracts::wasm::OwnerInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Nonce;
            impl ::subxt::StorageEntry for Nonce {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "Nonce";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ContractInfoOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ContractInfoOf<'_> {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "ContractInfoOf";
                type Value = runtime_types::pallet_contracts::storage::RawContractInfo<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DeletionQueue;
            impl ::subxt::StorageEntry for DeletionQueue {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "DeletionQueue";
                type Value =
                    ::std::vec::Vec<runtime_types::pallet_contracts::storage::DeletedContract>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code(
                    &self,
                    _0: &'a ::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<PristineCode>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                127u8, 37u8, 115u8, 151u8, 42u8, 7u8, 36u8, 60u8, 41u8, 190u8,
                                142u8, 209u8, 66u8, 110u8, 45u8, 192u8, 172u8, 177u8, 151u8, 216u8,
                                227u8, 211u8, 156u8, 119u8, 102u8, 119u8, 93u8, 23u8, 35u8, 17u8,
                                49u8, 85u8,
                            ]
                        {
                            let entry = PristineCode(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, PristineCode<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<PristineCode>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                127u8, 37u8, 115u8, 151u8, 42u8, 7u8, 36u8, 60u8, 41u8, 190u8,
                                142u8, 209u8, 66u8, 110u8, 45u8, 192u8, 172u8, 177u8, 151u8, 216u8,
                                227u8, 211u8, 156u8, 119u8, 102u8, 119u8, 93u8, 23u8, 35u8, 17u8,
                                49u8, 85u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage(
                    &self,
                    _0: &'a ::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<
                            runtime_types::pallet_contracts::wasm::PrefabWasmModule,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<CodeStorage>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                137u8, 128u8, 184u8, 8u8, 144u8, 135u8, 177u8, 16u8, 13u8, 53u8,
                                126u8, 61u8, 246u8, 19u8, 157u8, 227u8, 213u8, 105u8, 117u8, 136u8,
                                251u8, 222u8, 141u8, 234u8, 233u8, 205u8, 241u8, 237u8, 224u8,
                                104u8, 71u8, 10u8,
                            ]
                        {
                            let entry = CodeStorage(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, CodeStorage<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<CodeStorage>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                137u8, 128u8, 184u8, 8u8, 144u8, 135u8, 177u8, 16u8, 13u8, 53u8,
                                126u8, 61u8, 246u8, 19u8, 157u8, 227u8, 213u8, 105u8, 117u8, 136u8,
                                251u8, 222u8, 141u8, 234u8, 233u8, 205u8, 241u8, 237u8, 224u8,
                                104u8, 71u8, 10u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of(
                    &self,
                    _0: &'a ::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<runtime_types::pallet_contracts::wasm::OwnerInfo>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<OwnerInfoOf>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                19u8, 83u8, 201u8, 115u8, 173u8, 47u8, 224u8, 40u8, 215u8, 46u8,
                                131u8, 182u8, 197u8, 47u8, 29u8, 112u8, 35u8, 230u8, 27u8, 0u8,
                                231u8, 72u8, 220u8, 49u8, 86u8, 210u8, 45u8, 170u8, 238u8, 160u8,
                                140u8, 14u8,
                            ]
                        {
                            let entry = OwnerInfoOf(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, OwnerInfoOf<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<OwnerInfoOf>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                19u8, 83u8, 201u8, 115u8, 173u8, 47u8, 224u8, 40u8, 215u8, 46u8,
                                131u8, 182u8, 197u8, 47u8, 29u8, 112u8, 35u8, 230u8, 27u8, 0u8,
                                231u8, 72u8, 220u8, 49u8, 86u8, 210u8, 45u8, 170u8, 238u8, 160u8,
                                140u8, 14u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " This is a **monotonic** counter incremented on contract instantiation."]
                #[doc = ""]
                #[doc = " This is used in order to generate unique trie ids for contracts."]
                #[doc = " The trie id of a new contract is calculated from hash(account_id, nonce)."]
                #[doc = " The nonce is required because otherwise the following sequence would lead to"]
                #[doc = " a possible collision of storage:"]
                #[doc = ""]
                #[doc = " 1. Create a new contract."]
                #[doc = " 2. Terminate the contract."]
                #[doc = " 3. Immediately recreate the contract with the same account_id."]
                #[doc = ""]
                #[doc = " This is bad because the contents of a trie are deleted lazily and there might be"]
                #[doc = " storage of the old instantiation still in it when the new contract is created. Please"]
                #[doc = " note that we can't replace the counter by the block number because the sequence above"]
                #[doc = " can happen in the same block. We also can't keep the account counter in memory only"]
                #[doc = " because storage is the only way to communicate across different extrinsics in the"]
                #[doc = " same block."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Do not use it to determine the number of contracts. It won't be decremented if"]
                #[doc = " a contract is destroyed."]
                pub fn nonce(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<Nonce>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                122u8, 169u8, 95u8, 131u8, 85u8, 32u8, 154u8, 114u8, 143u8, 56u8,
                                12u8, 182u8, 64u8, 150u8, 241u8, 249u8, 254u8, 251u8, 160u8, 235u8,
                                192u8, 41u8, 101u8, 232u8, 186u8, 108u8, 187u8, 149u8, 210u8, 91u8,
                                179u8, 98u8,
                            ]
                        {
                            let entry = Nonce;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of(
                    &self,
                    _0: &'a ::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::core::option::Option<
                            runtime_types::pallet_contracts::storage::RawContractInfo<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u128,
                            >,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ContractInfoOf>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                168u8, 140u8, 207u8, 90u8, 218u8, 31u8, 75u8, 19u8, 184u8, 168u8,
                                140u8, 69u8, 212u8, 73u8, 62u8, 149u8, 21u8, 208u8, 196u8, 49u8,
                                162u8, 7u8, 41u8, 9u8, 143u8, 130u8, 84u8, 204u8, 154u8, 39u8,
                                146u8, 169u8,
                            ]
                        {
                            let entry = ContractInfoOf(_0);
                            client.storage().fetch(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::subxt::KeyIter<'a, T, ContractInfoOf<'a>>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<ContractInfoOf>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                168u8, 140u8, 207u8, 90u8, 218u8, 31u8, 75u8, 19u8, 184u8, 168u8,
                                140u8, 69u8, 212u8, 73u8, 62u8, 149u8, 21u8, 208u8, 196u8, 49u8,
                                162u8, 7u8, 41u8, 9u8, 143u8, 130u8, 84u8, 204u8, 154u8, 39u8,
                                146u8, 169u8,
                            ]
                        {
                            client.storage().iter(block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
                #[doc = " Evicted contracts that await child trie deletion."]
                #[doc = ""]
                #[doc = " Child trie deletion is a heavy operation depending on the amount of storage items"]
                #[doc = " stored in said trie. Therefore this operation is performed lazily in `on_initialize`."]
                pub fn deletion_queue(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        ::std::vec::Vec<runtime_types::pallet_contracts::storage::DeletedContract>,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<DeletionQueue>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                128u8, 1u8, 152u8, 100u8, 164u8, 50u8, 180u8, 212u8, 12u8, 13u8,
                                212u8, 23u8, 73u8, 74u8, 25u8, 203u8, 44u8, 42u8, 255u8, 217u8,
                                197u8, 50u8, 181u8, 187u8, 249u8, 188u8, 76u8, 63u8, 192u8, 165u8,
                                215u8, 10u8,
                            ]
                        {
                            let entry = DeletionQueue;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Cost schedule and limits."]
                pub fn schedule(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::pallet_contracts::schedule::Schedule,
                    ::subxt::BasicError,
                > {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "Schedule")?
                        == [
                            30u8, 158u8, 222u8, 25u8, 84u8, 128u8, 123u8, 63u8, 244u8, 129u8, 77u8,
                            243u8, 104u8, 63u8, 19u8, 240u8, 84u8, 73u8, 5u8, 203u8, 141u8, 61u8,
                            61u8, 160u8, 232u8, 225u8, 212u8, 228u8, 94u8, 203u8, 239u8, 250u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("Schedule")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of contracts that can be pending for deletion."]
                #[doc = ""]
                #[doc = " When a contract is deleted by calling `seal_terminate` it becomes inaccessible"]
                #[doc = " immediately, but the deletion of the storage items it has accumulated is performed"]
                #[doc = " later. The contract is put into the deletion queue. This defines how many"]
                #[doc = " contracts can be queued up at the same time. If that limit is reached `seal_terminate`"]
                #[doc = " will fail. The action must be retried in a later block in that case."]
                #[doc = ""]
                #[doc = " The reasons for limiting the queue depth are:"]
                #[doc = ""]
                #[doc = " 1. The queue is in storage in order to be persistent between blocks. We want to limit"]
                #[doc = " \tthe amount of storage that can be consumed."]
                #[doc = " 2. The queue is stored in a vector and needs to be decoded as a whole when reading"]
                #[doc = "\t\tit at the end of each block. Longer queues take more weight to decode and hence"]
                #[doc = "\t\tlimit the amount of items that can be deleted per block."]
                pub fn deletion_queue_depth(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "DeletionQueueDepth")?
                        == [
                            49u8, 84u8, 123u8, 112u8, 161u8, 159u8, 147u8, 82u8, 66u8, 152u8,
                            109u8, 254u8, 230u8, 67u8, 207u8, 52u8, 83u8, 238u8, 173u8, 92u8,
                            181u8, 253u8, 99u8, 195u8, 187u8, 184u8, 0u8, 187u8, 224u8, 226u8,
                            128u8, 245u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("DeletionQueueDepth")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum amount of weight that can be consumed per block for lazy trie removal."]
                #[doc = ""]
                #[doc = " The amount of weight that is dedicated per block to work on the deletion queue. Larger"]
                #[doc = " values allow more trie keys to be deleted in each block but reduce the amount of"]
                #[doc = " weight that is left for transactions. See [`Self::DeletionQueueDepth`] for more"]
                #[doc = " information about the deletion queue."]
                pub fn deletion_weight_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "DeletionWeightLimit")?
                        == [
                            88u8, 41u8, 185u8, 176u8, 168u8, 133u8, 38u8, 126u8, 226u8, 214u8,
                            47u8, 114u8, 19u8, 170u8, 186u8, 65u8, 95u8, 2u8, 90u8, 189u8, 29u8,
                            210u8, 84u8, 166u8, 148u8, 216u8, 199u8, 15u8, 151u8, 21u8, 173u8,
                            223u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("DeletionWeightLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The amount of balance a caller has to pay for each byte of storage."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "DepositPerByte")?
                        == [
                            202u8, 189u8, 26u8, 57u8, 89u8, 78u8, 68u8, 253u8, 20u8, 191u8, 209u8,
                            44u8, 238u8, 231u8, 96u8, 10u8, 19u8, 178u8, 119u8, 7u8, 171u8, 158u8,
                            148u8, 178u8, 156u8, 66u8, 119u8, 176u8, 228u8, 238u8, 78u8, 147u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("DepositPerByte")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The weight per byte of code that is charged when loading a contract from storage."]
                #[doc = ""]
                #[doc = " Currently, FRAME only charges fees for computation incurred but not for PoV"]
                #[doc = " consumption caused for storage access. This is usually not exploitable because"]
                #[doc = " accessing storage carries some substantial weight costs, too. However in case"]
                #[doc = " of contract code very much PoV consumption can be caused while consuming very little"]
                #[doc = " computation. This could be used to keep the chain busy without paying the"]
                #[doc = " proper fee for it. Until this is resolved we charge from the weight meter for"]
                #[doc = " contract access."]
                #[doc = ""]
                #[doc = " For more information check out: <https://github.com/paritytech/substrate/issues/10301>"]
                #[doc = ""]
                #[doc = " [`DefaultContractAccessWeight`] is a safe default to be used for polkadot or kusama"]
                #[doc = " parachains."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " This is only relevant for parachains. Set to zero in case of a standalone chain."]
                pub fn contract_access_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "ContractAccessWeight")?
                        == [
                            32u8, 104u8, 38u8, 223u8, 54u8, 204u8, 26u8, 64u8, 231u8, 103u8, 50u8,
                            250u8, 95u8, 218u8, 25u8, 248u8, 124u8, 154u8, 113u8, 84u8, 35u8,
                            223u8, 36u8, 105u8, 140u8, 131u8, 157u8, 39u8, 75u8, 11u8, 29u8, 206u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("ContractAccessWeight")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The amount of balance a caller has to pay for each storage item."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_item(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    if metadata.constant_hash("Contracts", "DepositPerItem")?
                        == [
                            192u8, 138u8, 22u8, 95u8, 20u8, 69u8, 245u8, 223u8, 2u8, 72u8, 34u8,
                            109u8, 210u8, 249u8, 155u8, 72u8, 181u8, 166u8, 58u8, 156u8, 114u8,
                            60u8, 108u8, 200u8, 6u8, 23u8, 24u8, 175u8, 19u8, 74u8, 210u8, 228u8,
                        ]
                    {
                        let pallet = metadata.pallet("Contracts")?;
                        let constant = pallet.constant("DepositPerItem")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> impl ::core::future::Future<
                    Output = ::core::result::Result<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::sp_core::H256,
                        >,
                        ::subxt::BasicError,
                    >,
                > + 'a {
                    let client = self.client;
                    async move {
                        let runtime_storage_hash = {
                            let locked_metadata = client.metadata();
                            let metadata = locked_metadata.read();
                            match metadata.storage_hash::<RandomMaterial>() {
                                Ok(hash) => hash,
                                Err(e) => return Err(e.into()),
                            }
                        };
                        if runtime_storage_hash
                            == [
                                60u8, 176u8, 119u8, 155u8, 161u8, 136u8, 144u8, 88u8, 26u8, 57u8,
                                142u8, 34u8, 5u8, 37u8, 115u8, 11u8, 90u8, 222u8, 147u8, 194u8,
                                82u8, 194u8, 70u8, 227u8, 175u8, 198u8, 235u8, 24u8, 7u8, 87u8,
                                203u8, 182u8,
                            ]
                        {
                            let entry = RandomMaterial;
                            client.storage().fetch_or_default(&entry, block_hash).await
                        } else {
                            Err(::subxt::MetadataError::IncompatibleMetadata.into())
                        }
                    }
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod schedule {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum LookupError {
                        #[codec(index = 0)]
                        Unknown,
                        #[codec(index = 1)]
                        BadFormat,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum MaybeHashed<_0, _1> {
                        #[codec(index = 0)]
                        Value(_0),
                        #[codec(index = 1)]
                        Hash(_1),
                    }
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod laguna_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 2)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 3)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 4)]
                Tokens(runtime_types::orml_tokens::module::Call),
                #[codec(index = 5)]
                Currencies(runtime_types::pallet_currencies::pallet::Call),
                #[codec(index = 6)]
                ContractAssetsRegistry(runtime_types::pallet_contract_asset_registry::pallet::Call),
                #[codec(index = 9)]
                FeeEnablement(runtime_types::pallet_fee_enablement::pallet::Call),
                #[codec(index = 11)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 12)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 2)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 3)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 4)]
                Tokens(runtime_types::orml_tokens::module::Event),
                #[codec(index = 8)]
                FluentFee(runtime_types::pallet_fluent_fee::pallet::Event),
                #[codec(index = 11)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 12)]
                Contracts(runtime_types::pallet_contracts::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 1)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Runtime;
        }
        pub mod orml_tokens {
            use super::runtime_types;
            pub mod module {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "It will decrease the total issuance of the system by the"]
                    #[doc = "`TransferFee`. If the sender's account is below the existential"]
                    #[doc = "deposit as a result of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the"]
                    #[doc = "transactor."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `currency_id`: currency type."]
                    #[doc = "- `amount`: free balance amount to tranfer."]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Transfer all remaining balance to the given account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_"]
                    #[doc = "balances. This means that any locked, reserved, or existential"]
                    #[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
                    #[doc = "this function. To ensure that this function results in a killed"]
                    #[doc = "account, you might need to prepare the account by removing any"]
                    #[doc = "reference counters, storage deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the"]
                    #[doc = "transactor."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `currency_id`: currency type."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
                    #[doc = "  operation should send all of the funds the account has, causing"]
                    #[doc = "  the sender account to be killed (false), or transfer everything"]
                    #[doc = "  except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
                    #[doc = "will not kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the"]
                    #[doc = "transactor."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `currency_id`: currency type."]
                    #[doc = "- `amount`: free balance amount to tranfer."]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source"]
                    #[doc = "account may be specified."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "- `source`: The sender of the transfer."]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `currency_id`: currency type."]
                    #[doc = "- `amount`: free balance amount to tranfer."]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
                    #[doc = "will also decrease the total issuance of the system"]
                    #[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
                    #[doc = "existential deposit, it will reap the `AccountInfo`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The balance is too low"]
                    BalanceTooLow,
                    #[codec(index = 1)]
                    #[doc = "Cannot convert Amount into Balance type"]
                    AmountIntoBalanceFailed,
                    #[codec(index = 2)]
                    #[doc = "Failed because liquidity restrictions due to locking"]
                    LiquidityRestrictions,
                    #[codec(index = 3)]
                    #[doc = "Failed because the maximum locks was exceeded"]
                    MaxLocksExceeded,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below"]
                    #[doc = "ExistentialDeposit, resulting in an outright loss."]
                    DustLost {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some reserved balance was repatriated (moved from reserved to"]
                    #[doc = "another account)."]
                    ReserveRepatriated {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 6)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "The total issuance of an currency has been set"]
                    TotalIssuanceSet {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some balances were withdrawn (e.g. pay for transaction fee)"]
                    Withdrawn {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some balances were slashed (e.g. due to mis-behavior)"]
                    Slashed {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free_amount: ::core::primitive::u128,
                        reserved_amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Deposited some balance into an account"]
                    Deposited {
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some funds are locked"]
                    LockSet {
                        lock_id: [::core::primitive::u8; 8usize],
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some locked funds were unlocked"]
                    LockRemoved {
                        lock_id: [::core::primitive::u8; 8usize],
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub frozen: _0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
        }
        pub mod pallet_contract_asset_registry {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    register_asset {
                        asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                        enabled: ::core::primitive::bool,
                    },
                    #[codec(index = 1)]
                    suspend_asset {
                        asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    unregister_asset {
                        asset_contract_address: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidAsset,
                }
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Makes a call to an account, optionally transferring some balance."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `dest`: Address of the contract to call."]
                    #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                    #[doc = "  caller to pay for the storage consumed."]
                    #[doc = "* `data`: The input data to pass to the contract."]
                    #[doc = ""]
                    #[doc = "* If the account is a smart-contract account, the associated code will be"]
                    #[doc = "executed and any value will be transferred."]
                    #[doc = "* If the account is a regular account, any value will be transferred."]
                    #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                    #[doc = "a regular account will be created and any value will be transferred."]
                    call {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                    #[doc = "some balance."]
                    #[doc = ""]
                    #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                    #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                    #[doc = "also check the documentation of [`Self::upload_code`]."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                    #[doc = "  from the caller to pay for the storage consumed."]
                    #[doc = "* `code`: The contract code to deploy in raw bytes."]
                    #[doc = "* `data`: The input data to pass to the contract constructor."]
                    #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                    #[doc = ""]
                    #[doc = "Instantiation is executed as follows:"]
                    #[doc = ""]
                    #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                    #[doc = "  code."]
                    #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                    #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                    #[doc = "- The smart-contract account is created at the computed address."]
                    #[doc = "- The `value` is transferred to the new account."]
                    #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                    instantiate_with_code {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                    #[doc = ""]
                    #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                    #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                    #[doc = "must be supplied."]
                    instantiate {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                        code_hash: ::subxt::sp_core::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Upload new `code` without instantiating a contract from it."]
                    #[doc = ""]
                    #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                    #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                    #[doc = "depends on the instrumented size of the the supplied `code`."]
                    #[doc = ""]
                    #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                    #[doc = "the in storage version to the current"]
                    #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                    #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                    #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                    #[doc = "through [`Self::instantiate_with_code`]."]
                    upload_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        storage_deposit_limit: ::core::option::Option<::core::primitive::u128>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                    #[doc = ""]
                    #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                    #[doc = "not used by any contract."]
                    remove_code { code_hash: ::subxt::sp_core::H256 },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "A new schedule must have a greater version than the current one."]
                    InvalidScheduleVersion,
                    #[codec(index = 1)]
                    #[doc = "Invalid combination of flags supplied to `seal_call` or `seal_delegate_call`."]
                    InvalidCallFlags,
                    #[codec(index = 2)]
                    #[doc = "The executed contract exhausted its gas limit."]
                    OutOfGas,
                    #[codec(index = 3)]
                    #[doc = "The output buffer supplied to a contract API call was too small."]
                    OutputBufferTooSmall,
                    #[codec(index = 4)]
                    #[doc = "Performing the requested transfer failed. Probably because there isn't enough"]
                    #[doc = "free balance in the sender's account."]
                    TransferFailed,
                    #[codec(index = 5)]
                    #[doc = "Performing a call was denied because the calling depth reached the limit"]
                    #[doc = "of what is specified in the schedule."]
                    MaxCallDepthReached,
                    #[codec(index = 6)]
                    #[doc = "No contract was found at the specified address."]
                    ContractNotFound,
                    #[codec(index = 7)]
                    #[doc = "The code supplied to `instantiate_with_code` exceeds the limit specified in the"]
                    #[doc = "current schedule."]
                    CodeTooLarge,
                    #[codec(index = 8)]
                    #[doc = "No code could be found at the supplied code hash."]
                    CodeNotFound,
                    #[codec(index = 9)]
                    #[doc = "A buffer outside of sandbox memory was passed to a contract API function."]
                    OutOfBounds,
                    #[codec(index = 10)]
                    #[doc = "Input passed to a contract API function failed to decode as expected type."]
                    DecodingFailed,
                    #[codec(index = 11)]
                    #[doc = "Contract trapped during execution."]
                    ContractTrapped,
                    #[codec(index = 12)]
                    #[doc = "The size defined in `T::MaxValueSize` was exceeded."]
                    ValueTooLarge,
                    #[codec(index = 13)]
                    #[doc = "Termination of a contract is not allowed while the contract is already"]
                    #[doc = "on the call stack. Can be triggered by `seal_terminate`."]
                    TerminatedWhileReentrant,
                    #[codec(index = 14)]
                    #[doc = "`seal_call` forwarded this contracts input. It therefore is no longer available."]
                    InputForwarded,
                    #[codec(index = 15)]
                    #[doc = "The subject passed to `seal_random` exceeds the limit."]
                    RandomSubjectTooLong,
                    #[codec(index = 16)]
                    #[doc = "The amount of topics passed to `seal_deposit_events` exceeds the limit."]
                    TooManyTopics,
                    #[codec(index = 17)]
                    #[doc = "The topics passed to `seal_deposit_events` contains at least one duplicate."]
                    DuplicateTopics,
                    #[codec(index = 18)]
                    #[doc = "The chain does not provide a chain extension. Calling the chain extension results"]
                    #[doc = "in this error. Note that this usually  shouldn't happen as deploying such contracts"]
                    #[doc = "is rejected."]
                    NoChainExtension,
                    #[codec(index = 19)]
                    #[doc = "Removal of a contract failed because the deletion queue is full."]
                    #[doc = ""]
                    #[doc = "This can happen when calling `seal_terminate`."]
                    #[doc = "The queue is filled by deleting contracts and emptied by a fixed amount each block."]
                    #[doc = "Trying again during another block is the only way to resolve this issue."]
                    DeletionQueueFull,
                    #[codec(index = 20)]
                    #[doc = "A contract with the same AccountId already exists."]
                    DuplicateContract,
                    #[codec(index = 21)]
                    #[doc = "A contract self destructed in its constructor."]
                    #[doc = ""]
                    #[doc = "This can be triggered by a call to `seal_terminate`."]
                    TerminatedInConstructor,
                    #[codec(index = 22)]
                    #[doc = "The debug message specified to `seal_debug_message` does contain invalid UTF-8."]
                    DebugMessageInvalidUTF8,
                    #[codec(index = 23)]
                    #[doc = "A call tried to invoke a contract that is flagged as non-reentrant."]
                    ReentranceDenied,
                    #[codec(index = 24)]
                    #[doc = "Origin doesn't have enough balance to pay the required storage deposits."]
                    StorageDepositNotEnoughFunds,
                    #[codec(index = 25)]
                    #[doc = "More storage was created than allowed by the storage deposit limit."]
                    StorageDepositLimitExhausted,
                    #[codec(index = 26)]
                    #[doc = "Code removal was denied because the code is still in use by at least one contract."]
                    CodeInUse,
                    #[codec(index = 27)]
                    #[doc = "The contract ran to completion but decided to revert its storage changes."]
                    #[doc = "Please note that this error is only returned from extrinsics. When called directly"]
                    #[doc = "or via RPC an `Ok` will be returned. In this case the caller needs to inspect the flags"]
                    #[doc = "to determine whether a reversion has taken place."]
                    ContractReverted,
                    #[codec(index = 28)]
                    #[doc = "The contract's code was found to be invalid during validation or instrumentation."]
                    #[doc = "A more detailed error can be found on the node console if debug messages are enabled"]
                    #[doc = "or in the debug buffer which is returned to RPC clients."]
                    CodeRejected,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Contract deployed by address at the specified address."]
                    Instantiated {
                        deployer: ::subxt::sp_core::crypto::AccountId32,
                        contract: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Contract has been removed."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
                    #[doc = "`seal_terminate`."]
                    Terminated {
                        contract: ::subxt::sp_core::crypto::AccountId32,
                        beneficiary: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Code with the specified hash has been stored."]
                    CodeStored { code_hash: ::subxt::sp_core::H256 },
                    #[codec(index = 3)]
                    #[doc = "A custom event emitted by the contract."]
                    ContractEmitted {
                        contract: ::subxt::sp_core::crypto::AccountId32,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "A code with the specified hash was removed."]
                    CodeRemoved { code_hash: ::subxt::sp_core::H256 },
                    #[codec(index = 5)]
                    #[doc = "A contract's code was updated."]
                    ContractCodeUpdated {
                        contract: ::subxt::sp_core::crypto::AccountId32,
                        new_code_hash: ::subxt::sp_core::H256,
                        old_code_hash: ::subxt::sp_core::H256,
                    },
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct HostFnWeights {
                    pub caller: ::core::primitive::u64,
                    pub is_contract: ::core::primitive::u64,
                    pub code_hash: ::core::primitive::u64,
                    pub own_code_hash: ::core::primitive::u64,
                    pub caller_is_origin: ::core::primitive::u64,
                    pub address: ::core::primitive::u64,
                    pub gas_left: ::core::primitive::u64,
                    pub balance: ::core::primitive::u64,
                    pub value_transferred: ::core::primitive::u64,
                    pub minimum_balance: ::core::primitive::u64,
                    pub block_number: ::core::primitive::u64,
                    pub now: ::core::primitive::u64,
                    pub weight_to_fee: ::core::primitive::u64,
                    pub gas: ::core::primitive::u64,
                    pub input: ::core::primitive::u64,
                    pub input_per_byte: ::core::primitive::u64,
                    pub r#return: ::core::primitive::u64,
                    pub return_per_byte: ::core::primitive::u64,
                    pub terminate: ::core::primitive::u64,
                    pub random: ::core::primitive::u64,
                    pub deposit_event: ::core::primitive::u64,
                    pub deposit_event_per_topic: ::core::primitive::u64,
                    pub deposit_event_per_byte: ::core::primitive::u64,
                    pub debug_message: ::core::primitive::u64,
                    pub set_storage: ::core::primitive::u64,
                    pub set_storage_per_new_byte: ::core::primitive::u64,
                    pub set_storage_per_old_byte: ::core::primitive::u64,
                    pub set_code_hash: ::core::primitive::u64,
                    pub clear_storage: ::core::primitive::u64,
                    pub clear_storage_per_byte: ::core::primitive::u64,
                    pub contains_storage: ::core::primitive::u64,
                    pub contains_storage_per_byte: ::core::primitive::u64,
                    pub get_storage: ::core::primitive::u64,
                    pub get_storage_per_byte: ::core::primitive::u64,
                    pub take_storage: ::core::primitive::u64,
                    pub take_storage_per_byte: ::core::primitive::u64,
                    pub transfer: ::core::primitive::u64,
                    pub call: ::core::primitive::u64,
                    pub delegate_call: ::core::primitive::u64,
                    pub call_transfer_surcharge: ::core::primitive::u64,
                    pub call_per_cloned_byte: ::core::primitive::u64,
                    pub instantiate: ::core::primitive::u64,
                    pub instantiate_transfer_surcharge: ::core::primitive::u64,
                    pub instantiate_per_salt_byte: ::core::primitive::u64,
                    pub hash_sha2_256: ::core::primitive::u64,
                    pub hash_sha2_256_per_byte: ::core::primitive::u64,
                    pub hash_keccak_256: ::core::primitive::u64,
                    pub hash_keccak_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_256: ::core::primitive::u64,
                    pub hash_blake2_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_128: ::core::primitive::u64,
                    pub hash_blake2_128_per_byte: ::core::primitive::u64,
                    pub ecdsa_recover: ::core::primitive::u64,
                    pub ecdsa_to_eth_address: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct InstructionWeights {
                    pub version: ::core::primitive::u32,
                    pub i64const: ::core::primitive::u32,
                    pub i64load: ::core::primitive::u32,
                    pub i64store: ::core::primitive::u32,
                    pub select: ::core::primitive::u32,
                    pub r#if: ::core::primitive::u32,
                    pub br: ::core::primitive::u32,
                    pub br_if: ::core::primitive::u32,
                    pub br_table: ::core::primitive::u32,
                    pub br_table_per_entry: ::core::primitive::u32,
                    pub call: ::core::primitive::u32,
                    pub call_indirect: ::core::primitive::u32,
                    pub call_indirect_per_param: ::core::primitive::u32,
                    pub local_get: ::core::primitive::u32,
                    pub local_set: ::core::primitive::u32,
                    pub local_tee: ::core::primitive::u32,
                    pub global_get: ::core::primitive::u32,
                    pub global_set: ::core::primitive::u32,
                    pub memory_current: ::core::primitive::u32,
                    pub memory_grow: ::core::primitive::u32,
                    pub i64clz: ::core::primitive::u32,
                    pub i64ctz: ::core::primitive::u32,
                    pub i64popcnt: ::core::primitive::u32,
                    pub i64eqz: ::core::primitive::u32,
                    pub i64extendsi32: ::core::primitive::u32,
                    pub i64extendui32: ::core::primitive::u32,
                    pub i32wrapi64: ::core::primitive::u32,
                    pub i64eq: ::core::primitive::u32,
                    pub i64ne: ::core::primitive::u32,
                    pub i64lts: ::core::primitive::u32,
                    pub i64ltu: ::core::primitive::u32,
                    pub i64gts: ::core::primitive::u32,
                    pub i64gtu: ::core::primitive::u32,
                    pub i64les: ::core::primitive::u32,
                    pub i64leu: ::core::primitive::u32,
                    pub i64ges: ::core::primitive::u32,
                    pub i64geu: ::core::primitive::u32,
                    pub i64add: ::core::primitive::u32,
                    pub i64sub: ::core::primitive::u32,
                    pub i64mul: ::core::primitive::u32,
                    pub i64divs: ::core::primitive::u32,
                    pub i64divu: ::core::primitive::u32,
                    pub i64rems: ::core::primitive::u32,
                    pub i64remu: ::core::primitive::u32,
                    pub i64and: ::core::primitive::u32,
                    pub i64or: ::core::primitive::u32,
                    pub i64xor: ::core::primitive::u32,
                    pub i64shl: ::core::primitive::u32,
                    pub i64shrs: ::core::primitive::u32,
                    pub i64shru: ::core::primitive::u32,
                    pub i64rotl: ::core::primitive::u32,
                    pub i64rotr: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub stack_height: ::core::option::Option<::core::primitive::u32>,
                    pub globals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub call_depth: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub code_len: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DeletedContract {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct RawContractInfo<_0, _1> {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                    pub code_hash: _0,
                    pub storage_deposit: _1,
                }
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct OwnerInfo {
                    pub owner: ::subxt::sp_core::crypto::AccountId32,
                    #[codec(compact)]
                    pub deposit: ::core::primitive::u128,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct PrefabWasmModule {
                    #[codec(compact)]
                    pub instruction_weights_version: ::core::primitive::u32,
                    #[codec(compact)]
                    pub initial: ::core::primitive::u32,
                    #[codec(compact)]
                    pub maximum: ::core::primitive::u32,
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
        }
        pub mod pallet_currencies {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        to: ::subxt::sp_core::crypto::AccountId32,
                        currency_id: runtime_types::primitives::currency::CurrencyId,
                        balance: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceTooLow,
                    #[codec(index = 1)]
                    InvalidContractOperation,
                }
            }
        }
        pub mod pallet_fee_enablement {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    onboard_asset {
                        asset_id: runtime_types::primitives::currency::CurrencyId,
                        enabled: ::core::primitive::bool,
                    },
                    #[codec(index = 1)]
                    enable_asset {
                        asset_id: runtime_types::primitives::currency::CurrencyId,
                    },
                    #[codec(index = 2)]
                    disable_asset {
                        asset_id: runtime_types::primitives::currency::CurrencyId,
                    },
                }
            }
        }
        pub mod pallet_fluent_fee {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    Placeholder,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {}
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
                    #[doc = "stalled. This will trigger a forced authority set change at the beginning"]
                    #[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
                    #[doc = "should be high enough to safely assume that the block signalling the"]
                    #[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
                    #[doc = "will start the new authority set using the given finalized block as base."]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Anonymously schedule a task."]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::laguna_runtime::Call,
                                ::subxt::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Cancel an anonymously scheduled task."]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Schedule a named task."]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::laguna_runtime::Call,
                                ::subxt::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a named scheduled task."]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Anonymously schedule a task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule`]."]
                    #[doc = "# </weight>"]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::laguna_runtime::Call,
                                ::subxt::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Schedule a named task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                    #[doc = "# </weight>"]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::laguna_runtime::Call,
                                ::subxt::sp_core::H256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Failed to schedule a call"]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    #[doc = "Cannot find the scheduled call."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Given target block number is in the past."]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    #[doc = "Reschedule failed because it does not change scheduled time."]
                    RescheduleNoChange,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Events type."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Scheduled some task."]
                    Scheduled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Canceled some task."]
                    Canceled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Dispatched some task."]
                    Dispatched {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "The call for the provided hash was not found so the task has been aborted."]
                    CallLookupFailed {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        error: runtime_types::frame_support::traits::schedule::LookupError,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ScheduledV3<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::laguna_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod primitives {
            use super::runtime_types;
            pub mod currency {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum CurrencyId {
                    #[codec(index = 0)]
                    NativeToken(runtime_types::primitives::currency::TokenId),
                    #[codec(index = 1)]
                    Erc20([::core::primitive::u8; 32usize]),
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum TokenId {
                    #[codec(index = 0)]
                    Laguna,
                    #[codec(index = 1)]
                    FeeToken,
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
            if let Self::Module(module_error) = self {
                Some(::subxt::ModuleErrorData {
                    pallet_index: module_error.index,
                    error: module_error.error,
                })
            } else {
                None
            }
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<T: ::subxt::Config, X> Clone for RuntimeApi<T, X> {
        fn clone(&self) -> Self {
            Self {
                client: self.client.clone(),
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X> RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn validate_metadata(&'a self) -> Result<(), ::subxt::MetadataError> {
            let runtime_metadata_hash = {
                let locked_metadata = self.client.metadata();
                let metadata = locked_metadata.read();
                metadata.metadata_hash(&PALLETS)
            };
            if runtime_metadata_hash
                != [
                    228u8, 211u8, 15u8, 249u8, 21u8, 121u8, 10u8, 241u8, 116u8, 98u8, 121u8, 75u8,
                    154u8, 8u8, 143u8, 135u8, 220u8, 78u8, 160u8, 28u8, 86u8, 129u8, 245u8, 20u8,
                    146u8, 188u8, 80u8, 114u8, 65u8, 217u8, 10u8, 223u8,
                ]
            {
                Err(::subxt::MetadataError::IncompatibleMetadata)
            } else {
                Ok(())
            }
        }
        pub fn constants(&'a self) -> ConstantsApi<'a, T> {
            ConstantsApi {
                client: &self.client,
            }
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
        pub fn events(&'a self) -> EventsApi<'a, T> {
            EventsApi {
                client: &self.client,
            }
        }
    }
    pub struct EventsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
        pub async fn at(
            &self,
            block_hash: T::Hash,
        ) -> Result<::subxt::events::Events<T, Event>, ::subxt::BasicError> {
            ::subxt::events::at::<T, Event>(self.client, block_hash).await
        }
        pub async fn subscribe(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe::<T, Event>(self.client).await
        }
        pub async fn subscribe_finalized(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<
                'a,
                ::subxt::events::FinalizedEventSub<'a, T::Header>,
                T,
                Event,
            >,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
        }
    }
    pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
            system::constants::ConstantsApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
            timestamp::constants::ConstantsApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::constants::ConstantsApi<'a, T> {
            scheduler::constants::ConstantsApi::new(self.client)
        }
        pub fn tokens(&self) -> tokens::constants::ConstantsApi<'a, T> {
            tokens::constants::ConstantsApi::new(self.client)
        }
        pub fn currencies(&self) -> currencies::constants::ConstantsApi<'a, T> {
            currencies::constants::ConstantsApi::new(self.client)
        }
        pub fn contract_assets_registry(
            &self,
        ) -> contract_assets_registry::constants::ConstantsApi<'a, T> {
            contract_assets_registry::constants::ConstantsApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi<'a, T> {
            transaction_payment::constants::ConstantsApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
            grandpa::constants::ConstantsApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::constants::ConstantsApi<'a, T> {
            contracts::constants::ConstantsApi::new(self.client)
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn tokens(&self) -> tokens::storage::StorageApi<'a, T> {
            tokens::storage::StorageApi::new(self.client)
        }
        pub fn contract_assets_registry(
            &self,
        ) -> contract_assets_registry::storage::StorageApi<'a, T> {
            contract_assets_registry::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn fee_enablement(&self) -> fee_enablement::storage::StorageApi<'a, T> {
            fee_enablement::storage::StorageApi::new(self.client)
        }
        pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
            aura::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::storage::StorageApi<'a, T> {
            contracts::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<'a, T, X> TransactionApi<'a, T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T, X> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn tokens(&self) -> tokens::calls::TransactionApi<'a, T, X> {
            tokens::calls::TransactionApi::new(self.client)
        }
        pub fn currencies(&self) -> currencies::calls::TransactionApi<'a, T, X> {
            currencies::calls::TransactionApi::new(self.client)
        }
        pub fn contract_assets_registry(
            &self,
        ) -> contract_assets_registry::calls::TransactionApi<'a, T, X> {
            contract_assets_registry::calls::TransactionApi::new(self.client)
        }
        pub fn fee_enablement(&self) -> fee_enablement::calls::TransactionApi<'a, T, X> {
            fee_enablement::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::calls::TransactionApi<'a, T, X> {
            contracts::calls::TransactionApi::new(self.client)
        }
    }
}
