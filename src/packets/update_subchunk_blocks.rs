use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::*;
use napi::bindgen_prelude::*;
use crate::packets::types::*;

#[packet(0xac)]
pub struct UpdateSubchunkBlocksPacket {
  pub x: ZigZag,
  pub y: ZigZag,
  pub z: ZigZag,
  #[VarInt] pub blocks: Vec<BlockUpdate>
}

#[packet]
pub struct BlockUpdate {
  pub postion: BlockCoordinates,
  pub runtime_id: VarInt,
  pub flags: VarInt,
  pub entity_unique_id: ZigZong,
  #[napi(ts_type = "SubChunkTransitionType")]
  pub transition_type: VarInt,
}

#[napi]
pub enum SubChunkTransitionType {
  Entity,
  Create,
  Destroy,
}
