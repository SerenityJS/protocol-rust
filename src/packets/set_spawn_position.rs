use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x2b)]
pub struct SetSpawnPositionPacket {
  pub spawn_type: ZigZag,
  pub player_position: BlockCoordinates,
  pub dimension: ZigZag,
  pub world_position: BlockCoordinates,
}
