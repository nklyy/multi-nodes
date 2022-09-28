use std::str::FromStr;

use bitcoin::{
    consensus::encode, hashes::hex::FromHex, locktime::PackedLockTime, psbt::Psbt, Address, Amount,
    OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

use super::handler::{ToAddresses, Utxo};

pub fn create_transaction(
    utxos: &Vec<Utxo>,
    to: &Vec<ToAddresses>,
    change: &str,
    fee_rate: f64,
) -> Result<String, &'static str> {
    let tx_in_amount: f64 = utxos.iter().map(|utxo| utxo.amount.unwrap()).sum();
    let tx_out_amount: f64 = to.iter().map(|t| t.amount.unwrap()).sum();

    let mut txs_in: Vec<TxIn> = Vec::new();
    for utxo in utxos {
        let tx_id = match Txid::from_hex(utxo.tx_id.as_ref().unwrap()) {
            Ok(tx_id) => tx_id,
            Err(_err) => return Err("failed to decode tx_id"),
        };

        txs_in.push(TxIn {
            previous_output: OutPoint {
                txid: tx_id,
                vout: utxo.vout.unwrap(),
            },
            script_sig: Script::new(),
            sequence: Sequence::MAX, // Disable LockTime and RBF.,
            witness: Witness::default(),
        })
    }

    let mut txs_out: Vec<TxOut> = Vec::new();
    for t in to {
        let script_pubkey = match Address::from_str(t.to_address.as_ref().unwrap()) {
            Ok(key) => key.script_pubkey(),
            Err(_err) => return Err("failed to decode script_pubkey"),
        };

        let value = match Amount::from_btc(t.amount.unwrap()) {
            Ok(value) => value.to_sat(),
            Err(_err) => return Err("failed decode to address amount to sat"),
        };

        txs_out.push(TxOut {
            value,
            script_pubkey,
        })
    }

    if tx_out_amount > tx_in_amount {
        return Err("your balance too low for this transaction");
    }

    let change_amount = format!("{:.8}", (tx_in_amount - tx_out_amount));

    let change_amount = match Amount::from_btc(change_amount.parse().unwrap()) {
        Ok(amount) => amount.to_sat(),
        Err(_err) => return Err("failed parse change amount"),
    };
    let change_address_script = match Address::from_str(change) {
        Ok(address) => address.script_pubkey(),
        Err(_err) => return Err("failed to decode change address to script"),
    };

    // push change address and amount
    txs_out.push(TxOut {
        value: change_amount,
        script_pubkey: change_address_script,
    });

    let tx_byte_size = txs_in.len() * 180 + txs_out.len() * 34 + 10 + txs_in.len();
    // convert fee to satoshis to kb and mul
    let total_fee = Amount::from_sat((fee_rate * 1.0e5 * tx_byte_size as f64) as u64).to_btc();
    let total_fee_sat = match Amount::from_btc(total_fee) {
        Ok(value) => value.to_sat(),
        Err(_err) => return Err("failed to convert total_fee to sat"),
    };

    // sub fee from out transaction
    if (txs_out.len() - 1) > 1 {
        let fee_for_each_tx = total_fee / (txs_out.len() - 1) as f64;
        let to_len = txs_out.len() - 1;

        for tx in &mut txs_out[..to_len] {
            let converted_fee_for_each_tx = match Amount::from_btc(fee_for_each_tx) {
                Ok(value) => value.to_sat(),
                Err(_err) => return Err("failed to convert fee_for_each_tx to sat"),
            };

            tx.value = tx.value - converted_fee_for_each_tx;
        }
    } else {
        txs_out[0].value = txs_out[0].value - total_fee_sat;
    }

    let tx = Transaction {
        version: 2,
        lock_time: PackedLockTime::ZERO,
        input: txs_in,
        output: txs_out,
    };

    let psbt = match Psbt::from_unsigned_tx(tx) {
        Ok(psbt) => psbt,
        Err(_err) => return Err("failed decode transaction to unsigned"),
    };

    Ok(encode::serialize_hex(&psbt.extract_tx()))
}
