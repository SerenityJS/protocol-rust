use protocol_derive::packet;
use crate::binary::prelude::ZigZag;

#[packet(0x45)]
pub struct RequestChunkRadiusPacket {
  pub chunk_radius: ZigZag,
  pub max_radius: u8,
}
