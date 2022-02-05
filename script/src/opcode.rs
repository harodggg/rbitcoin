

// https://wiki.bitcoinsv.io/index.php/Opcodes_used_in_Bitcoin_Script
// bitcoin operator code 
// u8 ,a word, one byte,256

enum OpCode { 
    //constans 
    // 0 false
    OP_0 = 0x00,
    OP_FLASE = 0x00,

    // 1 ~ 75
    OP_PUSHBYTES_1 = 0x01,
	OP_PUSHBYTES_2 = 0x02,
	OP_PUSHBYTES_3 = 0x03,
	OP_PUSHBYTES_4 = 0x04,
	OP_PUSHBYTES_5 = 0x05,
	OP_PUSHBYTES_6 = 0x06,
	OP_PUSHBYTES_7 = 0x07,
	OP_PUSHBYTES_8 = 0x08,
	OP_PUSHBYTES_9 = 0x09,
	OP_PUSHBYTES_10 = 0x0a,
	OP_PUSHBYTES_11 = 0x0b,
	OP_PUSHBYTES_12 = 0x0c,
	OP_PUSHBYTES_13 = 0x0d,
	OP_PUSHBYTES_14 = 0x0e,
	OP_PUSHBYTES_15 = 0x0f,
	OP_PUSHBYTES_16 = 0x10,
	OP_PUSHBYTES_17 = 0x11,
	OP_PUSHBYTES_18 = 0x12,
	OP_PUSHBYTES_19 = 0x13,
	OP_PUSHBYTES_20 = 0x14,
	OP_PUSHBYTES_21 = 0x15,
	OP_PUSHBYTES_22 = 0x16,
	OP_PUSHBYTES_23 = 0x17,
	OP_PUSHBYTES_24 = 0x18,
	OP_PUSHBYTES_25 = 0x19,
	OP_PUSHBYTES_26 = 0x1a,
	OP_PUSHBYTES_27 = 0x1b,
	OP_PUSHBYTES_28 = 0x1c,
	OP_PUSHBYTES_29 = 0x1d,
	OP_PUSHBYTES_30 = 0x1e,
	OP_PUSHBYTES_31 = 0x1f,
	OP_PUSHBYTES_32 = 0x20,
	OP_PUSHBYTES_33 = 0x21,
	OP_PUSHBYTES_34 = 0x22,
	OP_PUSHBYTES_35 = 0x23,
	OP_PUSHBYTES_36 = 0x24,
	OP_PUSHBYTES_37 = 0x25,
	OP_PUSHBYTES_38 = 0x26,
	OP_PUSHBYTES_39 = 0x27,
	OP_PUSHBYTES_40 = 0x28,
	OP_PUSHBYTES_41 = 0x29,
	OP_PUSHBYTES_42 = 0x2a,
	OP_PUSHBYTES_43 = 0x2b,
	OP_PUSHBYTES_44 = 0x2c,
	OP_PUSHBYTES_45 = 0x2d,
	OP_PUSHBYTES_46 = 0x2e,
	OP_PUSHBYTES_47 = 0x2f,
	OP_PUSHBYTES_48 = 0x30,
	OP_PUSHBYTES_49 = 0x31,
	OP_PUSHBYTES_50 = 0x32,
	OP_PUSHBYTES_51 = 0x33,
	OP_PUSHBYTES_52 = 0x34,
	OP_PUSHBYTES_53 = 0x35,
	OP_PUSHBYTES_54 = 0x36,
	OP_PUSHBYTES_55 = 0x37,
	OP_PUSHBYTES_56 = 0x38,
	OP_PUSHBYTES_57 = 0x39,
	OP_PUSHBYTES_58 = 0x3a,
	OP_PUSHBYTES_59 = 0x3b,
	OP_PUSHBYTES_60 = 0x3c,
	OP_PUSHBYTES_61 = 0x3d,
	OP_PUSHBYTES_62 = 0x3e,
	OP_PUSHBYTES_63 = 0x3f,
	OP_PUSHBYTES_64 = 0x40,
	OP_PUSHBYTES_65 = 0x41,
	OP_PUSHBYTES_66 = 0x42,
	OP_PUSHBYTES_67 = 0x43,
	OP_PUSHBYTES_68 = 0x44,
	OP_PUSHBYTES_69 = 0x45,
	OP_PUSHBYTES_70 = 0x46,
	OP_PUSHBYTES_71 = 0x47,
	OP_PUSHBYTES_72 = 0x48,
	OP_PUSHBYTES_73 = 0x49,
	OP_PUSHBYTES_74 = 0x4a,
	OP_PUSHBYTES_75 = 0x4b,

    // 76 ~ 78, no OP_PUSHDATA3
    OP_PUSHDATA1 = 0x4c,
    OP_PUSHDATA2 = 0x4d,
    OP_PUSHDATA4 = 0x4f,

    // 79 
    OP_1NEGATE = 0x4f,

    // bool True on 0x50
    OP_1 = 0x51,
    OP_TRUE = 0x51,
    OP_2 = 0x52,
    OP_3 = 0x53,
    OP_4 = 0x54,
    OP_5 = 0x55,
    OP_6 = 0x56,
    OP_7 = 0x57,
    OP_8 = 0x58,
    OP_9 = 0x59,
    OP_10 = 0x5a,
    OP_11 = 0x5b,
    OP_12 = 0x5c,
    OP_13 = 0x5d,
    OP_14 = 0x5e,
    OP_15 = 0x5f,
    OP_16 = 0x60,

    // Flow control
    OP_NOP = 0x61,
	OP_VER = 0x62,
	OP_IF = 0x63,
	OP_NOTIF = 0x64,
	OP_VERIF = 0x65,
	OP_VERNOTIF = 0x66,
	OP_ELSE = 0x67,
	OP_ENDIF = 0x68,
	OP_VERIFY = 0x69,
	OP_RETURN = 0x6a,

    // Stack
    OP_TOALTSTACK = 0x6b,
	OP_FROMALTSTACK = 0x6c,
	OP_2DROP = 0x6d,
	OP_2DUP = 0x6e,
	OP_3DUP = 0x6f,
	OP_2OVER = 0x70,
	OP_2ROT = 0x71,
	OP_2SWAP = 0x72,
	OP_IFDUP = 0x73,
	OP_DEPTH = 0x74,
	OP_DROP = 0x75,
	OP_DUP = 0x76,
	OP_NIP = 0x77,
	OP_OVER = 0x78,
	OP_PICK = 0x79,
	OP_ROLL = 0x7a,
	OP_ROT = 0x7b,
	OP_SWAP = 0x7c,
	OP_TUCK = 0x7d,

    // 
}
// Self is  a type, &self === self: &Self, &mut self = self: &mut Self，有self必须是对象，是对象，必须被初始化有new,或者default方法。
impl OpCode { 
    fn from_u8（u: u8) -> Option<Self> { 
        match u { 
            0x00 => Some(OP_0),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::opcode::*;

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(OpCode::OP_0,OpCode::from_u8(0));
    }

}

