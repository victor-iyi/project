#![allow(dead_code)]

use crate::error::Result;
use serde::Serialize;

pub(crate) fn parse<T: Serialize>(
  _content: &str,
  _variable: &T,
) -> Result<String> {
  unimplemented!()
}
