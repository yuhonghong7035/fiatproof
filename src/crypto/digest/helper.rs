use super::Digest;
use ::std::borrow::{Borrow, BorrowMut};

pub trait Helpable {
   type D:Digest;
   fn create_digest() -> Self::D;
}

pub struct Helper<DH:Helpable> {
   pub digest: DH::D,
}

impl <DH:Helpable> Helper<DH> {
   pub fn new() -> Self {
      Self { digest: DH::create_digest() }
   }

   pub fn output_bits(&self)  -> usize { self.digest.output_bits() }
   pub fn output_bytes(&self) -> usize { self.digest.output_bytes() }
   pub fn block_size(&self)   -> usize { self.digest.block_size() }
   
   pub fn reset(&mut self) {
      self.digest.reset()
   }
   pub fn input(&mut self, input: &[u8]) {
      self.digest.input(input)
   }
   pub fn result(&mut self, out: &mut [u8]) {
      self.digest.result(out)
   }
   
   pub fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b(input).unwrap().as_ref())
   }
   pub fn input_hex_rev<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b_rev(input).unwrap().as_ref())
   }
   pub fn result_u8(&mut self) -> Box<[u8]> {
      let len = self.digest.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   pub fn result_hex(&mut self) -> String {
      ::utils::b2h(self.result_u8())
   }
   pub fn result_hex_rev(&mut self) -> String {
      ::utils::b2h_rev(self.result_u8())
   }

   pub fn u8_to_u8<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input(input.borrow());
      self.result_u8()
   }
   pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex()
   }
   pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex_rev()
   }
   pub fn hex_to_u8<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex(input.borrow());
      self.result_hex()
   }
   pub fn hex_to_u8_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_hex()
   }
}

macro_rules! def_helper {
   ($n:ident, $d:path) => {
      impl ::crypto::digest::helper::Helpable for $d {
         type D = $d;
         fn create_digest() -> Self::D {
            <$d>::new()
         }
      }
      pub type $n = ::crypto::digest::helper::Helper<$d>;
   };
}
def_helper!(Sha1Helper,      super::Sha1);
def_helper!(Sha256Helper,    super::Sha256);
def_helper!(Sha512Helper,    super::Sha512);
def_helper!(Ripemd160Helper, super::Ripemd160);


#[test]
fn test_sha512() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "3a9c593fc7d573a876aeec8303d4ef20cb62d055ee24f20334534b578b45dfd49924708385b9bbde280c2138f7f1dfd0ced554ad455a01b8ac8436043a2d6b5e";

   let mut d = super::Sha512Helper::new();
   assert_eq!(64, d.output_bytes());
   assert_eq!(expect, d.u8_to_hex(input));
}
