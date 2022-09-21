const TWO_BITS_MASK: u32 = 0x3;
const FOUR_BITS_MASK: u32 = 0xf;
const SIX_BITS_MASK: u32 = 0x3f;
const EIGHT_BITS_MASK: u32 = 0xff;

// fieldVal implements optimized fixed-precision arithmetic over the
// secp256k1 finite field.  This means all arithmetic is performed modulo
// 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f.  It
// represents each 256-bit value as 10 32-bit integers in base 2^26.  This
// provides 6 bits of overflow in each word (10 bits in the most significant
// word) for a total of 64 bits of overflow (9*6 + 10 = 64).  It only implements
// the arithmetic needed for elliptic curve operations.
//
// The following depicts the internal representation:
// 	 -----------------------------------------------------------------
// 	|        n[9]       |        n[8]       | ... |        n[0]       |
// 	| 32 bits available | 32 bits available | ... | 32 bits available |
// 	| 22 bits for value | 26 bits for value | ... | 26 bits for value |
// 	| 10 bits overflow  |  6 bits overflow  | ... |  6 bits overflow  |
// 	| Mult: 2^(26*9)    | Mult: 2^(26*8)    | ... | Mult: 2^(26*0)    |
// 	 -----------------------------------------------------------------
//
// For example, consider the number 2^49 + 1.  It would be represented as:
// 	n[0] = 1
// 	n[1] = 2^23
// 	n[2..9] = 0
//
// The full 256-bit value is then calculated by looping i from 9..0 and
// doing sum(n[i] * 2^(26i)) like so:
// 	n[9] * 2^(26*9) = 0    * 2^234 = 0
// 	n[8] * 2^(26*8) = 0    * 2^208 = 0
// 	...
// 	n[1] * 2^(26*1) = 2^23 * 2^26  = 2^49
// 	n[0] * 2^(26*0) = 1    * 2^0   = 1
// 	Sum: 0 + 0 + ... + 2^49 + 1 = 2^49 + 1

#[derive(Debug, Clone, Copy)]
pub struct FieldVal {
    pub n: [u32; 10],
}

impl FieldVal {
    pub fn new() -> Self {
        Self { n: [0; 10] }
    }
}

impl FieldVal {
    // SetBytes packs the passed 32-byte big-endian value into the internal field
    // value representation.
    //
    // The field value is returned to support chaining.  This enables syntax like:
    // f := new(fieldVal).SetBytes(byteArray).Mul(f2) so that f = ba * f2.
    pub fn set_bytes(&mut self, b: [u8; 32]) -> &Self {
        // Pack the 256 total bits across the 10 uint32 words with a max of
        // 26-bits per word.  This could be done with a couple of for loops,
        // but this unrolled version is significantly faster.  Benchmarks show
        // this is about 34 times faster than the variant which uses loops.
        self.n[0] = b[31] as u32
            | (b[30] << 8) as u32
            | (b[29] << 16) as u32
            | (b[28] as u32 & TWO_BITS_MASK) << 24;

        self.n[1] = (b[28] >> 2) as u32
            | (b[27] << 6) as u32
            | (b[26] << 14) as u32
            | (b[25] as u32 & FOUR_BITS_MASK) << 22;

        self.n[2] = (b[25] >> 4) as u32
            | (b[24] << 4) as u32
            | (b[23] << 12) as u32
            | (b[22] as u32 & SIX_BITS_MASK) << 20;

        self.n[3] =
            (b[22] >> 6) as u32 | (b[21] << 2) as u32 | (b[20] << 10) as u32 | (b[19] << 18) as u32;

        self.n[4] = b[18] as u32
            | (b[17] << 8) as u32
            | (b[16] << 16) as u32
            | (b[15] as u32 & TWO_BITS_MASK) << 24;

        self.n[5] = (b[15] >> 2) as u32
            | (b[14] << 6) as u32
            | (b[13] << 14) as u32
            | (b[12] as u32 & FOUR_BITS_MASK) << 22;

        self.n[6] = (b[12] >> 4) as u32
            | (b[11] << 4) as u32
            | (b[10] << 12) as u32
            | (b[9] as u32 & SIX_BITS_MASK) << 20;

        self.n[7] =
            (b[9] >> 6) as u32 | (b[8] << 2) as u32 | (b[7] << 10) as u32 | (b[6] << 18) as u32;

        self.n[8] = b[5] as u32
            | (b[4] << 8) as u32
            | (b[3] << 16) as u32
            | (b[2] as u32 & TWO_BITS_MASK) << 24;

        self.n[9] = (b[2] >> 2) as u32 | (b[1] << 6) as u32 | (b[0] << 14) as u32;

        self
    }

    // SetByteSlice interprets the provided slice as a 256-bit big-endian unsigned
    // integer (meaning it is truncated to the first 32 bytes), packs it into the
    // internal field value representation, and returns the updated field value.
    //
    // Note that since passing a slice with more than 32 bytes is truncated, it is
    // possible that the truncated value is less than the field prime.  It is up to
    // the caller to decide whether it needs to provide numbers of the appropriate
    // size or if it is acceptable to use this function with the described
    // truncation behavior.
    //
    // The field value is returned to support chaining.  This enables syntax like:
    // f := new(fieldVal).SetByteSlice(byteSlice)
    pub fn set_bytes_slice(&mut self, mut b: Vec<u8>) -> &Self {
        let mut b32: [u8; 32] = [0; 32];
        if b.len() > 32 {
            b = b[..32].to_vec()
        }

        // copy(srcBytes[1:], src)
        // copy(b32[32-len(b):], b)
        // src_bytes[1..].copy_from_slice(src.as_bytes());
        b32[32 - b.len()..].copy_from_slice(&b);

        self.set_bytes(b32)
    }

    // SetHex decodes the passed big-endian hex string into the internal field value
    // representation.  Only the first 32-bytes are used.
    //
    // The field value is returned to support chaining.  This enables syntax like:
    // f := new(fieldVal).SetHex("0abc").Add(1) so that f = 0x0abc + 1
    pub fn set_hex(&mut self, mut hex_string: String) -> &Self {
        if hex_string.len() % 2 != 0 {
            hex_string = "0".to_string() + &hex_string;
        }

        // bytes, _ := hex.DecodeString(hexString)
        let bytes = match hex::decode(hex_string) {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        };

        self.set_bytes_slice(bytes)
    }
}
