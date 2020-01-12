use serde::{Deserialize, Deserializer, de::{Visitor}};
use std::fmt;

extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, PartialEq, Eq)]
pub struct WrapperOptionalI32(Option<i32>);

struct OptionalI32Visitor;
impl<'de> Visitor<'de> for OptionalI32Visitor {
  type Value = WrapperOptionalI32;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
      formatter.write_str("parse failed")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
      E: serde::de::Error,
  {
      let result = match v {
          "" => 0,
          other => other.parse::<i32>().expect("parse error!"),
      };

      Ok(WrapperOptionalI32(Some(result)))
  }
}

impl <'de>Deserialize<'de> for WrapperOptionalI32 {
  fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
  where
      D: Deserializer<'de>,
  {
    deserializer.deserialize_identifier(OptionalI32Visitor)
  }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn deserialize_unit_enum() {
    let result = vec![
        ("zero".to_owned(), WrapperOptionalI32(Some(0))),
        ("one".to_owned(), WrapperOptionalI32(Some(1))),
        ("two".to_owned(), WrapperOptionalI32(Some(2))),
        
    ];

    assert_eq!(
        serde_urlencoded::from_str("zero=&one=1&two=2"),
        Ok(result)
    );
}