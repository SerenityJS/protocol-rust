use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x28)]
pub struct SetEntityMotionPacket {
  pub runtime_id: VarLong,
  pub velocity: Vec3f,
}
