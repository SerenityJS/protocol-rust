use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x9d)]
pub struct MotionPredictionHintsPacket {
  pub entity_runtime_id: VarLong,
  pub velocity: Vec3f,
  pub on_ground: bool,
}
