
struct KeyPair { 
    public: vec<u8>, 
    private: vec<u8>
}

impl KeyPair { 
    fn default() -> Self { 

        KeyPair { 

        }
    }

    fn to_address() -> String { 

        "address"
    }
}