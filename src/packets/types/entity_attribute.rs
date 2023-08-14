use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct EntityAttribute {
  pub name: String,
  pub min: LF32,
  pub max: LF32,
  pub value: LF32,
}
