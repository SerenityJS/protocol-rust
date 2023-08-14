use protocol_derive::packet;
use crate::binary::prelude::ZigZag;

#[packet(0x46)]
pub struct ChunkRadiusUpdatePacket {
  pub chunk_radius: ZigZag,
}
