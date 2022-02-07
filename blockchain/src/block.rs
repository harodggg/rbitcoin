use transaction;

struct Block { 
	//块版本，version 
	version: u32,
	// 计算的nonce.
	nonce: u32,
	// 块时间
	timestramp: u32,
	// 块难度
	bit: u32,
	prev_block_hash:H256,
	
	merkle_hash:H256,

	transaction: vec<Transaction>,
}

impl Block { 
	fn default(version:u32,nonce:u32,
		timestramp:u32,bit:u32,
		prev_block_hash:H256,
		merkle_hash:H256,
		transaction: vec<Transaction>) -> Self { 

		Block { 


		}
	}

	fn build_merkle_root(&self) -> H256 { 

	}

	fn hash(&self) -> H256 { 

	}
}

