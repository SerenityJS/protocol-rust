use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x19)]
pub struct LevelEventPacket {
  pub event: ZigZag,
  pub position: Vec3f,
  pub data: ZigZag,
}
