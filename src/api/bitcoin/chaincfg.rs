// Params defines a Bitcoin network by its parameters.  These parameters may be
// used by Bitcoin applications to differentiate networks as well as addresses
// and keys for one network from those intended for use on another network.

use core::time;

use super::{msg_block::MsgBlock, types::U256};

type BitcoinNet = u32;
type Hash = Vec<u8>;
type Byte = u8;
const DEFINED_DEPLOYMENTS: usize = 4;

// Checkpoint identifies a known good point in the block chain.  Using
// checkpoints allows a few optimizations for old blocks during initial download
// and also prevents forks from old blocks.
//
// Each checkpoint is selected based upon several factors.  See the
// documentation for blockchain.IsCheckpointCandidate for details on the
// selection criteria.
struct Checkpoint {
    height: i32,
    hash: Hash,
}

// DNSSeed identifies a DNS seed.
struct DNSSeed {
    // Host defines the hostname of the seed.
    host: String,

    // HasFiltering defines whether the seed supports filtering
    // by service flags (wire.ServiceFlag).
    has_filtering: bool,
}

// ConsensusDeployment defines details related to a specific consensus rule
// change that is voted in.  This is part of BIP0009.
struct ConsensusDeployment {
    // BitNumber defines the specific bit number within the block version
    // this particular soft-fork deployment refers to.
    bit_number: u8,

    // StartTime is the median block time after which voting on the
    // deployment starts.
    start_time: u64,

    // ExpireTime is the median block time after which the attempted
    // deployment expires.
    expire_time: u64,
}

pub struct Params {
    // Name defines a human-readable identifier for the network.
    name: String,

    // Net defines the magic bytes used to identify the network.
    net: BitcoinNet,

    // DefaultPort defines the default peer-to-peer port for the network.
    default_port: String,

    // DNSSeeds defines a list of DNS seeds for the network that are used
    // as one method to discover peers.
    dns_seeds: Vec<DNSSeed>,

    // GenesisBlock defines the first block of the chain.
    genesis_block: MsgBlock,

    // GenesisHash is the starting block hash.
    genesis_hash: Hash,

    // PowLimit defines the highest allowed proof of work value for a block
    // as a uint256.
    // pow_limit: U256,
    pow_limit: U256,

    // PowLimitBits defines the highest allowed proof of work value for a
    // block in compact form.
    pow_limit_bits: u32,

    // These fields define the block heights at which the specified softfork
    // BIP became active.
    bip_0034_height: i32,
    bip_0065_height: i32,
    bip_0066_height: i32,

    // CoinbaseMaturity is the number of blocks required before newly mined
    // coins (coinbase transactions) can be spent.
    coinbase_maturity: u16,

    // SubsidyReductionInterval is the interval of blocks before the subsidy
    // is reduced.
    subsidy_reduction_interval: i32,

    // TargetTimespan is the desired amount of time that should elapse
    // before the block difficulty requirement is examined to determine how
    // it should be changed in order to maintain the desired block
    // generation rate.
    target_timespan: time::Duration,

    // TargetTimePerBlock is the desired amount of time to generate each
    // block.
    target_time_per_block: time::Duration,

    // RetargetAdjustmentFactor is the adjustment factor used to limit
    // the minimum and maximum amount of adjustment that can occur between
    // difficulty retargets.
    retarget_adjustment_factor: i64,

    // ReduceMinDifficulty defines whether the network should reduce the
    // minimum required difficulty after a long enough period of time has
    // passed without finding a block.  This is really only useful for test
    // networks and should not be set on a main network.
    reduce_min_difficulty: bool,

    // MinDiffReductionTime is the amount of time after which the minimum
    // required difficulty should be reduced when a block hasn't been found.
    //
    // NOTE: This only applies if ReduceMinDifficulty is true.
    min_diff_reduction_time: time::Duration,

    // GenerateSupported specifies whether or not CPU mining is allowed.
    generate_supported: bool,

    // Checkpoints ordered from oldest to newest.
    checkpoints: Vec<Checkpoint>,

    // These fields are related to voting on consensus rule changes as
    // defined by BIP0009.
    //
    // RuleChangeActivationThreshold is the number of blocks in a threshold
    // state retarget window for which a positive vote for a rule change
    // must be cast in order to lock in a rule change. It should typically
    // be 95% for the main network and 75% for test networks.
    //
    // MinerConfirmationWindow is the number of blocks in each threshold
    // state retarget window.
    //
    // Deployments define the specific consensus rule changes to be voted
    // on.
    rule_change_activation_threshold: u32,
    miner_confirmation_window: u32,
    deployments: [ConsensusDeployment; DEFINED_DEPLOYMENTS],

    // Mempool parameters
    relay_non_std_txs: bool,

    // Human-readable part for Bech32 encoded segwit addresses, as defined
    // in BIP 173.
    bech_32_hrp_segwit: String,

    // Address encoding magics
    pub_key_hash_addr_id: Byte,         // First byte of a P2PKH address
    script_hash_addr_id: Byte,          // First byte of a P2SH address
    private_key_id: Byte,               // First byte of a WIF private key
    witness_pub_key_hash_addr_id: Byte, // First byte of a P2WPKH address
    witness_script_hash_addr_id: Byte,  // First byte of a P2WSH address

    // BIP32 hierarchical deterministic extended key magics
    hd_private_key_id: [Byte; 4],
    hd_public_key_id: [Byte; 4],

    // BIP44 coin type used in the hierarchical deterministic path for
    // address generation.
    hd_coin_type: u32,
}
