use super::{block_header::BlockHeader, msg_tx::MsgTx};

// MsgBlock implements the Message interface and represents a bitcoin
// block message.  It is used to deliver block and transaction information in
// response to a getdata message (MsgGetData) for a given block hash.
pub struct MsgBlock {
    header: BlockHeader,
    transactions: Vec<MsgTx>,
}
