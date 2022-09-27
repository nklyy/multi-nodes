use std::str::FromStr;

use bitcoin::{
    consensus::encode, hashes::hex::FromHex, locktime::PackedLockTime, psbt::Psbt, Address, Amount,
    OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

use super::handler::{ToAddresses, Utxo};

pub fn create_transaction(utxos: &Vec<Utxo>, to: &Vec<ToAddresses>, change: &str, fee_rate: f64) {
    let tx_in_amount: f64 = utxos.iter().map(|utxo| utxo.amount.unwrap()).sum();
    let tx_out_amount: f64 = to.iter().map(|t| t.amount.unwrap()).sum();

    println!(
        "TX_IN_AMOUNT: {}, TX_OUT_AMOUNT: {}, FEE_RATE: {}",
        tx_in_amount, tx_out_amount, fee_rate
    );

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

    let mut txs_out: Vec<TxOut> = to
        .iter()
        .map(|t| TxOut {
            value: Amount::from_btc(t.amount.unwrap()).unwrap().to_sat(),
            script_pubkey: Address::from_str(t.to_address.as_ref().unwrap())
                .unwrap()
                .script_pubkey(),
        })
        .collect();

    if tx_out_amount > tx_in_amount {
        panic!("your balance too low for this transaction")
    }

    let change_amount = format!("{:.8}", (tx_in_amount - tx_out_amount));
    println!("change_amount FEE {:.8}", change_amount);

    txs_out.push(TxOut {
        value: Amount::from_btc(change_amount.parse().unwrap())
            .unwrap()
            .to_sat(),
        script_pubkey: Address::from_str(change).unwrap().script_pubkey(),
    });

    let tx_byte_size = txs_in.len() * 180 + txs_out.len() * 34 + 10 + txs_in.len();

    // convert fee to satoshis to kb and mul
    let total_fee = Amount::from_sat((fee_rate * 1.0e5 * tx_byte_size as f64) as u64).to_btc();
    println!("TOTAL FEE {}", total_fee);

    // sub fee from out transaction
    if txs_out.len() - 1 > 1 {
        let fee_for_each_tx = total_fee / (txs_out.len() - 1) as f64;

        let to_len = txs_out.len() - 1;
        txs_out[..to_len].iter_mut().for_each(|tx| {
            tx.value = tx.value - Amount::from_btc(fee_for_each_tx).unwrap().to_sat();
        });
    } else {
        txs_out[0].value = txs_out[0].value - Amount::from_btc(total_fee).unwrap().to_sat();
    }

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
