use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x0e)]
pub struct RemoveEntityPacket {
  pub unique_id: ZigZong,
}
