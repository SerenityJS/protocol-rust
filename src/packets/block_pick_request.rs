use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x22)]
pub struct BlockPickRequestPacket {
  pub x: ZigZag,
  pub y: ZigZag,
  pub z: ZigZag,
  pub with_data: bool,
  pub selected_slot: u8,
}
