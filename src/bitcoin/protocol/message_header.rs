use super::apriori::COMMAND_LENGTH;

#[derive(Debug,Default,Clone)]
pub struct MessageHeader {
   pub magic:    u32,
   pub command:  [u8; COMMAND_LENGTH],
   pub length:   u32,
   pub checksum: u32,
}


use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for MessageHeader {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.serialize_u32le(ws, self.magic));
      r += try!(e.serialize_octets(ws, &self.command[..]));
      r += try!(e.serialize_u32le(ws, self.length));
      r += try!(e.serialize_u32le(ws, self.checksum));
      Ok(r)
   }
}
impl BitcoinDeserializee for MessageHeader {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.deserialize_u32le(rs, &mut self.magic));
      r += try!(d.deserialize_octets(rs, &mut self.command[..]));
      r += try!(d.deserialize_u32le(rs, &mut self.length));
      r += try!(d.deserialize_u32le(rs, &mut self.checksum));
      Ok(r)
   }
}


#[test]
fn test_message_header() {
   use super::apriori::COMMAND_LENGTH;
   const VERSION:[u8; COMMAND_LENGTH] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];
   let obj = MessageHeader {
      magic:    ::bitcoin::presets::bitcoin_mainnet::CHAIN.magic,
      command:  VERSION,
      length:   0x39,
      checksum: 0x12345678,
   };

   let mut w = ::iostream::VecWriteStream::default();
   {
      let m = ::bitcoin::serialize::Medium::new("net").unwrap();
      let e = ::bitcoin::serialize::Serializer::new(&m);
      assert_matches!(obj.serialize(&(), &e, &mut w), Ok(24usize));
   }
   assert_eq!(&w.get_ref()[..24],
              [0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12]);
}
