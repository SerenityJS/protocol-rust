// PMK744

use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x99)]
pub struct PositionTrackingDbBroadcastPacket {
  #[napi(ts_type = "BroadcastAction")]
  pub action: u8,
  pub tracking_id: ZigZag,
  // TODO: Figure out how to serialize this
  // pub nbt: NBT,
}
