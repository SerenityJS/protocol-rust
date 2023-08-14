use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x23)]
pub struct EntityPickRequestPacket {
  pub runtime_id: LU64,
  pub selected_slot: u8,
  pub with_data: bool,
}
