use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x17)]
pub struct TickSyncPacket {
  pub request_time: LI64,
  pub response_time: LI64,
}
