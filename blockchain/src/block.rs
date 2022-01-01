


struct BlockHeader { 
	version: u32,
	timestamp: u64,
	bits: u32,
	HashPrevBlock: u256,
	HashMerkleBlock: u256,
	nonce: u32,
}

struct Block { 
	blockheader: BlockHeader,
}


trait Block { 
fn defalut() -> Block { 

}

}
