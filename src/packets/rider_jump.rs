use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x14)]
pub struct RiderJumpPacket {
  pub jump_strength: ZigZag,
}
