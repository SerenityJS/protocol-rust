use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0xa7)]
pub struct RemoveVolumeEntityPacket {
  pub runtime_id: VarLong,
}
