// PMK744

use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0x9a)]
pub struct PositionTrackingDbRequestPacket {
  #[napi(ts_type = "BroadcastAction")]
  pub action: u8,
  pub tracking_id: ZigZag,
}

#[napi]
pub enum BroadcastAction {
    Update,
    Destroy,
    NotFound,
}