use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct PlayerAttribute {
  pub min: LF32,
  pub max: LF32,
  pub current: LF32,
  pub default: LF32,
  pub name: String,
  #[VarInt] pub modifiers: Vec<AttributeModifier>
}

#[packet]
pub struct AttributeModifier {
  pub id: String,
  pub name: String,
  pub amount: LF32,
  pub operation: LI32,
  pub operand: LI32,
  pub serialize: bool,
}