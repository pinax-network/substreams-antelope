#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = None;
pub mod types {
    use substreams_antelope::types::*;
    pub type BlockSigningAuthority = BlockSigningAuthorityV0;
    pub type BlockchainParametersT = BlockchainParametersV1;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AbiHash {
        pub owner: Name,
        pub hash: Checksum256,
    }
    impl std::str::FromStr for AbiHash {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ActionReturnBuyram {
        pub payer: Name,
        pub receiver: Name,
        pub quantity: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes_purchased: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
        pub fee: Asset,
    }
    impl std::str::FromStr for ActionReturnBuyram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ActionReturnRamtransfer {
        pub from: Name,
        pub to: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub from_ram_bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub to_ram_bytes: Int64,
    }
    impl std::str::FromStr for ActionReturnRamtransfer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ActionReturnSellram {
        pub account: Name,
        pub quantity: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes_sold: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
        pub fee: Asset,
    }
    impl std::str::FromStr for ActionReturnSellram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Authority {
        pub threshold: Uint32,
        pub keys: Vec<KeyWeight>,
        pub accounts: Vec<PermissionLevelWeight>,
        pub waits: Vec<WaitWeight>,
    }
    impl std::str::FromStr for Authority {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BidRefund {
        pub bidder: Name,
        pub amount: Asset,
    }
    impl std::str::FromStr for BidRefund {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockHeader {
        pub timestamp: Uint32,
        pub producer: Name,
        pub confirmed: Uint16,
        pub previous: Checksum256,
        pub transaction_mroot: Checksum256,
        pub action_mroot: Checksum256,
        pub schedule_version: Uint32,
        pub new_producers: ProducerSchedule,
    }
    impl std::str::FromStr for BlockHeader {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockInfoRecord {
        pub version: Uint8,
        pub block_height: Uint32,
        pub block_timestamp: TimePoint,
    }
    impl std::str::FromStr for BlockInfoRecord {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockSigningAuthorityV0 {
        pub threshold: Uint32,
        pub keys: Vec<KeyWeight>,
    }
    impl std::str::FromStr for BlockSigningAuthorityV0 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockchainParameters {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub max_block_net_usage: Uint64,
        pub target_block_net_usage_pct: Uint32,
        pub max_transaction_net_usage: Uint32,
        pub base_per_transaction_net_usage: Uint32,
        pub net_usage_leeway: Uint32,
        pub context_free_discount_net_usage_num: Uint32,
        pub context_free_discount_net_usage_den: Uint32,
        pub max_block_cpu_usage: Uint32,
        pub target_block_cpu_usage_pct: Uint32,
        pub max_transaction_cpu_usage: Uint32,
        pub min_transaction_cpu_usage: Uint32,
        pub max_transaction_lifetime: Uint32,
        pub deferred_trx_expiration_window: Uint32,
        pub max_transaction_delay: Uint32,
        pub max_inline_action_size: Uint32,
        pub max_inline_action_depth: Uint16,
        pub max_authority_depth: Uint16,
    }
    impl std::str::FromStr for BlockchainParameters {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockchainParametersV1 {
        pub max_action_return_value_size: Uint32,
    }
    impl std::str::FromStr for BlockchainParametersV1 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Connector {
        pub balance: Asset,
        pub weight: Float64,
    }
    impl std::str::FromStr for Connector {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct DelegatedBandwidth {
        pub from: Name,
        pub to: Name,
        pub net_weight: Asset,
        pub cpu_weight: Asset,
    }
    impl std::str::FromStr for DelegatedBandwidth {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct EosioGlobalState {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub max_ram_size: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub total_ram_bytes_reserved: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub total_ram_stake: Int64,
        pub last_producer_schedule_update: BlockTimestampType,
        pub last_pervote_bucket_fill: TimePoint,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub pervote_bucket: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub perblock_bucket: Int64,
        pub total_unpaid_blocks: Uint32,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub total_activated_stake: Int64,
        pub thresh_activated_stake_time: TimePoint,
        pub last_producer_schedule_size: Uint16,
        pub total_producer_vote_weight: Float64,
        pub last_name_close: BlockTimestampType,
    }
    impl std::str::FromStr for EosioGlobalState {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct EosioGlobalState2 {
        pub new_ram_per_block: Uint16,
        pub last_ram_increase: BlockTimestampType,
        pub last_block_num: BlockTimestampType,
        pub total_producer_votepay_share: Float64,
        pub revision: Uint8,
    }
    impl std::str::FromStr for EosioGlobalState2 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct EosioGlobalState3 {
        pub last_vpay_state_update: TimePoint,
        pub total_vpay_share_change_rate: Float64,
    }
    impl std::str::FromStr for EosioGlobalState3 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct EosioGlobalState4 {
        pub continuous_rate: Float64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub inflation_pay_factor: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub votepay_factor: Int64,
    }
    impl std::str::FromStr for EosioGlobalState4 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ExchangeState {
        pub supply: Asset,
        pub base: Connector,
        pub quote: Connector,
    }
    impl std::str::FromStr for ExchangeState {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct KeyWeight {
        pub key: PublicKey,
        pub weight: Uint16,
    }
    impl std::str::FromStr for KeyWeight {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct NameBid {
        pub newname: Name,
        pub high_bidder: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub high_bid: Int64,
        pub last_bid_time: TimePoint,
    }
    impl std::str::FromStr for NameBid {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairTimePointSecInt64 {
        pub first: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub second: Int64,
    }
    impl std::str::FromStr for PairTimePointSecInt64 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PermissionLevel {
        pub actor: Name,
        pub permission: Name,
    }
    impl std::str::FromStr for PermissionLevel {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PermissionLevelWeight {
        pub permission: PermissionLevel,
        pub weight: Uint16,
    }
    impl std::str::FromStr for PermissionLevelWeight {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PowerupConfig {
        pub net: PowerupConfigResource,
        pub cpu: PowerupConfigResource,
        pub powerup_days: Uint32,
        pub min_powerup_fee: Asset,
    }
    impl std::str::FromStr for PowerupConfig {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PowerupConfigResource {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub current_weight_ratio: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub target_weight_ratio: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub assumed_stake_weight: Int64,
        pub target_timestamp: TimePointSec,
        pub exponent: Float64,
        pub decay_secs: Uint32,
        pub min_price: Asset,
        pub max_price: Asset,
    }
    impl std::str::FromStr for PowerupConfigResource {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PowerupOrder {
        pub version: Uint8,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub id: Uint64,
        pub owner: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub net_weight: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub cpu_weight: Int64,
        pub expires: TimePointSec,
    }
    impl std::str::FromStr for PowerupOrder {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PowerupState {
        pub version: Uint8,
        pub net: PowerupStateResource,
        pub cpu: PowerupStateResource,
        pub powerup_days: Uint32,
        pub min_powerup_fee: Asset,
    }
    impl std::str::FromStr for PowerupState {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PowerupStateResource {
        pub version: Uint8,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub weight: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub weight_ratio: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub assumed_stake_weight: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub initial_weight_ratio: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub target_weight_ratio: Int64,
        pub initial_timestamp: TimePointSec,
        pub target_timestamp: TimePointSec,
        pub exponent: Float64,
        pub decay_secs: Uint32,
        pub min_price: Asset,
        pub max_price: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub utilization: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub adjusted_utilization: Int64,
        pub utilization_timestamp: TimePointSec,
    }
    impl std::str::FromStr for PowerupStateResource {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ProducerInfo {
        pub owner: Name,
        pub total_votes: Float64,
        pub producer_key: PublicKey,
        pub is_active: Bool,
        pub url: String,
        pub unpaid_blocks: Uint32,
        pub last_claim_time: TimePoint,
        pub location: Uint16,
        pub producer_authority: BlockSigningAuthority,
    }
    impl std::str::FromStr for ProducerInfo {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ProducerInfo2 {
        pub owner: Name,
        pub votepay_share: Float64,
        pub last_votepay_share_update: TimePoint,
    }
    impl std::str::FromStr for ProducerInfo2 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ProducerKey {
        pub producer_name: Name,
        pub block_signing_key: PublicKey,
    }
    impl std::str::FromStr for ProducerKey {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ProducerSchedule {
        pub version: Uint32,
        pub producers: Vec<ProducerKey>,
    }
    impl std::str::FromStr for ProducerSchedule {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RefundRequest {
        pub owner: Name,
        pub request_time: TimePointSec,
        pub net_amount: Asset,
        pub cpu_amount: Asset,
    }
    impl std::str::FromStr for RefundRequest {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexBalance {
        pub version: Uint8,
        pub owner: Name,
        pub vote_stake: Asset,
        pub rex_balance: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub matured_rex: Int64,
        pub rex_maturities: Vec<PairTimePointSecInt64>,
    }
    impl std::str::FromStr for RexBalance {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexFund {
        pub version: Uint8,
        pub owner: Name,
        pub balance: Asset,
    }
    impl std::str::FromStr for RexFund {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexLoan {
        pub version: Uint8,
        pub from: Name,
        pub receiver: Name,
        pub payment: Asset,
        pub balance: Asset,
        pub total_staked: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
        pub expiration: TimePoint,
    }
    impl std::str::FromStr for RexLoan {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexMaturity {
        pub num_of_maturity_buckets: Uint32,
        pub sell_matured_rex: Bool,
        pub buy_rex_to_savings: Bool,
    }
    impl std::str::FromStr for RexMaturity {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexOrder {
        pub version: Uint8,
        pub owner: Name,
        pub rex_requested: Asset,
        pub proceeds: Asset,
        pub stake_change: Asset,
        pub order_time: TimePoint,
        pub is_open: Bool,
    }
    impl std::str::FromStr for RexOrder {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexPool {
        pub version: Uint8,
        pub total_lent: Asset,
        pub total_unlent: Asset,
        pub total_rent: Asset,
        pub total_lendable: Asset,
        pub total_rex: Asset,
        pub namebid_proceeds: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
    }
    impl std::str::FromStr for RexPool {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexReturnBuckets {
        pub version: Uint8,
        pub return_buckets: Vec<PairTimePointSecInt64>,
    }
    impl std::str::FromStr for RexReturnBuckets {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RexReturnPool {
        pub version: Uint8,
        pub last_dist_time: TimePointSec,
        pub pending_bucket_time: TimePointSec,
        pub oldest_bucket_time: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub pending_bucket_proceeds: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub current_rate_of_increase: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub proceeds: Int64,
    }
    impl std::str::FromStr for RexReturnPool {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct SchedulesInfo {
        pub start_time: TimePointSec,
        pub continuous_rate: Float64,
    }
    impl std::str::FromStr for SchedulesInfo {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UserResources {
        pub owner: Name,
        pub net_weight: Asset,
        pub cpu_weight: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
    }
    impl std::str::FromStr for UserResources {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct VoterInfo {
        pub owner: Name,
        pub proxy: Name,
        pub producers: Vec<Name>,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub staked: Int64,
        pub last_vote_weight: Float64,
        pub proxied_vote_weight: Float64,
        pub is_proxy: Bool,
        pub flags1: Uint32,
        pub reserved2: Uint32,
        pub reserved3: Asset,
    }
    impl std::str::FromStr for VoterInfo {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct WaitWeight {
        pub wait_sec: Uint32,
        pub weight: Uint16,
    }
    impl std::str::FromStr for WaitWeight {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct LimitAuthChange {
        pub version: Uint8,
        pub account: Name,
        pub allow_perms: Vec<Name>,
        pub disallow_perms: Vec<Name>,
    }
    impl std::str::FromStr for LimitAuthChange {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Activate {
        pub feature_digest: Checksum256,
    }
    impl std::str::FromStr for Activate {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Activate {
        const NAME: &'static str = "activate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Bidname {
        pub bidder: Name,
        pub newname: Name,
        pub bid: Asset,
    }
    impl std::str::FromStr for Bidname {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Bidname {
        const NAME: &'static str = "bidname";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Bidrefund {
        pub bidder: Name,
        pub newname: Name,
    }
    impl std::str::FromStr for Bidrefund {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Bidrefund {
        const NAME: &'static str = "bidrefund";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Buyram {
        pub payer: Name,
        pub receiver: Name,
        pub quant: Asset,
    }
    impl std::str::FromStr for Buyram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Buyram {
        const NAME: &'static str = "buyram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Buyramburn {
        pub payer: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Buyramburn {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Buyramburn {
        const NAME: &'static str = "buyramburn";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Buyrambytes {
        pub payer: Name,
        pub receiver: Name,
        pub bytes: Uint32,
    }
    impl std::str::FromStr for Buyrambytes {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Buyrambytes {
        const NAME: &'static str = "buyrambytes";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Buyramself {
        pub account: Name,
        pub quant: Asset,
    }
    impl std::str::FromStr for Buyramself {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Buyramself {
        const NAME: &'static str = "buyramself";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Buyrex {
        pub from: Name,
        pub amount: Asset,
    }
    impl std::str::FromStr for Buyrex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Buyrex {
        const NAME: &'static str = "buyrex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Canceldelay {
        pub canceling_auth: PermissionLevel,
        pub trx_id: Checksum256,
    }
    impl std::str::FromStr for Canceldelay {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Canceldelay {
        const NAME: &'static str = "canceldelay";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Cfgpowerup {
        pub args: PowerupConfig,
    }
    impl std::str::FromStr for Cfgpowerup {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Cfgpowerup {
        const NAME: &'static str = "cfgpowerup";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claimrewards {
        pub owner: Name,
    }
    impl std::str::FromStr for Claimrewards {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Claimrewards {
        const NAME: &'static str = "claimrewards";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Closerex {
        pub owner: Name,
    }
    impl std::str::FromStr for Closerex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Closerex {
        const NAME: &'static str = "closerex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Cnclrexorder {
        pub owner: Name,
    }
    impl std::str::FromStr for Cnclrexorder {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Cnclrexorder {
        const NAME: &'static str = "cnclrexorder";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Consolidate {
        pub owner: Name,
    }
    impl std::str::FromStr for Consolidate {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Consolidate {
        const NAME: &'static str = "consolidate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Defcpuloan {
        pub from: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
        pub amount: Asset,
    }
    impl std::str::FromStr for Defcpuloan {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Defcpuloan {
        const NAME: &'static str = "defcpuloan";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Defnetloan {
        pub from: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
        pub amount: Asset,
    }
    impl std::str::FromStr for Defnetloan {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Defnetloan {
        const NAME: &'static str = "defnetloan";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Delegatebw {
        pub from: Name,
        pub receiver: Name,
        pub stake_net_quantity: Asset,
        pub stake_cpu_quantity: Asset,
        pub transfer: Bool,
    }
    impl std::str::FromStr for Delegatebw {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Delegatebw {
        const NAME: &'static str = "delegatebw";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deleteauth {
        pub account: Name,
        pub permission: Name,
        pub authorized_by: Name,
    }
    impl std::str::FromStr for Deleteauth {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Deleteauth {
        const NAME: &'static str = "deleteauth";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Delschedule {
        pub start_time: TimePointSec,
    }
    impl std::str::FromStr for Delschedule {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Delschedule {
        const NAME: &'static str = "delschedule";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deposit {
        pub owner: Name,
        pub amount: Asset,
    }
    impl std::str::FromStr for Deposit {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Deposit {
        const NAME: &'static str = "deposit";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Donatetorex {
        pub payer: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Donatetorex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Donatetorex {
        const NAME: &'static str = "donatetorex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Execschedule {}
    impl std::str::FromStr for Execschedule {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Execschedule {
        const NAME: &'static str = "execschedule";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Fundcpuloan {
        pub from: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
        pub payment: Asset,
    }
    impl std::str::FromStr for Fundcpuloan {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Fundcpuloan {
        const NAME: &'static str = "fundcpuloan";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Fundnetloan {
        pub from: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub loan_num: Uint64,
        pub payment: Asset,
    }
    impl std::str::FromStr for Fundnetloan {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Fundnetloan {
        const NAME: &'static str = "fundnetloan";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Init {
        pub version: VarUint32,
        pub core: Symbol,
    }
    impl std::str::FromStr for Init {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Init {
        const NAME: &'static str = "init";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Limitauthchg {
        pub account: Name,
        pub allow_perms: Vec<Name>,
        pub disallow_perms: Vec<Name>,
    }
    impl std::str::FromStr for Limitauthchg {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Limitauthchg {
        const NAME: &'static str = "limitauthchg";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Linkauth {
        pub account: Name,
        pub code: Name,
        #[serde(rename = "type")]
        pub type_: Name,
        pub requirement: Name,
        pub authorized_by: Name,
    }
    impl std::str::FromStr for Linkauth {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Linkauth {
        const NAME: &'static str = "linkauth";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Logbuyram {
        pub payer: Name,
        pub receiver: Name,
        pub quantity: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
        pub fee: Asset,
    }
    impl std::str::FromStr for Logbuyram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Logbuyram {
        const NAME: &'static str = "logbuyram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Logramchange {
        pub owner: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
    }
    impl std::str::FromStr for Logramchange {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Logramchange {
        const NAME: &'static str = "logramchange";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Logsellram {
        pub account: Name,
        pub quantity: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
        pub fee: Asset,
    }
    impl std::str::FromStr for Logsellram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Logsellram {
        const NAME: &'static str = "logsellram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Logsystemfee {
        pub protocol: Name,
        pub fee: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Logsystemfee {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Logsystemfee {
        const NAME: &'static str = "logsystemfee";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Mvfrsavings {
        pub owner: Name,
        pub rex: Asset,
    }
    impl std::str::FromStr for Mvfrsavings {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Mvfrsavings {
        const NAME: &'static str = "mvfrsavings";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Mvtosavings {
        pub owner: Name,
        pub rex: Asset,
    }
    impl std::str::FromStr for Mvtosavings {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Mvtosavings {
        const NAME: &'static str = "mvtosavings";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Newaccount {
        pub creator: Name,
        pub name: Name,
        pub owner: Authority,
        pub active: Authority,
    }
    impl std::str::FromStr for Newaccount {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Newaccount {
        const NAME: &'static str = "newaccount";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Onblock {
        pub header: BlockHeader,
    }
    impl std::str::FromStr for Onblock {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Onblock {
        const NAME: &'static str = "onblock";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Onerror {
        pub sender_id: Uint128,
        pub sent_trx: Bytes,
    }
    impl std::str::FromStr for Onerror {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Onerror {
        const NAME: &'static str = "onerror";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Powerup {
        pub payer: Name,
        pub receiver: Name,
        pub days: Uint32,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub net_frac: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub cpu_frac: Int64,
        pub max_payment: Asset,
    }
    impl std::str::FromStr for Powerup {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Powerup {
        const NAME: &'static str = "powerup";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Powerupexec {
        pub user: Name,
        pub max: Uint16,
    }
    impl std::str::FromStr for Powerupexec {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Powerupexec {
        const NAME: &'static str = "powerupexec";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ramburn {
        pub owner: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        pub memo: String,
    }
    impl std::str::FromStr for Ramburn {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Ramburn {
        const NAME: &'static str = "ramburn";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ramtransfer {
        pub from: Name,
        pub to: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
        pub memo: String,
    }
    impl std::str::FromStr for Ramtransfer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Ramtransfer {
        const NAME: &'static str = "ramtransfer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Refund {
        pub owner: Name,
    }
    impl std::str::FromStr for Refund {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Refund {
        const NAME: &'static str = "refund";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Regproducer {
        pub producer: Name,
        pub producer_key: PublicKey,
        pub url: String,
        pub location: Uint16,
    }
    impl std::str::FromStr for Regproducer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Regproducer {
        const NAME: &'static str = "regproducer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Regproducer2 {
        pub producer: Name,
        pub producer_authority: BlockSigningAuthority,
        pub url: String,
        pub location: Uint16,
    }
    impl std::str::FromStr for Regproducer2 {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Regproducer2 {
        const NAME: &'static str = "regproducer2";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Regproxy {
        pub proxy: Name,
        pub isproxy: Bool,
    }
    impl std::str::FromStr for Regproxy {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Regproxy {
        const NAME: &'static str = "regproxy";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rentcpu {
        pub from: Name,
        pub receiver: Name,
        pub loan_payment: Asset,
        pub loan_fund: Asset,
    }
    impl std::str::FromStr for Rentcpu {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Rentcpu {
        const NAME: &'static str = "rentcpu";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rentnet {
        pub from: Name,
        pub receiver: Name,
        pub loan_payment: Asset,
        pub loan_fund: Asset,
    }
    impl std::str::FromStr for Rentnet {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Rentnet {
        const NAME: &'static str = "rentnet";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rexexec {
        pub user: Name,
        pub max: Uint16,
    }
    impl std::str::FromStr for Rexexec {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Rexexec {
        const NAME: &'static str = "rexexec";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rmvproducer {
        pub producer: Name,
    }
    impl std::str::FromStr for Rmvproducer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Rmvproducer {
        const NAME: &'static str = "rmvproducer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Sellram {
        pub account: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub bytes: Int64,
    }
    impl std::str::FromStr for Sellram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Sellram {
        const NAME: &'static str = "sellram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Sellrex {
        pub from: Name,
        pub rex: Asset,
    }
    impl std::str::FromStr for Sellrex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Sellrex {
        const NAME: &'static str = "sellrex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setabi {
        pub account: Name,
        pub abi: Bytes,
        pub memo: String,
    }
    impl std::str::FromStr for Setabi {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setabi {
        const NAME: &'static str = "setabi";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setacctcpu {
        pub account: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub cpu_weight: Int64,
    }
    impl std::str::FromStr for Setacctcpu {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setacctcpu {
        const NAME: &'static str = "setacctcpu";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setacctnet {
        pub account: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub net_weight: Int64,
    }
    impl std::str::FromStr for Setacctnet {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setacctnet {
        const NAME: &'static str = "setacctnet";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setacctram {
        pub account: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
    }
    impl std::str::FromStr for Setacctram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setacctram {
        const NAME: &'static str = "setacctram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setalimits {
        pub account: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub ram_bytes: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub net_weight: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub cpu_weight: Int64,
    }
    impl std::str::FromStr for Setalimits {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setalimits {
        const NAME: &'static str = "setalimits";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setcode {
        pub account: Name,
        pub vmtype: Uint8,
        pub vmversion: Uint8,
        pub code: Bytes,
        pub memo: String,
    }
    impl std::str::FromStr for Setcode {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setcode {
        const NAME: &'static str = "setcode";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setinflation {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub annual_rate: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub inflation_pay_factor: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub votepay_factor: Int64,
    }
    impl std::str::FromStr for Setinflation {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setinflation {
        const NAME: &'static str = "setinflation";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setparams {
        pub params: BlockchainParametersT,
    }
    impl std::str::FromStr for Setparams {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setparams {
        const NAME: &'static str = "setparams";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setpayfactor {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub inflation_pay_factor: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub votepay_factor: Int64,
    }
    impl std::str::FromStr for Setpayfactor {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setpayfactor {
        const NAME: &'static str = "setpayfactor";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setpriv {
        pub account: Name,
        pub is_priv: Uint8,
    }
    impl std::str::FromStr for Setpriv {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setpriv {
        const NAME: &'static str = "setpriv";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setram {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub max_ram_size: Uint64,
    }
    impl std::str::FromStr for Setram {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setram {
        const NAME: &'static str = "setram";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setramrate {
        pub bytes_per_block: Uint16,
    }
    impl std::str::FromStr for Setramrate {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setramrate {
        const NAME: &'static str = "setramrate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setrex {
        pub balance: Asset,
    }
    impl std::str::FromStr for Setrex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setrex {
        const NAME: &'static str = "setrex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setrexmature {
        pub num_of_maturity_buckets: Uint32,
        pub sell_matured_rex: Bool,
        pub buy_rex_to_savings: Bool,
    }
    impl std::str::FromStr for Setrexmature {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setrexmature {
        const NAME: &'static str = "setrexmature";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setschedule {
        pub start_time: TimePointSec,
        pub continuous_rate: Float64,
    }
    impl std::str::FromStr for Setschedule {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setschedule {
        const NAME: &'static str = "setschedule";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Undelegatebw {
        pub from: Name,
        pub receiver: Name,
        pub unstake_net_quantity: Asset,
        pub unstake_cpu_quantity: Asset,
    }
    impl std::str::FromStr for Undelegatebw {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Undelegatebw {
        const NAME: &'static str = "undelegatebw";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unlinkauth {
        pub account: Name,
        pub code: Name,
        #[serde(rename = "type")]
        pub type_: Name,
        pub authorized_by: Name,
    }
    impl std::str::FromStr for Unlinkauth {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Unlinkauth {
        const NAME: &'static str = "unlinkauth";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unregprod {
        pub producer: Name,
    }
    impl std::str::FromStr for Unregprod {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Unregprod {
        const NAME: &'static str = "unregprod";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unstaketorex {
        pub owner: Name,
        pub receiver: Name,
        pub from_net: Asset,
        pub from_cpu: Asset,
    }
    impl std::str::FromStr for Unstaketorex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Unstaketorex {
        const NAME: &'static str = "unstaketorex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unvest {
        pub account: Name,
        pub unvest_net_quantity: Asset,
        pub unvest_cpu_quantity: Asset,
    }
    impl std::str::FromStr for Unvest {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Unvest {
        const NAME: &'static str = "unvest";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Updateauth {
        pub account: Name,
        pub permission: Name,
        pub parent: Name,
        pub auth: Authority,
        pub authorized_by: Name,
    }
    impl std::str::FromStr for Updateauth {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Updateauth {
        const NAME: &'static str = "updateauth";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Updaterex {
        pub owner: Name,
    }
    impl std::str::FromStr for Updaterex {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Updaterex {
        const NAME: &'static str = "updaterex";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Updtrevision {
        pub revision: Uint8,
    }
    impl std::str::FromStr for Updtrevision {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Updtrevision {
        const NAME: &'static str = "updtrevision";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Voteproducer {
        pub voter: Name,
        pub proxy: Name,
        pub producers: Vec<Name>,
    }
    impl std::str::FromStr for Voteproducer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Voteproducer {
        const NAME: &'static str = "voteproducer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Voteupdate {
        pub voter_name: Name,
    }
    impl std::str::FromStr for Voteupdate {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Voteupdate {
        const NAME: &'static str = "voteupdate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Wasmcfg {
        pub settings: Name,
    }
    impl std::str::FromStr for Wasmcfg {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Wasmcfg {
        const NAME: &'static str = "wasmcfg";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Withdraw {
        pub owner: Name,
        pub amount: Asset,
    }
    impl std::str::FromStr for Withdraw {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Withdraw {
        const NAME: &'static str = "withdraw";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
}