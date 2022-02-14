
fn dhash(bytes: &Bytes) -> Hash160 { 

}

fn dhash256(bytes: &Bytes) -> Hash256 { 

}

trait Hashable { 

    fn hash(&self) {

    }
}

impl Hashable for Address { 
    fn hash(&self) { 
        dhash160();
    }
}

impl Hashable for Block { 
    fn hash(&self) { 
        dhash256();
    }
}