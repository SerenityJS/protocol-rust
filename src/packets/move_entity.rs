use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x12)]
pub struct MoveEntityPacket {
  pub runtime_id: VarLong,
  pub flags: u8,
  pub position: Vec3f,
  pub rotation: Rotation
}
