use std::str::FromStr;

use bitcoin::{
    consensus::encode, hashes::hex::FromHex, locktime::PackedLockTime, psbt::Psbt, Address, Amount,
    Network, OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

// module for creating tx and other
use super::handler::{ToAddresses, Utxo};
const NETWORK: Network = Network::Testnet;

pub fn create_transaction(utxos: &Vec<Utxo>, to: &Vec<ToAddresses>, change: &str) {
    // let change = Address::from_str(change);
    // let script = utxos[0].pk_script.as_ref().unwrap();

    let txs_in: Vec<TxIn> = utxos
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hex(utxo.tx_id.as_ref().unwrap()).unwrap(),
                vout: utxo.vout.unwrap(),
            },
            script_sig: Script::new(),
            sequence: Sequence::MAX, // Disable LockTime and RBF.,
            witness: Witness::default(),
        })
        .collect();

    let txs_out: Vec<TxOut> = to
        .iter()
        .map(|t| TxOut {
            value: Amount::from_str(t.amount.as_ref().unwrap())
                .unwrap()
                .to_sat(),
            script_pubkey: Address::from_str(t.to_address.as_ref().unwrap())
                .unwrap()
                .script_pubkey(),
        })
        .collect();

    let tx = Transaction {
        version: 2,
        lock_time: PackedLockTime::ZERO,
        input: txs_in,
        output: txs_out,
    };

    let psbt = Psbt::from_unsigned_tx(tx).unwrap();

    let hex = encode::serialize_hex(&psbt.extract_tx());
    println!(
        "You should now be able to broadcast the following transaction: \n\n{}",
        hex
    );
}
