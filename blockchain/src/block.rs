


struct BlockHeader { 
	version: u32,
	timestamp: u64,
	bits: u32,
	HashPrevBlock: u256,
	HashMerkleBlock: u256,
	nonce: u32,
}

struct Transaction { 
	inputs: Vec![TransactionInput],
	vouts: Vec![TransactionOutput],
	
}

struct TransactionInput { 

	scriptSig: Bytes,

}
struct TransactionOutput { 
	value:u64,
	scriptPubKey: Bytes,

}

struct Block { 
	blockheader: BlockHeader,
	transactions: &Transaction
}


trait Block { 
fn defalut() -> Block { 

}

}
