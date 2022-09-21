use secp256k1::{PublicKey, Secp256k1};

use super::{chaincfg::Params as ChainParams, utils::last_index_byte};

// NewAddressPubKey returns a new AddressPubKey which represents a pay-to-pubkey
// address.  The serializedPubKey parameter must be a valid pubkey and can be
// uncompressed, compressed, or hybrid.

// func NewAddressPubKey(serializedPubKey []byte, net *chaincfg.Params) (*AddressPubKey, error) {
// pubKey, err := btcec.ParsePubKey(serializedPubKey, btcec.S256())
// if err != nil {
// 	return nil, err
// }

// 	// Set the format of the pubkey.  This probably should be returned
// 	// from btcec, but do it here to avoid API churn.  We already know the
// 	// pubkey is valid since it parsed above, so it's safe to simply examine
// 	// the leading byte to get the format.
// 	pkFormat := PKFUncompressed
// 	switch serializedPubKey[0] {
// 	case 0x02, 0x03:
// 		pkFormat = PKFCompressed
// 	case 0x06, 0x07:
// 		pkFormat = PKFHybrid
// 	}

// 	return &AddressPubKey{
// 		pubKeyFormat: pkFormat,
// 		pubKey:       pubKey,
// 		pubKeyHashID: net.PubKeyHashAddrID,
// 	}, nil
// }

fn ParsePubKey(serialized_pub_key: Vec<u8>, net: ChainParams) {
    let pub_key = match PublicKey::from_slice(&serialized_pub_key) {
        Ok(key) => key,
        Err(err) => panic!("{}", err),
    };
}

pub fn decode_address(address: &str, defaultNet: ChainParams) {
    let oneIndex = last_index_byte(address, b'1');

    if oneIndex > 1 {
        let prefix = &address[..oneIndex as usize + 1 as usize];
        println!("{}", &prefix);

        todo!()
    }

    // Serialized public keys are either 65 bytes (130 hex chars) if
    // uncompressed/hybrid or 33 bytes (66 hex chars) if compressed.
    if address.len() == 130 || address.len() == 66 {
        // serializedPubKey, err := hex.DecodeString(addr)
        // if err != nil {
        // 	return nil, err
        // }
        // return NewAddressPubKey(serializedPubKey, defaultNet)

        let serializedPubKey = match hex::decode(address) {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        };

        // return NewAddressPubKey(serializedPubKey, defaultNet);
    }
}
