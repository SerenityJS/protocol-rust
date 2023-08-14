use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x15)]
pub struct UpdateBlockPacket {
  pub position: BlockCoordinates,
  pub runtime_id: VarInt,
  // TODO: pub flags
  pub layer: VarInt,
}
