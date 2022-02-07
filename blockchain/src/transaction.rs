struct TxIn { 
    // nSequence,主要是用于判断相同输入的交易哪一个更新，值越大越新
    sequence: u32,
    script_sig: Script,
    prevout: OutPoint,
}

struct TxOut { 
    // 花费的钱
    value: u64,
    script_pubkey: Script,
}

struct OutPoint { 
    // 第几个交易。
    n: u32,
    hash: HASH256,
}


struct Transaction { 
    // version 用于后续升级
    version: u32,
    // locktime 脚本锁定时间
    locktime: u32,
    //交易输入，和输出
    inputs: vec<TxIn>,
    outputs: vec<TxOut>
}