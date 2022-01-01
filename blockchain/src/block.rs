

struct BlockHeader { 
	version: u32,
	timestamp: u64,
	bits: u32,
	HashPrevBlock: u256,
	HashMerkleBlock: u256,
	nonce: u32,
}

struct Transaction { 
	version: i32,
	locktime: u32,
	inputs: Vec![TransactionInput],
	outputs: Vec![TransactionOutput],
}

struct TransactionInput { 
	pre_outpoint: OutPoint,
	scriptSig: Bytes,
}

struct TransactionOutput { 
	value:u64,
	scriptPubKey: Bytes,

}

struct OutPoint { 
	point: H256,
	index: u32,

}

struct Block { 
	blockheader: BlockHeader,
	transactions: &Transaction
}


trait Block { 
fn defalut() -> Block { 

}

}
