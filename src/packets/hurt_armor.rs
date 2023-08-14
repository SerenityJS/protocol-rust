use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x26)]
pub struct HurtArmorPacket {
  pub cause: ZigZag,
  pub damage: ZigZag,
  pub armor_slot: ZigZag,
}
