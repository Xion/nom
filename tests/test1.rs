#![cfg(feature = "stream")]

#[macro_use]
extern crate nom;

use nom::{IResult,Producer,FileProducer,not_line_ending};

use std::str;
use std::fmt::Debug;

#[test]
#[allow(unused_must_use)]
fn tag() {
  FileProducer::new("assets/links.txt", 20).map(|producer: FileProducer| {
    let mut p = producer;
    p.refill();

    consumer_from_parser!(PrintConsumer<()>, flat_map!(map_res!(tag!("https!"), str::from_utf8), print));
    let mut cs = PrintConsumer::new();
    for _ in 1..4 {
      p.apply(&mut cs);
    }
  });
}

pub fn print<T: Debug>(input: T) -> IResult<T,()> {
  println!("{:?}", input);
  IResult::Done(input, ())
}


#[test]
fn is_not() {
  //is_not!(foo b"\r\n");
  named!(foo<&[u8],&[u8]>, is_not!(&b"\r\n"[..]));
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(foo(a), IResult::Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}

#[test]
fn exported_public_method_defined_by_macro() {
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(not_line_ending(a), IResult::Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}

#[test]
fn recognize_leaving_empty_input() {
  named!(number< &[u8] >,  recognize!(is_a!("0123456789")));

  assert_eq!(IResult::Done(&b""[..], &b"42"[..]), number(&b"42"[..]));
  assert_eq!(IResult::Done(&b""[..], &b"3"[..]), number(&b"3"[..]));
  assert_eq!(IResult::Done(&b""[..], &b"0"[..]), number(&b"0"[..]));
}
