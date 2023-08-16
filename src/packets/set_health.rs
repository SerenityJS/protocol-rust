use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x2a)]
pub struct SetHealthPacket {
  pub health: ZigZag,
}
