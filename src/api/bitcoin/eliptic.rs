use super::types::U256;

// CurveParams contains the parameters of an elliptic curve and also provides
// a generic, non-constant time implementation of Curve.
#[derive(Debug, Clone)]
pub struct CurveParams {
    pub p: U256, // the order of the underlying field
    pub n: U256, // the order of the base point
    pub b: U256, // the constant of the curve equation
    pub gx: U256,
    pub gy: U256,        // (x,y) of the base point
    pub bit_size: usize, // the size of the underlying field
    // H is the cofactor of the secp256k1 curve.
    pub h: usize,
    pub name: String, // the canonical name of the curve
}

impl CurveParams {
    pub fn new() -> CurveParams {
        CurveParams {
            p: U256::from("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"),
            n: U256::from("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141"),
            b: U256::from("0000000000000000000000000000000000000000000000000000000000000007"),
            gx: U256::from("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"),
            gy: U256::from("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"),
            bit_size: 256,
            h: 1,
            name: String::from("secp256k1"),
        }
    }
}
