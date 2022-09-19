use core::time;

type Hash = Vec<u8>;

// BlockHeader defines information about a block and is used in the bitcoin
// block (MsgBlock) and headers (MsgHeaders) messages.
pub struct BlockHeader {
    // Version of the block.  This is not the same as the protocol version.
    version: i32,

    // Hash of the previous block header in the block chain.
    prev_block: Hash,

    // Merkle tree reference to hash of all transactions for the block.
    merkle_root: Hash,

    // Time the block was created.  This is, unfortunately, encoded as a
    // uint32 on the wire and therefore is limited to 2106.
    timestamp: time::Duration,

    // Difficulty target for the block.
    bits: u32,

    // Nonce used to generate the block.
    nonce: u32,
}
