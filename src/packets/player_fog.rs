use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0xa0)]
pub struct PlayerFogPacket {
  #[VarInt] pub stack: Vec<String>
}
