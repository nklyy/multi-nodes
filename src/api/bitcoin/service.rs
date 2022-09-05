// module for creating tx and other

use std::{array, io::Bytes};

use super::{handler::Utxo, utils::last_index_byte};

const DEFAULT_TX_IN_OUT_ALLOC: usize = 15;
const HASH_SIZE: i32 = 32;
const MAX_HASH_STRING_SIZE: i32 = HASH_SIZE * 2;
const MAX_TX_IN_SEQUENCE_NUM: u32 = 0xffffffff;

type TxWitness = Vec<Vec<u8>>;
type Hash = Vec<u8>;

#[derive(Debug)]
struct OutPoint {
    Hash: Hash,
    Index: u32,
}

impl OutPoint {
    fn new() -> OutPoint {
        OutPoint {
            Hash: vec![Default::default(); 32],
            Index: 0,
        }
    }
}

#[derive(Debug)]
struct TxIn {
    PreviousOutPoint: OutPoint,
    SignatureScript: Option<Vec<u8>>,
    Witness: Option<TxWitness>,
    Sequence: u32,
}

impl TxIn {
    fn new() -> TxIn {
        TxIn {
            PreviousOutPoint: OutPoint::new(),
            SignatureScript: Some(Vec::new()),
            Witness: Some(Vec::new()),
            Sequence: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct TxOut {
    Value: i64,
    PkScript: Vec<u8>,
}

impl TxOut {
    fn new() -> TxOut {
        TxOut {
            Value: 0,
            PkScript: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct MsgTx {
    Version: i32,
    TxIn: Vec<TxIn>,
    TxOut: Vec<TxOut>,
}

impl MsgTx {
    fn new(version: i32) -> MsgTx {
        MsgTx {
            Version: version,
            TxIn: std::iter::repeat_with(|| TxIn::new())
                .take(DEFAULT_TX_IN_OUT_ALLOC)
                .collect::<Vec<_>>(),
            TxOut: std::iter::repeat_with(|| TxOut::new())
                .take(DEFAULT_TX_IN_OUT_ALLOC)
                .collect::<Vec<_>>(),
        }
    }

    fn add_tx_in(&mut self, ti: TxIn) {
        self.TxIn.push(ti);
    }
}

pub fn create_transaction(utxos: &Vec<Utxo>) {
    // Calculate all unspent amount
    // let mut utxosAmount: usize = 0;
    // utxos
    //     .iter()
    //     .for_each(|utxo| utxosAmount += utxo.amount.unwrap());

    // println!("AMOUNT {}", utxosAmount);

    // Init transaction
    let mut tx = MsgTx::new(2);

    // prepare transaction inputs
    let mut source_utxos_amount: usize = 0;

    // for (idx, utxo) in utxos.iter().enumerate() {}
    for utxo in utxos {
        let hash_str = utxo.tx_id.as_ref().unwrap();

        source_utxos_amount += utxo.amount.unwrap();

        let source_utxo_hash = hash_from_str(hash_str.to_string());

        let source_utxo_index = utxo.vout.unwrap() as u32;

        let source_utxo = OutPoint {
            Hash: source_utxo_hash,
            Index: source_utxo_index,
        };

        // let sourceTxIn = wire.NewTxIn(sourceUTXO, nil, nil);
        let source_tx_in = TxIn {
            PreviousOutPoint: source_utxo,
            SignatureScript: None,
            Witness: None,
            Sequence: MAX_TX_IN_SEQUENCE_NUM,
        };

        tx.add_tx_in(source_tx_in);

        // println!("{:?}", tx);
        println!("{:?}", source_utxos_amount);
    }
}

fn hash_from_str(hash: String) -> Hash {
    let mut dst: Hash = vec![Default::default(); 32];
    decode(&mut dst, hash);

    dst
}

fn decode(dst: &mut Hash, src: String) {
    // Return error if hash string is too long.
    if src.len() > MAX_HASH_STRING_SIZE as usize {
        panic!("max hash string length is {} bytes", MAX_HASH_STRING_SIZE);
        // return ErrHashStrSize
    }

    let mut srcBytes: Vec<u8> = Vec::new();
    if src.len() % 2 == 0 {
        srcBytes = src.as_bytes().to_vec();
    } else {
        srcBytes = vec![Default::default(); 1 + src.len()];
        srcBytes[0] = b'0';

        srcBytes[1..].copy_from_slice(src.as_bytes());
    }

    // Hex decode the source bytes to a temporary destination.
    let mut reversed_hash: Hash = vec![Default::default(); 32];

    let decoded_len = srcBytes.len() / 2;
    match hex::decode_to_slice(srcBytes, &mut reversed_hash[32 - decoded_len..]) {
        Ok(_) => println!("success decoded"),
        Err(err) => panic!("{}", err),
    };

    // Reverse copy from the temporary hash to destination.  Because the
    // temporary was zeroed, the written result will be correctly padded.
    let to: usize = (HASH_SIZE / 2).try_into().unwrap();
    for (i, b) in reversed_hash[..to as usize].into_iter().enumerate() {
        (dst[i], dst[HASH_SIZE as usize - 1 as usize - i]) =
            (reversed_hash[HASH_SIZE as usize - 1 as usize - i], *b)
    }
}

// fn decode_address(address: String) {
//     let oneIndex = last_index_byte(address, b'1');
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn decode_test() {
        let mut dst: Hash = vec![Default::default(); 32];
        let src = "989d301c546841d0ac5c8354c7d78079e3603b089682d1639b2ee1c1a8010c6a";

        let check = [
            106, 12, 1, 168, 193, 225, 46, 155, 99, 209, 130, 150, 8, 59, 96, 227, 121, 128, 215,
            199, 84, 131, 92, 172, 208, 65, 104, 84, 28, 48, 157, 152,
        ];

        decode(&mut dst, src.to_string());

        assert_eq!(dst, check);
    }
}
