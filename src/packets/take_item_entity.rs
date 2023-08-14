use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x11)]
pub struct TakeItemEntityPacket {
  pub runtime_id: VarLong,
  pub target: VarInt,
}
