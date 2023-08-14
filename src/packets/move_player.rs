use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x13)]
pub struct MovePlayerPacket {
  pub runtime_id: VarInt,
  pub position: Vec3f,
  pub pitch: LF32,
  pub yaw: LF32,
  pub head_yaw: LF32,
  #[napi(ts_type = "MovePlayerMode")]
  pub mode: u8,
  pub on_ground: bool,
  pub riding_entity_runtime_id: VarInt,
  // TODO: pub teleport
}

#[napi]
pub enum MovePlayerMode {
  Normal,
  Reset,
  Teleport,
  Rotation,
}
