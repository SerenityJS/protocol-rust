use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x0a)]
pub struct SetTimePacket {
  pub time: ZigZag
}
