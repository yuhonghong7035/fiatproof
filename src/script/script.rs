use ::std;
use super::{opcode, Statement};

#[derive(Debug,Default,Clone)]
pub struct Script {
   bytecode:   Vec<u8>,
   statements: Vec<Statement>,
}
impl Script {
   pub fn new() -> Self {
      Script { bytecode: Vec::new(), statements: Vec::new() }
   }
   pub fn compile(v: Vec<u8>) -> ::Result<Self> {
      /*
      super::Parser(&v).parse().map(
         |statements| Script { bytecode: v, statements: statements }
      )
       */
      Ok(Script::new())
   }
   pub fn bytecode(&self) -> &Vec<u8> {
      &self.bytecode
   }
   pub fn statements(&self) -> &Vec<Statement> {
      &self.statements
   }
}
impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for s in self.statements.iter() {
         let _ = match s {
            &Statement::Value(x)     => write!(f, "[0x{:X}] ", x),
            &Statement::Bytes(ref x) => write!(f, "[({})] ", x.len()),
            &Statement::Op(x)        => write!(f, "{} ", opcode::OPCODE_INFO[x as usize].name),
         };
      }
      write!(f, "")
   }
}

use ::std::borrow::Borrow;
use ::encode::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for Script {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(self.bytecode().as_slice()));
      Ok(r)
   }
}

impl Decodee for Script {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut v:Vec<u8> = Vec::new();
      r += try!(d.decode_sequence_u8(&mut v));
      *self = try!(Script::compile(v));
      Ok(r)
   }
}


/*
impl ::ToBytes for Script {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      Ok(self.bytecode.clone())
   }
}

impl ::ToDigest for Script {
   fn to_digest_input(&self) -> ::Result<Vec<u8>> {
      Ok(self.bytecode.clone())
   }
}

impl ::FromBytes for Script {
   fn from_bytes<S: ::std::convert::AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      *self = try!(Script::compile(Vec::<u8>::from(s.as_ref())));
      Ok(())
   }
}
 */

