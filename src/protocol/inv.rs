use std;
extern crate time;
use ::UInt256;

#[derive(Debug,Clone,PartialEq)]
pub enum InvType {
   Unknown       = 0,
   Tx            = 1,
   Block         = 2,
   FilteredBlock = 3,
}

impl Default for InvType {
   fn default() -> Self { InvType::Unknown }
}

impl std::fmt::Display for InvType {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match *self {
         InvType::Tx            => write!(f, "tx"),
         InvType::Block         => write!(f, "block"),
         InvType::FilteredBlock => write!(f, "filtered"),
         _ => write!(f, "{}", *self),
      }
   }
}
impl InvType {
   pub fn is_tx(&self)             -> bool { *self == InvType::Tx }
   pub fn is_block(&self)          -> bool { *self == InvType::Block }
   pub fn is_filtered_block(&self) -> bool { *self == InvType::FilteredBlock }
}


#[derive(Debug,Clone,Default)]
pub struct Inv {
   pub invtype: InvType,
   pub hash:    UInt256,
}
impl std::fmt::Display for Inv {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}:{}", self.invtype, self.hash)
   }
}
impl Inv {
   #[allow(dead_code)]
   pub fn new(invtype_:InvType, hash_: UInt256) -> Self {
      Inv {
         invtype: invtype_,
         hash:    hash_,
      }
   }
   pub fn new_tx(hash: UInt256)             -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)          -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filtered_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}