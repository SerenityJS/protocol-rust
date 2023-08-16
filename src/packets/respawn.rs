use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x2d)]
pub struct RespawnPacket {
  pub position: Vec3f,
  pub state: u8,
  pub runtime_id: VarLong,
}
