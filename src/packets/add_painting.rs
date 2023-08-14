use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x16)]
pub struct AddPaintingPacket {
  pub entity_id: ZigZong,
  pub runtime_id: VarLong,
  pub position: Vec3f,
  pub direction: ZigZag,
  pub title: String,
}
