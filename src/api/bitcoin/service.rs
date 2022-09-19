// module for creating tx and other
use super::{handler::Utxo, msg_tx::MsgTx, types::U256};
use crate::api::bitcoin::msg_tx::{OutPoint, TxIn};

type Hash = Vec<u8>;

const HASH_SIZE: i32 = 32;
const MAX_HASH_STRING_SIZE: i32 = HASH_SIZE * 2;
const MAX_TX_IN_SEQUENCE_NUM: u32 = 0xffffffff;

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
    let mut source_utxos_amount: U256 = "0".into();
    // let mut source_utxos_amount: usize = 0;

    // for (idx, utxo) in utxos.iter().enumerate() {}
    for utxo in utxos {
        let hash_str = utxo.tx_id.as_ref().unwrap();
        // source_utxos_amount += utxo.amount.unwrap();

        source_utxos_amount = match source_utxos_amount.checked_add(utxo.amount.unwrap().into()) {
            Some(r) => r,
            None => panic!("overflow utxos amount"),
        };

        let source_utxo_hash = hash_from_str(hash_str);

        let source_utxo_index = utxo.vout.unwrap() as u32;

        let source_utxo = OutPoint {
            hash: source_utxo_hash,
            index: source_utxo_index,
        };

        // let sourceTxIn = wire.NewTxIn(sourceUTXO, nil, nil);
        let source_tx_in = TxIn {
            previous_out_point: source_utxo,
            signature_script: None,
            witness: None,
            sequence: MAX_TX_IN_SEQUENCE_NUM,
        };

        tx.add_tx_in(source_tx_in);

        // println!("{:?}", tx);
        println!("{:?}", source_utxos_amount);
    }
}

fn hash_from_str(hash: &str) -> Hash {
    let mut dst: Hash = vec![Default::default(); 32];
    decode(&mut dst, hash);

    dst
}

fn decode(dst: &mut Hash, src: &str) {
    // Return error if hash string is too long.
    if src.len() > MAX_HASH_STRING_SIZE as usize {
        panic!("max hash string length is {} bytes", MAX_HASH_STRING_SIZE);
        // return ErrHashStrSize
    }

    let mut src_bytes: Vec<u8> = Vec::new();

    if src.len() % 2 == 0 {
        src_bytes = src.as_bytes().to_vec();
    } else {
        src_bytes = vec![Default::default(); 1 + src.len()];
        src_bytes[0] = b'0';

        src_bytes[1..].copy_from_slice(src.as_bytes());
    }

    // Hex decode the source bytes to a temporary destination.
    let mut reversed_hash: Hash = vec![Default::default(); 32];

    let decoded_len = src_bytes.len() / 2;
    match hex::decode_to_slice(src_bytes, &mut reversed_hash[32 - decoded_len..]) {
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

        decode(&mut dst, src);

        assert_eq!(dst, check);
    }
}
