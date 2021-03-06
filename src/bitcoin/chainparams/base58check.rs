pub struct Versions<'a> {
   pub p2pkh:      &'a [u8],
   pub p2sh:       &'a [u8],
   pub secret_key: &'a [u8],
   pub xpub:       &'a [u8],
   pub xprv:       &'a [u8],
}

pub struct Base58check<'a> {
   pub table:   &'a str,
   pub versions: Versions<'a>,
}

