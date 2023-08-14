use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct EntityInts {
  pub index: VarInt,
  pub value: ZigZag,
}

#[packet]
pub struct EntityFloats {
  pub index: VarInt,
  pub value: LF32,
}

#[packet]
pub struct EntityProperties {
  #[VarInt] pub ints: Vec<EntityInts>,
  #[VarInt] pub floats: Vec<EntityFloats>,
}