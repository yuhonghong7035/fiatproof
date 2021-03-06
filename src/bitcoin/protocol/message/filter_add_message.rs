use std;

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for FilterAddMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x61, 0x64, 0x64, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for FilterAddMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.serialize_var_octets(ws, &self.data[..], ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDeserializee for FilterAddMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.deserialize_var_octets(rs, &mut self.data, ::std::usize::MAX));
      Ok(r)
   }
}
