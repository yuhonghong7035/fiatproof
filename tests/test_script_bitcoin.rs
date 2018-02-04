#[macro_use] extern crate assert_matches;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rsbitcoin;

#[derive(Debug)]
struct EMessage(String);

macro_rules! impl_error {
   ($from:ty) => {
      impl From<$from> for EMessage {
         fn from(err: $from) -> EMessage {
            EMessage(format!("{:?}", err))
         }
      }
   }
}
impl_error!( serde_json::error::Error );

#[derive(Debug)]
struct Witness {
   pub witnesses: Vec<String>,
   pub amount: serde_json::Number,
}

#[derive(Debug)]
struct TestData {
   pub lineno: usize,
   pub witness: Option< Witness >,
   pub scriptSig: String,
   pub scriptPubKey: String,
   pub flags: String,
   pub expect: String,
   pub comments: String,
}

#[derive(Debug)]
enum TestCase {
   Comment(String),
   T(TestData),
}

fn as_string<'a>(v: &'a serde_json::Value) -> Result<&'a String, &'static str> {
   match v {
      &serde_json::Value::String(ref s) => Ok(s),
      _ => Err("not a string"),
   }
}
fn as_strings<'a>(v: &'a [serde_json::Value]) -> Result<Vec<&'a String>, &'static str> {
   v.iter().fold(Ok(Vec::new()), |acc,item| {
      match acc {
         Err(e) => Err(e),
         Ok(mut a) => {
            match item {
               &serde_json::Value::String(ref s) => {
                  a.push(s);
                  Ok(a)
               },
               _ => Err("not a string"),
            }
         }
      }
   })
}
fn as_strings_join<'a>(vv: &'a [serde_json::Value]) -> Result<String, &'static str> {
   as_strings(vv).and_then(|v| {
      let s = v.iter().fold(String::new(), |mut acc, item| {
         acc.push_str(item.as_str());
         acc
      });
      Ok(s)
   })
}

fn parse_testcase(v: &Vec<serde_json::Value>, lineno:usize) -> Result<TestCase, &'static str> {
   if v.len() == 1 {
      if let serde_json::Value::String(ref s) = v[0] {
         Ok(TestCase::Comment(s.clone()))
      } else {
         Err("unexpected comment type")
      }
   } else if let serde_json::Value::String(_) = v[0] {
      if v.len() < 4 {
         Err("no enough fields")
      } else {
         Ok(TestCase::T(TestData {
            lineno: lineno, 
            witness: None,
            scriptSig: as_string(&v[0])?.clone(),
            scriptPubKey: as_string(&v[1])?.clone(),
            flags: as_string(&v[2])?.clone(),
            expect: as_string(&v[3])?.clone(),
            comments: as_strings_join(&v[4..])?.clone(),
         }))
      }
   } else if let serde_json::Value::Array(ref v0) = v[0] {
      let len = v0.len();
      if len < 2 {
         Err("no enough witness fields")
      } else if let serde_json::Value::Number(ref n) = v0[len-1] {
         as_strings(&v0[0..(len-1)]).and_then(|witnesses| {
            Ok(TestCase::T(TestData {
               lineno: lineno,
               witness: Some(Witness {
                  witnesses: witnesses.into_iter().cloned().collect(),
                  amount: n.clone(),
               }),
               scriptSig: as_string(&v[1])?.clone(),
               scriptPubKey: as_string(&v[2])?.clone(),
               flags: as_string(&v[3])?.clone(),
               expect: as_string(&v[4])?.clone(),
               comments: as_strings_join(&v[5..])?.clone(),
            }))
         })
      } else {
         Err("no witness amount")
      }
   } else {
      Err("unexpected format")
   }
}

fn read_testcases() -> Result<Vec<TestCase>, String> {
   println!("cwd={}", ::std::env::current_dir().unwrap().display());
   let path = "tests/bitcoin-test-data/script_tests.json";
   let f = ::std::fs::File::open(path).unwrap();
   let lines:Vec< Vec<serde_json::Value> > = serde_json::from_reader(f).unwrap();
   lines.iter().enumerate().fold(Ok(Vec::new()), |acc, (n,s)| {
      match (acc, n, s) {
         (Err(e), _, _) => { Err(e) }
         (Ok(mut v), _, _) => {
            let r = parse_testcase(s, n+1);
            match r {
               Err(msg) => {
                  let msg = format!("{} at {}: {:?}", msg, n, s);
                  Err(msg)
               },
               Ok(tc) => {
                  v.push(tc);
                  Ok(v)
               }
            }
         }
      }
   })
}

use rsbitcoin::script::Flags;
fn parse_flags(input:&str) -> Flags {
   let flags = Flags {
      script_verify: rsbitcoin::script::flags::ScriptVerify::default(),
      sig_version:   rsbitcoin::script::flags::SigVersion::WitnessV0,
   };
   input.split(',').fold(flags, |mut acc,s| {
      match s {
         "" => (), 
         "P2SH" => {
            acc.script_verify = acc.script_verify.p2sh(true);
         },
         "STRICTENC" => {
            acc.script_verify = acc.script_verify.strict_enc(true);
         },
         "DERSIG" => {
            acc.script_verify = acc.script_verify.der_sig(true);
         },
         "LOW_S" => {
            acc.script_verify = acc.script_verify.low_s(true);
         },
         "NULLDUMMY" => {
            acc.script_verify = acc.script_verify.null_dummy(true);
         },
         "SIGPUSHONLY" => {
            acc.script_verify = acc.script_verify.sig_push_only(true);
         },
         "MINIMALDATA" => {
            acc.script_verify = acc.script_verify.minimal_data(true);
         },
         "DISCOURAGE_UPGRADABLE_NOPS" => {
            acc.script_verify = acc.script_verify.discourage_upgradable_nops(true);
         },
         "CLEANSTACK" => {
            acc.script_verify = acc.script_verify.clean_stack(true);
         },
         "CHECKLOCKTIMEVERIFY" => {
            acc.script_verify = acc.script_verify.check_locktime_verify(true);
         },
         "CHECKSEQUENCEVERIFY" => {
            acc.script_verify = acc.script_verify.check_sequence_verify(true);
         },
         "WITNESS" => {
            acc.script_verify = acc.script_verify.witness(true);
         },
         "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM" => {
            acc.script_verify = acc.script_verify.discourage_upgradable_witness_program(true);
         },
         "MINIMALIF" => {
            acc.script_verify = acc.script_verify.minimal_if(true);
         },
         "NULLFAIL" => {
            acc.script_verify = acc.script_verify.null_fail(true);
         },
         "WITNESS_PUBKEYTYPE" => {
            acc.script_verify = acc.script_verify.witness_pubkey_type(true);
         },
         _ => {
            assert!(false, format!("  unknown flags {}", s));
         }
      }
      acc
   })
}

#[test]
fn test_script_bitcoin() {
   let r = read_testcases();
   assert_matches!(r, Ok(_));
   let tests = r.unwrap();

   let dump = |head:&str, bytes:&[u8]| {
      use std::fmt::Write;
      let mut s = String::new();
      for b in bytes.into_iter() {
         let _ = write!(&mut s, "{:x} ", b);
      }
      println!("{}: {}", head, s);
   };
   
   let compile = |s:&str| {
      use rsbitcoin::script::compile;
      let r = compile(s);
      if r.is_err() {
         use std::error::Error;
         assert!(false, format!("  compile fail: script=\"{}\", err={}", s, r.unwrap_err().description()));
      }
      r.unwrap()
   };
   let verify = |sig:&[u8], pk:&[u8], flags:&Flags, expect:&str| {
      dump("sig", sig); dump("pk", pk); println!("expect={}", expect);
      use rsbitcoin::script::verify;
      let tx = rsbitcoin::Tx::default();
      let r = verify(sig, pk, &tx, 0, flags);
      use std::error::Error; //description()
      use rsbitcoin::Error::InterpretScript as IS;
      use rsbitcoin::script::InterpretErrorCode::*;
      match (expect, r) {
         ("OK", Ok(_)) => (),
         ("SIG_DER", Err(IS(ref e))) if e.code == SigDer as u32 => (),
         ("EVAL_FALSE", Err(IS(ref e))) if e.code == EvalFalse as u32 => (),
         ("BAD_OPCODE", Err(IS(ref e))) if e.code == BadOpcode as u32 => (),
         ("UNBALANCED_CONDITIONAL", Err(IS(ref e))) if e.code == UnbalancedConditional as u32 => (),
         (_, Ok(_)) => {
            assert!(false, format!("   verify fail: expect {} but OK", expect));
         },
         (_, Err(IS(ref e))) => {
            println!("{}", e.backtrace);
            assert!(false, format!("   verify fail: expect {} but {}", expect, e.description()));
         },
         (_, Err(ref e)) => {
            assert!(false, format!("   verify fail: expect {} but {}", expect, e.description()));
         },
      }
   };
   for tc in tests {
      match tc {
         TestCase::Comment(c) => {
            println!("{}", c);
         },
         TestCase::T(t) => {
            println!("test {}:{}, sig='{}', pk='{}', exp={}", t.lineno, t.comments, t.scriptSig, t.scriptPubKey, t.expect);
            let script_sig = compile(&t.scriptSig);
            let script_pk  = compile(&t.scriptPubKey);
            let flags = parse_flags(&t.flags);
            verify(script_sig.as_slice(), script_pk.as_slice(), &flags, t.expect.as_str());
         },
      }
   }
}


