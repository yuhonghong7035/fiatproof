use super::{UInt256, Script};

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct TxOutPoint {
   pub txid: UInt256,
   pub n:    u32,
}
impl TxOutPoint {
   pub fn new_null() -> Self {
      TxOutPoint {
         txid: UInt256::new_null(),
         n: ::std::u32::MAX
      }
   }
   pub fn set_null(&mut self) {
      self.txid.set_null();
      self.n = ::std::u32::MAX;
   }
   pub fn is_null(&self) -> bool {
      self.n == ::std::u32::MAX && self.txid.is_null()
   }
}

#[derive(Debug,Default,Clone)]
pub struct TxIn {
   pub prevout:    TxOutPoint,
   pub script_sig: Script,
   pub sequence:   u32,
}


impl TxIn {
   // Sequence 型を作るべきか
   pub const SEQUENCE_FINAL:u32                 = 0xFFFFFFFFu32;
   pub const SEQUENCE_LOCKTIME_DISABLE_FLAG:u32 = (1 << 31);
   pub const SEQUENCE_LOCKTIME_TYPE_FLAG:u32    = (1 << 22);
   pub const SEQUENCE_LOCKTIME_MASK:u32         = 0x0000FFFFu32;
   pub const SEQUENCE_GRANULARITY:u32           = 9;
   pub fn is_sequence_final(&self) -> bool {
      self.sequence == Self::SEQUENCE_FINAL
   }
   pub fn is_locktime_enable(&self) -> bool {
      (self.sequence & Self::SEQUENCE_LOCKTIME_DISABLE_FLAG) == 0
   }
   pub fn is_locktime_type(&self) -> bool {
      (self.sequence & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0
   }
   pub fn compare_sequence_locktime(l:u32, r:u32) -> Option<bool> {
      let l_is_blocktime = (l & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      let r_is_blocktime = (r & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      if l_is_blocktime ^ r_is_blocktime {
         None
      } else {
         Some((l & Self::SEQUENCE_LOCKTIME_MASK) > (r & Self::SEQUENCE_LOCKTIME_MASK))
      }
   }

   pub fn get_locktime_time(&self) -> Option<u64> {
      let v:u64 = ((self.sequence & Self::SEQUENCE_LOCKTIME_MASK) as u64) << Self::SEQUENCE_GRANULARITY;
      if v == 0 { None } else { Some(v-1) }
   }
   pub fn get_locktime_height(&self) -> Option<u32> {
      let v:u32 = self.sequence & Self::SEQUENCE_LOCKTIME_MASK;
      if v == 0 { None } else { Some(v-1) }
   }
}

impl ::std::fmt::Display for TxOutPoint {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "OutPoint(txid={}, n={})", self.txid, self.n)
   }
}
impl ::std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}


use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for TxOutPoint {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.encode(&(), e, ws));
      r += try!(e.encode_u32le(ws, self.n));
      Ok(r)
   }
}
impl BitcoinDecodee for TxOutPoint {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.decode(&(), d, rs));
      r += try!(d.decode_u32le(rs, &mut self.n));
      Ok(r)
   }
}

impl BitcoinEncodee for TxIn {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.encode(&(), e, ws));
      r += try!(self.script_sig.encode(&true, e, ws));
      r += try!(e.encode_u32le(ws, self.sequence));
      Ok(r)
   }
}
impl BitcoinDecodee for TxIn {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.decode(&(), d, rs));
      r += try!(self.script_sig.decode(&true, d, rs));
      r += try!(d.decode_u32le(rs, &mut self.sequence));
      Ok(r)
   }
}

