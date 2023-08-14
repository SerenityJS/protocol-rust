use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x24)]
pub struct PlayerActionPacket {
  pub runtime_id: VarLong,
  // TODO: Write enum for this
  pub action: ZigZag,
  pub position: BlockCoordinates,
  pub result_position: BlockCoordinates,
  pub face: ZigZag,
}
