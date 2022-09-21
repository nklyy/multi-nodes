use super::{eliptic::CurveParams, filed::FieldVal, types::U256};

// KoblitzCurve supports a koblitz curve implementation that fits the ECC Curve
// interface from crypto/elliptic.
pub struct KoblitzCurve {
    curve_params: CurveParams,
    // // q is the value (P+1)/4 used to compute the square root of field
    // // elements.
    // q: U256,

    // h: usize,         // cofactor of the curve.
    // half_order: U256, // half the order N

    // // fieldB is the constant B of the curve as a fieldVal.
    // field_b: FieldVal,

    // // byteSize is simply the bit size / 8 and is provided for convenience
    // // since it is calculated repeatedly.
    // byte_size: usize,

    // // bytePoints
    // // bytePoints *[32][256][3]fieldVal

    // // The next 6 values are used specifically for endomorphism
    // // optimizations in ScalarMult.

    // // lambda must fulfill lambda^3 = 1 mod N where N is the order of G.
    // lambda: U256,

    // // beta must fulfill beta^3 = 1 mod P where P is the prime field of the
    // // curve.
    // beta: FieldVal,

    // // See the EndomorphismVectors in gensecp256k1.go to see how these are
    // // derived.
    // a1: U256,
    // b1: U256,
    // a2: U256,
    // b2: U256,
}

// S256 returns a Curve which implements secp256k1.
pub fn s256() -> KoblitzCurve {
    let curve_params = CurveParams::new();

    KoblitzCurve {
        curve_params: curve_params.clone(),
        //     q: U256::checked_div(
        //         U256::checked_add(curve_params.p.clone(), U256::from(1)).unwrap(),
        //         U256::from(4),
        //     )
        //     .unwrap(),
        //     h: 1,
        //     half_order: curve_params.n >> 1,
        //     field_b: *FieldVal::new().set_hex(
        //         "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
        //     ),
        //     byte_size: curve_params.bit_size.clone() / 8,

        //     lambda: U256::from("5363AD4CC05C30E0A5261C028812645A122E22EA20816678DF02967C1B23BD72"),
        //     beta: *FieldVal::new().set_hex(
        //         "7AE96A2B657C07106E64479EAC3434E99CF0497512F58995C1396C28719501EE".to_string(),
        //     ),
        //     a1: U256::from("3086D221A7D46BCDE86C90E49284EB15"),
        //     b1: U256::from("-E4437ED6010E88286F547FA90ABFE4C3"),
        //     a2: U256::from("114CA50F7A8E2F3F657C1108D9D44CFD8"),
        //     b2: U256::from("3086D221A7D46BCDE86C90E49284EB15"),
        // }
    }
}
