#[derive(Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction<'a> {
   OP_0,

   OP_1NEGATE,
   OP_RESERVED,
   OP_1,
   OP_2,
   OP_3,
   OP_4,
   OP_5,
   OP_6,
   OP_7,
   OP_8,
   OP_9,
   OP_10,
   OP_11,
   OP_12,
   OP_13,
   OP_14,
   OP_15,
   OP_16,

   OP_NOP,
   OP_VER,
   OP_IF,
   OP_NOTIF,
   OP_VERIF,
   OP_VERNOTIF,
   OP_ELSE,
   OP_ENDIF,
   OP_VERIFY,
   OP_RETURN,

   OP_TOALTSTACK,
   OP_FROMALTSTACK,
   OP_2DROP,
   OP_2DUP,
   OP_3DUP,
   OP_2OVER,
   OP_2ROT,
   OP_2SWAP,
   OP_IFDUP,
   OP_DEPTH,
   OP_DROP,
   OP_DUP,
   OP_NIP,
   OP_OVER,
   OP_PICK,
   OP_ROLL,
   OP_ROT,
   OP_SWAP,
   OP_TUCK,

   OP_CAT,
   OP_SUBSTR,
   OP_LEFT,
   OP_RIGHT,
   OP_SIZE,

   OP_INVERT,
   OP_AND,
   OP_OR,
   OP_XOR,
   OP_EQUAL,
   OP_EQUALVERIFY,
   OP_RESERVED1,
   OP_RESERVED2,

   OP_1ADD,
   OP_1SUB,
   OP_2MUL,
   OP_2DIV,
   OP_NEGATE,
   OP_ABS,
   OP_NOT,
   OP_0NOTEQUAL,

   OP_ADD,
   OP_SUB,
   OP_MUL,
   OP_DIV,
   OP_MOD,
   OP_LSHIFT,
   OP_RSHIFT,

   OP_BOOLAND,
   OP_BOOLOR,
   OP_NUMEQUAL,
   OP_NUMEQUALVERIFY,
   OP_NUMNOTEQUAL,
   OP_LESSTHAN,
   OP_GREATERTHAN,
   OP_LESSTHANOREQUAL,
   OP_GREATERTHANOREQUAL,
   OP_MIN,
   OP_MAX,

   OP_WITHIN,

   OP_RIPEMD160,
   OP_SHA1,
   OP_SHA256,
   OP_HASH160,
   OP_HASH256,
   OP_CODESEPARATOR,
   OP_CHECKSIG,
   OP_CHECKSIGVERIFY,
   OP_CHECKMULTISIG,
   OP_CHECKMULTISIGVERIFY,

   OP_NOP1,
   OP_CHECKLOCKTIMEVERIFY,
   OP_CHECKSEQUENCEVERIFY,
   OP_NOP4,
   OP_NOP5,
   OP_NOP6,
   OP_NOP7,
   OP_NOP8,
   OP_NOP9,
   OP_NOP10,

   OP_SMALLINTEGER,
   OP_PUBKEYS,
   OP_PUBKEYHASH,
   OP_PUBKEY,

   OP_INVALIDOPCODE,

   VAR(usize, usize, &'a[u8]), // (x, len, data) where x is of PUSHDATAx
   FIX(usize, &'a[u8]),

   OP_UNUSED(u8),
}

use super::parser::Parsed;
pub fn make<'a>(bytecode:&'a [u8], parsed:&Parsed) -> Instruction<'a> {
   use super::opcode;
   let op = bytecode[parsed.opcode_offset];
   match op {
      opcode::OP_0 => Instruction::OP_0,
      opcode::OP_1NEGATE => Instruction::OP_1NEGATE,
      opcode::OP_RESERVED => Instruction::OP_RESERVED,
      opcode::OP_1 => Instruction::OP_1,
      opcode::OP_2 => Instruction::OP_2,
      opcode::OP_3 => Instruction::OP_3,
      opcode::OP_4 => Instruction::OP_4,
      opcode::OP_5 => Instruction::OP_5,
      opcode::OP_6 => Instruction::OP_6,
      opcode::OP_7 => Instruction::OP_7,
      opcode::OP_8 => Instruction::OP_8,
      opcode::OP_9 => Instruction::OP_9,
      opcode::OP_10 => Instruction::OP_10,
      opcode::OP_11 => Instruction::OP_11,
      opcode::OP_12 => Instruction::OP_12,
      opcode::OP_13 => Instruction::OP_13,
      opcode::OP_14 => Instruction::OP_14,
      opcode::OP_15 => Instruction::OP_15,
      opcode::OP_16 => Instruction::OP_16,

      opcode::OP_NOP => Instruction::OP_NOP,
      opcode::OP_VER => Instruction::OP_VER,
      opcode::OP_IF => Instruction::OP_IF,
      opcode::OP_NOTIF => Instruction::OP_NOTIF,
      opcode::OP_VERIF => Instruction::OP_VERIF,
      opcode::OP_VERNOTIF => Instruction::OP_VERNOTIF,
      opcode::OP_ELSE => Instruction::OP_ELSE,
      opcode::OP_ENDIF => Instruction::OP_ENDIF,
      opcode::OP_VERIFY => Instruction::OP_VERIFY,
      opcode::OP_RETURN => Instruction::OP_RETURN,

      opcode::OP_TOALTSTACK => Instruction::OP_TOALTSTACK,
      opcode::OP_FROMALTSTACK => Instruction::OP_FROMALTSTACK,
      opcode::OP_2DROP => Instruction::OP_2DROP,
      opcode::OP_2DUP => Instruction::OP_2DUP,
      opcode::OP_3DUP => Instruction::OP_3DUP,
      opcode::OP_2OVER => Instruction::OP_2OVER,
      opcode::OP_2ROT => Instruction::OP_2ROT,
      opcode::OP_2SWAP => Instruction::OP_2SWAP,
      opcode::OP_IFDUP => Instruction::OP_IFDUP,
      opcode::OP_DEPTH => Instruction::OP_DEPTH,
      opcode::OP_DROP => Instruction::OP_DROP,
      opcode::OP_DUP => Instruction::OP_DUP,
      opcode::OP_NIP => Instruction::OP_NIP,
      opcode::OP_OVER => Instruction::OP_OVER,
      opcode::OP_PICK => Instruction::OP_PICK,
      opcode::OP_ROLL => Instruction::OP_ROLL,
      opcode::OP_ROT => Instruction::OP_ROT,
      opcode::OP_SWAP => Instruction::OP_SWAP,
      opcode::OP_TUCK => Instruction::OP_TUCK,

      opcode::OP_CAT => Instruction::OP_CAT,
      opcode::OP_SUBSTR => Instruction::OP_SUBSTR,
      opcode::OP_LEFT => Instruction::OP_LEFT,
      opcode::OP_RIGHT => Instruction::OP_RIGHT,
      opcode::OP_SIZE => Instruction::OP_SIZE,

      opcode::OP_INVERT => Instruction::OP_INVERT,
      opcode::OP_AND => Instruction::OP_AND,
      opcode::OP_OR => Instruction::OP_OR,
      opcode::OP_XOR => Instruction::OP_XOR,
      opcode::OP_EQUAL => Instruction::OP_EQUAL,
      opcode::OP_EQUALVERIFY => Instruction::OP_EQUALVERIFY,
      opcode::OP_RESERVED1 => Instruction::OP_RESERVED1,
      opcode::OP_RESERVED2 => Instruction::OP_RESERVED2,

      opcode::OP_1ADD => Instruction::OP_1ADD,
      opcode::OP_1SUB => Instruction::OP_1SUB,
      opcode::OP_2MUL => Instruction::OP_2MUL,
      opcode::OP_2DIV => Instruction::OP_2DIV,
      opcode::OP_NEGATE => Instruction::OP_NEGATE,
      opcode::OP_ABS => Instruction::OP_ABS,
      opcode::OP_NOT => Instruction::OP_NOT,
      opcode::OP_0NOTEQUAL => Instruction::OP_0NOTEQUAL,

      opcode::OP_ADD => Instruction::OP_ADD,
      opcode::OP_SUB => Instruction::OP_SUB,
      opcode::OP_MUL => Instruction::OP_MUL,
      opcode::OP_DIV => Instruction::OP_DIV,
      opcode::OP_MOD => Instruction::OP_MOD,
      opcode::OP_LSHIFT => Instruction::OP_LSHIFT,
      opcode::OP_RSHIFT => Instruction::OP_RSHIFT,

      opcode::OP_BOOLAND => Instruction::OP_BOOLAND,
      opcode::OP_BOOLOR => Instruction::OP_BOOLOR,
      opcode::OP_NUMEQUAL => Instruction::OP_NUMEQUAL,
      opcode::OP_NUMEQUALVERIFY => Instruction::OP_NUMEQUALVERIFY,
      opcode::OP_NUMNOTEQUAL => Instruction::OP_NUMNOTEQUAL,
      opcode::OP_LESSTHAN => Instruction::OP_LESSTHAN,
      opcode::OP_GREATERTHAN => Instruction::OP_GREATERTHAN,
      opcode::OP_LESSTHANOREQUAL => Instruction::OP_LESSTHANOREQUAL,
      opcode::OP_GREATERTHANOREQUAL => Instruction::OP_GREATERTHANOREQUAL,
      opcode::OP_MIN => Instruction::OP_MIN,
      opcode::OP_MAX => Instruction::OP_MAX,

      opcode::OP_WITHIN => Instruction::OP_WITHIN,

      opcode::OP_RIPEMD160 => Instruction::OP_RIPEMD160,
      opcode::OP_SHA1 => Instruction::OP_SHA1,
      opcode::OP_SHA256 => Instruction::OP_SHA256,
      opcode::OP_HASH160 => Instruction::OP_HASH160,
      opcode::OP_HASH256 => Instruction::OP_HASH256,
      opcode::OP_CODESEPARATOR => Instruction::OP_CODESEPARATOR,
      opcode::OP_CHECKSIG => Instruction::OP_CHECKSIG,
      opcode::OP_CHECKSIGVERIFY => Instruction::OP_CHECKSIGVERIFY,
      opcode::OP_CHECKMULTISIG => Instruction::OP_CHECKMULTISIG,
      opcode::OP_CHECKMULTISIGVERIFY => Instruction::OP_CHECKMULTISIGVERIFY,

      opcode::OP_NOP1 => Instruction::OP_NOP1,
      opcode::OP_CHECKLOCKTIMEVERIFY => Instruction::OP_CHECKLOCKTIMEVERIFY,
      opcode::OP_CHECKSEQUENCEVERIFY => Instruction::OP_CHECKSEQUENCEVERIFY,
      opcode::OP_NOP4 => Instruction::OP_NOP4,
      opcode::OP_NOP5 => Instruction::OP_NOP5,
      opcode::OP_NOP6 => Instruction::OP_NOP6,
      opcode::OP_NOP7 => Instruction::OP_NOP7,
      opcode::OP_NOP8 => Instruction::OP_NOP8,
      opcode::OP_NOP9 => Instruction::OP_NOP9,
      opcode::OP_NOP10 => Instruction::OP_NOP10,

      opcode::OP_SMALLINTEGER => Instruction::OP_SMALLINTEGER,
      opcode::OP_PUBKEYS => Instruction::OP_PUBKEYS,
      opcode::OP_PUBKEYHASH => Instruction::OP_PUBKEYHASH,
      opcode::OP_PUBKEY => Instruction::OP_PUBKEY,

      opcode::OP_INVALIDOPCODE => Instruction::OP_INVALIDOPCODE,

      opcode::OP_PUSHDATAFIX_01 ... opcode::OP_PUSHDATA4 => {
         let datalen = parsed.data_len;
         let data    = &bytecode[parsed.data_offset .. (parsed.data_offset + parsed.data_len)];
         match op {
            opcode::OP_PUSHDATA1 => Instruction::VAR(1, datalen, data),
            opcode::OP_PUSHDATA2 => Instruction::VAR(2, datalen, data),
            opcode::OP_PUSHDATA4 => Instruction::VAR(4, datalen, data),
            _ => Instruction::FIX(datalen, data),
         }
      },
      
      _ => Instruction::OP_UNUSED(op),
   }
   }

impl <'a> ::std::fmt::Display for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      use super::opcode;
      match *self {
         Instruction::OP_0 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_0 as usize].name),
         Instruction::OP_1NEGATE => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_1NEGATE as usize].name),
         Instruction::OP_RESERVED => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RESERVED as usize].name),
         Instruction::OP_1 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_1 as usize].name),
         Instruction::OP_2 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2 as usize].name),
         Instruction::OP_3 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_3 as usize].name),
         Instruction::OP_4 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_4 as usize].name),
         Instruction::OP_5 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_5 as usize].name),
         Instruction::OP_6 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_6 as usize].name),
         Instruction::OP_7 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_7 as usize].name),
         Instruction::OP_8 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_8 as usize].name),
         Instruction::OP_9 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_9 as usize].name),
         Instruction::OP_10 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_10 as usize].name),
         Instruction::OP_11 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_11 as usize].name),
         Instruction::OP_12 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_12 as usize].name),
         Instruction::OP_13 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_13 as usize].name),
         Instruction::OP_14 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_14 as usize].name),
         Instruction::OP_15 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_15 as usize].name),
         Instruction::OP_16 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_16 as usize].name),

         Instruction::OP_NOP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP as usize].name),
         Instruction::OP_VER => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_VER as usize].name),
         Instruction::OP_IF => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_IF as usize].name),
         Instruction::OP_NOTIF => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOTIF as usize].name),
         Instruction::OP_VERIF => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_VERIF as usize].name),
         Instruction::OP_VERNOTIF => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_VERNOTIF as usize].name),
         Instruction::OP_ELSE => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ELSE as usize].name),
         Instruction::OP_ENDIF => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ENDIF as usize].name),
         Instruction::OP_VERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_VERIFY as usize].name),
         Instruction::OP_RETURN => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RETURN as usize].name),

         Instruction::OP_TOALTSTACK => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_TOALTSTACK as usize].name),
         Instruction::OP_FROMALTSTACK => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_FROMALTSTACK as usize].name),
         Instruction::OP_2DROP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2DROP as usize].name),
         Instruction::OP_2DUP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2DUP as usize].name),
         Instruction::OP_3DUP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_3DUP as usize].name),
         Instruction::OP_2OVER => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2OVER as usize].name),
         Instruction::OP_2ROT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2ROT as usize].name),
         Instruction::OP_2SWAP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2SWAP as usize].name),
         Instruction::OP_IFDUP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_IFDUP as usize].name),
         Instruction::OP_DEPTH => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_DEPTH as usize].name),
         Instruction::OP_DROP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_DROP as usize].name),
         Instruction::OP_DUP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_DUP as usize].name),
         Instruction::OP_NIP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NIP as usize].name),
         Instruction::OP_OVER => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_OVER as usize].name),
         Instruction::OP_PICK => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_PICK as usize].name),
         Instruction::OP_ROLL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ROLL as usize].name),
         Instruction::OP_ROT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ROT as usize].name),
         Instruction::OP_SWAP => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SWAP as usize].name),
         Instruction::OP_TUCK => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_TUCK as usize].name),

         Instruction::OP_CAT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CAT as usize].name),
         Instruction::OP_SUBSTR => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SUBSTR as usize].name),
         Instruction::OP_LEFT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_LEFT as usize].name),
         Instruction::OP_RIGHT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RIGHT as usize].name),
         Instruction::OP_SIZE => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SIZE as usize].name),

         Instruction::OP_INVERT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_INVERT as usize].name),
         Instruction::OP_AND => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_AND as usize].name),
         Instruction::OP_OR => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_OR as usize].name),
         Instruction::OP_XOR => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_XOR as usize].name),
         Instruction::OP_EQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_EQUAL as usize].name),
         Instruction::OP_EQUALVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_EQUALVERIFY as usize].name),
         Instruction::OP_RESERVED1 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RESERVED1 as usize].name),
         Instruction::OP_RESERVED2 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RESERVED2 as usize].name),

         Instruction::OP_1ADD => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_1ADD as usize].name),
         Instruction::OP_1SUB => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_1SUB as usize].name),
         Instruction::OP_2MUL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2MUL as usize].name),
         Instruction::OP_2DIV => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_2DIV as usize].name),
         Instruction::OP_NEGATE => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NEGATE as usize].name),
         Instruction::OP_ABS => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ABS as usize].name),
         Instruction::OP_NOT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOT as usize].name),
         Instruction::OP_0NOTEQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_0NOTEQUAL as usize].name),

         Instruction::OP_ADD => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_ADD as usize].name),
         Instruction::OP_SUB => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SUB as usize].name),
         Instruction::OP_MUL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_MUL as usize].name),
         Instruction::OP_DIV => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_DIV as usize].name),
         Instruction::OP_MOD => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_MOD as usize].name),
         Instruction::OP_LSHIFT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_LSHIFT as usize].name),
         Instruction::OP_RSHIFT => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RSHIFT as usize].name),

         Instruction::OP_BOOLAND => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_BOOLAND as usize].name),
         Instruction::OP_BOOLOR => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_BOOLOR as usize].name),
         Instruction::OP_NUMEQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NUMEQUAL as usize].name),
         Instruction::OP_NUMEQUALVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NUMEQUALVERIFY as usize].name),
         Instruction::OP_NUMNOTEQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NUMNOTEQUAL as usize].name),
         Instruction::OP_LESSTHAN => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_LESSTHAN as usize].name),
         Instruction::OP_GREATERTHAN => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_GREATERTHAN as usize].name),
         Instruction::OP_LESSTHANOREQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_LESSTHANOREQUAL as usize].name),
         Instruction::OP_GREATERTHANOREQUAL => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_GREATERTHANOREQUAL as usize].name),
         Instruction::OP_MIN => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_MIN as usize].name),
         Instruction::OP_MAX => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_MAX as usize].name),

         Instruction::OP_WITHIN => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_WITHIN as usize].name),

         Instruction::OP_RIPEMD160 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_RIPEMD160 as usize].name),
         Instruction::OP_SHA1 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SHA1 as usize].name),
         Instruction::OP_SHA256 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SHA256 as usize].name),
         Instruction::OP_HASH160 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_HASH160 as usize].name),
         Instruction::OP_HASH256 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_HASH256 as usize].name),
         Instruction::OP_CODESEPARATOR => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CODESEPARATOR as usize].name),
         Instruction::OP_CHECKSIG => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKSIG as usize].name),
         Instruction::OP_CHECKSIGVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKSIGVERIFY as usize].name),
         Instruction::OP_CHECKMULTISIG => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKMULTISIG as usize].name),
         Instruction::OP_CHECKMULTISIGVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKMULTISIGVERIFY as usize].name),

         Instruction::OP_NOP1 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP1 as usize].name),
         Instruction::OP_CHECKLOCKTIMEVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKLOCKTIMEVERIFY as usize].name),
         Instruction::OP_CHECKSEQUENCEVERIFY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_CHECKSEQUENCEVERIFY as usize].name),
         Instruction::OP_NOP4 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP4 as usize].name),
         Instruction::OP_NOP5 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP5 as usize].name),
         Instruction::OP_NOP6 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP6 as usize].name),
         Instruction::OP_NOP7 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP7 as usize].name),
         Instruction::OP_NOP8 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP8 as usize].name),
         Instruction::OP_NOP9 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP9 as usize].name),
         Instruction::OP_NOP10 => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_NOP10 as usize].name),

         Instruction::OP_SMALLINTEGER => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_SMALLINTEGER as usize].name),
         Instruction::OP_PUBKEYS => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_PUBKEYS as usize].name),
         Instruction::OP_PUBKEYHASH => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_PUBKEYHASH as usize].name),
         Instruction::OP_PUBKEY => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_PUBKEY as usize].name),

         Instruction::OP_INVALIDOPCODE => write!(f, "{}", opcode::OPCODE_INFO[opcode::OP_INVALIDOPCODE as usize].name),
         
         Instruction::VAR(v, len,_)  => write!(f, "[{}({})]", len, v),
         Instruction::FIX(len,_)  => write!(f, "[{}]", len),

         Instruction::OP_UNUSED(op) => write!(f, "UNUSED({})", op),
      }
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee};
impl <'a> Encodee for Instruction<'a> {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::opcode;
      match *self {
         Instruction::OP_0 => { r += try!(e.encode_u8(opcode::OP_0)); },
         Instruction::OP_1NEGATE => { r += try!(e.encode_u8(opcode::OP_1NEGATE)); },
         Instruction::OP_RESERVED => { r += try!(e.encode_u8(opcode::OP_RESERVED)); },
         Instruction::OP_1 => { r += try!(e.encode_u8(opcode::OP_1)); },
         Instruction::OP_2 => { r += try!(e.encode_u8(opcode::OP_2)); },
         Instruction::OP_3 => { r += try!(e.encode_u8(opcode::OP_3)); },
         Instruction::OP_4 => { r += try!(e.encode_u8(opcode::OP_4)); },
         Instruction::OP_5 => { r += try!(e.encode_u8(opcode::OP_5)); },
         Instruction::OP_6 => { r += try!(e.encode_u8(opcode::OP_6)); },
         Instruction::OP_7 => { r += try!(e.encode_u8(opcode::OP_7)); },
         Instruction::OP_8 => { r += try!(e.encode_u8(opcode::OP_8)); },
         Instruction::OP_9 => { r += try!(e.encode_u8(opcode::OP_9)); },
         Instruction::OP_10 => { r += try!(e.encode_u8(opcode::OP_10)); },
         Instruction::OP_11 => { r += try!(e.encode_u8(opcode::OP_11)); },
         Instruction::OP_12 => { r += try!(e.encode_u8(opcode::OP_12)); },
         Instruction::OP_13 => { r += try!(e.encode_u8(opcode::OP_13)); },
         Instruction::OP_14 => { r += try!(e.encode_u8(opcode::OP_14)); },
         Instruction::OP_15 => { r += try!(e.encode_u8(opcode::OP_15)); },
         Instruction::OP_16 => { r += try!(e.encode_u8(opcode::OP_16)); },

         Instruction::OP_NOP => { r += try!(e.encode_u8(opcode::OP_NOP)); },
         Instruction::OP_VER => { r += try!(e.encode_u8(opcode::OP_VER)); },
         Instruction::OP_IF => { r += try!(e.encode_u8(opcode::OP_IF)); },
         Instruction::OP_NOTIF => { r += try!(e.encode_u8(opcode::OP_NOTIF)); },
         Instruction::OP_VERIF => { r += try!(e.encode_u8(opcode::OP_VERIF)); },
         Instruction::OP_VERNOTIF => { r += try!(e.encode_u8(opcode::OP_VERNOTIF)); },
         Instruction::OP_ELSE => { r += try!(e.encode_u8(opcode::OP_ELSE)); },
         Instruction::OP_ENDIF => { r += try!(e.encode_u8(opcode::OP_ENDIF)); },
         Instruction::OP_VERIFY => { r += try!(e.encode_u8(opcode::OP_VERIFY)); },
         Instruction::OP_RETURN => { r += try!(e.encode_u8(opcode::OP_RETURN)); },

         Instruction::OP_TOALTSTACK => { r += try!(e.encode_u8(opcode::OP_TOALTSTACK)); },
         Instruction::OP_FROMALTSTACK => { r += try!(e.encode_u8(opcode::OP_FROMALTSTACK)); },
         Instruction::OP_2DROP => { r += try!(e.encode_u8(opcode::OP_2DROP)); },
         Instruction::OP_2DUP => { r += try!(e.encode_u8(opcode::OP_2DUP)); },
         Instruction::OP_3DUP => { r += try!(e.encode_u8(opcode::OP_3DUP)); },
         Instruction::OP_2OVER => { r += try!(e.encode_u8(opcode::OP_2OVER)); },
         Instruction::OP_2ROT => { r += try!(e.encode_u8(opcode::OP_2ROT)); },
         Instruction::OP_2SWAP => { r += try!(e.encode_u8(opcode::OP_2SWAP)); },
         Instruction::OP_IFDUP => { r += try!(e.encode_u8(opcode::OP_IFDUP)); },
         Instruction::OP_DEPTH => { r += try!(e.encode_u8(opcode::OP_DEPTH)); },
         Instruction::OP_DROP => { r += try!(e.encode_u8(opcode::OP_DROP)); },
         Instruction::OP_DUP => { r += try!(e.encode_u8(opcode::OP_DUP)); },
         Instruction::OP_NIP => { r += try!(e.encode_u8(opcode::OP_NIP)); },
         Instruction::OP_OVER => { r += try!(e.encode_u8(opcode::OP_OVER)); },
         Instruction::OP_PICK => { r += try!(e.encode_u8(opcode::OP_PICK)); },
         Instruction::OP_ROLL => { r += try!(e.encode_u8(opcode::OP_ROLL)); },
         Instruction::OP_ROT => { r += try!(e.encode_u8(opcode::OP_ROT)); },
         Instruction::OP_SWAP => { r += try!(e.encode_u8(opcode::OP_SWAP)); },
         Instruction::OP_TUCK => { r += try!(e.encode_u8(opcode::OP_TUCK)); },

         Instruction::OP_CAT => { r += try!(e.encode_u8(opcode::OP_CAT)); },
         Instruction::OP_SUBSTR => { r += try!(e.encode_u8(opcode::OP_SUBSTR)); },
         Instruction::OP_LEFT => { r += try!(e.encode_u8(opcode::OP_LEFT)); },
         Instruction::OP_RIGHT => { r += try!(e.encode_u8(opcode::OP_RIGHT)); },
         Instruction::OP_SIZE => { r += try!(e.encode_u8(opcode::OP_SIZE)); },

         Instruction::OP_INVERT => { r += try!(e.encode_u8(opcode::OP_INVERT)); },
         Instruction::OP_AND => { r += try!(e.encode_u8(opcode::OP_AND)); },
         Instruction::OP_OR => { r += try!(e.encode_u8(opcode::OP_OR)); },
         Instruction::OP_XOR => { r += try!(e.encode_u8(opcode::OP_XOR)); },
         Instruction::OP_EQUAL => { r += try!(e.encode_u8(opcode::OP_EQUAL)); },
         Instruction::OP_EQUALVERIFY => { r += try!(e.encode_u8(opcode::OP_EQUALVERIFY)); },
         Instruction::OP_RESERVED1 => { r += try!(e.encode_u8(opcode::OP_RESERVED1)); },
         Instruction::OP_RESERVED2 => { r += try!(e.encode_u8(opcode::OP_RESERVED2)); },

         Instruction::OP_1ADD => { r += try!(e.encode_u8(opcode::OP_1ADD)); },
         Instruction::OP_1SUB => { r += try!(e.encode_u8(opcode::OP_1SUB)); },
         Instruction::OP_2MUL => { r += try!(e.encode_u8(opcode::OP_2MUL)); },
         Instruction::OP_2DIV => { r += try!(e.encode_u8(opcode::OP_2DIV)); },
         Instruction::OP_NEGATE => { r += try!(e.encode_u8(opcode::OP_NEGATE)); },
         Instruction::OP_ABS => { r += try!(e.encode_u8(opcode::OP_ABS)); },
         Instruction::OP_NOT => { r += try!(e.encode_u8(opcode::OP_NOT)); },
         Instruction::OP_0NOTEQUAL => { r += try!(e.encode_u8(opcode::OP_0NOTEQUAL)); },

         Instruction::OP_ADD => { r += try!(e.encode_u8(opcode::OP_ADD)); },
         Instruction::OP_SUB => { r += try!(e.encode_u8(opcode::OP_SUB)); },
         Instruction::OP_MUL => { r += try!(e.encode_u8(opcode::OP_MUL)); },
         Instruction::OP_DIV => { r += try!(e.encode_u8(opcode::OP_DIV)); },
         Instruction::OP_MOD => { r += try!(e.encode_u8(opcode::OP_MOD)); },
         Instruction::OP_LSHIFT => { r += try!(e.encode_u8(opcode::OP_LSHIFT)); },
         Instruction::OP_RSHIFT => { r += try!(e.encode_u8(opcode::OP_RSHIFT)); },

         Instruction::OP_BOOLAND => { r += try!(e.encode_u8(opcode::OP_BOOLAND)); },
         Instruction::OP_BOOLOR => { r += try!(e.encode_u8(opcode::OP_BOOLOR)); },
         Instruction::OP_NUMEQUAL => { r += try!(e.encode_u8(opcode::OP_NUMEQUAL)); },
         Instruction::OP_NUMEQUALVERIFY => { r += try!(e.encode_u8(opcode::OP_NUMEQUALVERIFY)); },
         Instruction::OP_NUMNOTEQUAL => { r += try!(e.encode_u8(opcode::OP_NUMNOTEQUAL)); },
         Instruction::OP_LESSTHAN => { r += try!(e.encode_u8(opcode::OP_LESSTHAN)); },
         Instruction::OP_GREATERTHAN => { r += try!(e.encode_u8(opcode::OP_GREATERTHAN)); },
         Instruction::OP_LESSTHANOREQUAL => { r += try!(e.encode_u8(opcode::OP_LESSTHANOREQUAL)); },
         Instruction::OP_GREATERTHANOREQUAL => { r += try!(e.encode_u8(opcode::OP_GREATERTHANOREQUAL)); },
         Instruction::OP_MIN => { r += try!(e.encode_u8(opcode::OP_MIN)); },
         Instruction::OP_MAX => { r += try!(e.encode_u8(opcode::OP_MAX)); },

         Instruction::OP_WITHIN => { r += try!(e.encode_u8(opcode::OP_WITHIN)); },

         Instruction::OP_RIPEMD160 => { r += try!(e.encode_u8(opcode::OP_RIPEMD160)); },
         Instruction::OP_SHA1 => { r += try!(e.encode_u8(opcode::OP_SHA1)); },
         Instruction::OP_SHA256 => { r += try!(e.encode_u8(opcode::OP_SHA256)); },
         Instruction::OP_HASH160 => { r += try!(e.encode_u8(opcode::OP_HASH160)); },
         Instruction::OP_HASH256 => { r += try!(e.encode_u8(opcode::OP_HASH256)); },
         Instruction::OP_CODESEPARATOR => { r += try!(e.encode_u8(opcode::OP_CODESEPARATOR)); },
         Instruction::OP_CHECKSIG => { r += try!(e.encode_u8(opcode::OP_CHECKSIG)); },
         Instruction::OP_CHECKSIGVERIFY => { r += try!(e.encode_u8(opcode::OP_CHECKSIGVERIFY)); },
         Instruction::OP_CHECKMULTISIG => { r += try!(e.encode_u8(opcode::OP_CHECKMULTISIG)); },
         Instruction::OP_CHECKMULTISIGVERIFY => { r += try!(e.encode_u8(opcode::OP_CHECKMULTISIGVERIFY)); },

         Instruction::OP_NOP1 => { r += try!(e.encode_u8(opcode::OP_NOP1)); },
         Instruction::OP_CHECKLOCKTIMEVERIFY => { r += try!(e.encode_u8(opcode::OP_CHECKLOCKTIMEVERIFY)); },
         Instruction::OP_CHECKSEQUENCEVERIFY => { r += try!(e.encode_u8(opcode::OP_CHECKSEQUENCEVERIFY)); },
         Instruction::OP_NOP4 => { r += try!(e.encode_u8(opcode::OP_NOP4)); },
         Instruction::OP_NOP5 => { r += try!(e.encode_u8(opcode::OP_NOP5)); },
         Instruction::OP_NOP6 => { r += try!(e.encode_u8(opcode::OP_NOP6)); },
         Instruction::OP_NOP7 => { r += try!(e.encode_u8(opcode::OP_NOP7)); },
         Instruction::OP_NOP8 => { r += try!(e.encode_u8(opcode::OP_NOP8)); },
         Instruction::OP_NOP9 => { r += try!(e.encode_u8(opcode::OP_NOP9)); },
         Instruction::OP_NOP10 => { r += try!(e.encode_u8(opcode::OP_NOP10)); },

         Instruction::OP_SMALLINTEGER => { r += try!(e.encode_u8(opcode::OP_SMALLINTEGER)); },
         Instruction::OP_PUBKEYS => { r += try!(e.encode_u8(opcode::OP_PUBKEYS)); },
         Instruction::OP_PUBKEYHASH => { r += try!(e.encode_u8(opcode::OP_PUBKEYHASH)); },
         Instruction::OP_PUBKEY => { r += try!(e.encode_u8(opcode::OP_PUBKEY)); },

         Instruction::OP_INVALIDOPCODE => { r += try!(e.encode_u8(opcode::OP_INVALIDOPCODE)); },
         
         Instruction::VAR(x, len, data)  => {
            //if len < 0 { encode_error!(format!("vardata is too short: {}", len)) }
            match x {
               1 => {
                  if (u8::max_value() as usize) < len { encode_error!(format!("vardata is longer than 1 byte: {}", len)) }
                  r += try!(e.encode_u8(opcode::OP_PUSHDATA1));
                  r += try!(e.encode_u8(len as u8));
               },
               2 => {
                  if (u16::max_value() as usize) < len { encode_error!(format!("vardata is longer than 2 bytes: {}", len)) }
                  r += try!(e.encode_u8(opcode::OP_PUSHDATA2));
                  r += try!(e.encode_u16le(len as u16));
               },
               4 => {
                  if (u32::max_value() as usize) < len { encode_error!(format!("vardata is longer than 4 bytes: {}", len)) }
                  r += try!(e.encode_u8(opcode::OP_PUSHDATA4));
                  r += try!(e.encode_u32le(len as u32));
               },
               _ => { encode_error!(format!("var datalen is invalid: {}", x)) },
            }
            r += try!(e.encode_array_u8(&data[..len]));
         },
         Instruction::FIX(len, data)  => {
            if 0x4B < len { encode_error!(format!("fixdata is too long: {}", len)) }
            r += try!(e.encode_u8((opcode::OP_PUSHDATAFIX_01 + (len as u8) - 1) as u8));
            r += try!(e.encode_array_u8(&data[..len]));
         },

         Instruction::OP_UNUSED(op) => encode_error!(format!("UNUSED_OP: 0x{:x}",op)),
      }
      Ok(r)
   }
}

