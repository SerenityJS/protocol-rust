use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x2b)]
pub struct SetSpawnPositionPacket {
  #[napi(ts_type = "SpawnType")]
  pub spawn_type: ZigZag,
  pub player_position: BlockCoordinates,
  pub dimension: ZigZag,
  pub world_position: BlockCoordinates,
}

#[napi]
pub enum SpawnType {
  Player,
  World,
}
