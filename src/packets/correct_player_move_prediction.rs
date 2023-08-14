use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0xa1)]
pub struct CorrectPlayerMovePredictionPacket {
  pub position: Vec3f,
  pub delta: Vec3f,
  pub on_ground: bool,
  pub tick: VarLong
}
