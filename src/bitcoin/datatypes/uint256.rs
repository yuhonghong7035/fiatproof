#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct UInt256 {
   pub data: [u8;32],
}

impl ::std::hash::Hash for UInt256 {
   fn hash<H: ::std::hash::Hasher>(&self, state:&mut H) {
      state.write(&self.data[..]);
   }
}

impl UInt256 {
   pub fn new(d: &[u8]) -> UInt256 {
      let mut v = UInt256 { data: [0u8;32] };
      v.data.clone_from_slice(&d[0..32]);
      v
   }
   pub fn new_rev(d: &[u8]) -> UInt256 {
      let mut v = UInt256 { data: [0u8;32] };
      for i in 0..32 {
         v.data[i] = d[31-i];
      }
      v
   }
   pub fn new_null() -> Self {
      UInt256 { data: [0u8;32] }
   }
   pub fn set_null(&mut self) {
      self.data.clone_from_slice(&[0u8;32]);
   }
   pub fn is_null(&self) -> bool {
      self.data == [0u8; 32]
   }
   pub fn as_slice(&self) -> &[u8] {
      &self.data[..]
   }
}

impl ::std::ops::Index<usize> for UInt256 {
   type Output = u8;
   fn index(&self, i:usize) -> &u8 {
      &self.data[i]
   }
}
impl ::std::ops::IndexMut<usize> for UInt256 {
   fn index_mut(&mut self, i:usize) -> &mut u8 {
      &mut self.data[i]
   }
}
impl ::std::fmt::Display for UInt256 {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match ::ui::bitcoin::serialize(self, &()).map(|b| ::utils::b2h_rev(b)) {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(_) => f.write_fmt(format_args!("err")),
      }
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for UInt256 {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      e.serialize_octets(ws, &self.data[..])
   }
}
impl BitcoinDeserializee for UInt256 {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      d.deserialize_octets(rs, &mut self.data[..])
   }
}

#[test]
fn test_str() {
   let s = "00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81";
   let uint256 = ::ui::bitcoin::hex_to_uint256(s).unwrap();

   let expect:[u8;32] = [
      0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
      0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
   ];
   assert_eq!(expect, uint256.data);

   let t = format!("{}", uint256);
   assert_eq!(s, t);
}

#[test]
fn test_serialize() {
   let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
               0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let v = UInt256::new(&data);
   let octets = ::ui::bitcoin::serialize(&v, &()).unwrap();
   assert_eq!(octets.len(), 32);
   assert_eq!(&octets[..], &data[..]);
}

#[test]
fn test_deserialize() {
   let data:Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                           0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];

   let mut v = UInt256::default();
   let r = ::ui::bitcoin::deserialize(data, &(), &mut v);
   assert_matches!(r, Ok(32));
   assert_eq!(&v.data[..], &v.data[..]);
}

