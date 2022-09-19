const DEFAULT_TX_IN_OUT_ALLOC: usize = 15;

type TxWitness = Vec<Vec<u8>>;
type Hash = Vec<u8>;

#[derive(Debug)]
pub struct OutPoint {
    pub hash: Hash,
    pub index: u32,
}

impl OutPoint {
    pub fn new() -> OutPoint {
        OutPoint {
            hash: vec![Default::default(); 32],
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct TxIn {
    pub previous_out_point: OutPoint,
    pub signature_script: Option<Vec<u8>>,
    pub witness: Option<TxWitness>,
    pub sequence: u32,
}

impl TxIn {
    pub fn new() -> TxIn {
        TxIn {
            previous_out_point: OutPoint::new(),
            signature_script: Some(Vec::new()),
            witness: Some(Vec::new()),
            sequence: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: i64,
    pub pk_script: Vec<u8>,
}

impl TxOut {
    pub fn new() -> TxOut {
        TxOut {
            value: 0,
            pk_script: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MsgTx {
    pub version: i32,
    pub tx_in: Vec<TxIn>,
    pub tx_out: Vec<TxOut>,
}

impl MsgTx {
    pub fn new(version: i32) -> MsgTx {
        MsgTx {
            version: version,
            tx_in: std::iter::repeat_with(|| TxIn::new())
                .take(DEFAULT_TX_IN_OUT_ALLOC)
                .collect::<Vec<_>>(),
            tx_out: std::iter::repeat_with(|| TxOut::new())
                .take(DEFAULT_TX_IN_OUT_ALLOC)
                .collect::<Vec<_>>(),
        }
    }

    pub fn add_tx_in(&mut self, ti: TxIn) {
        self.tx_in.push(ti);
    }
}
