// PMK744

use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0x9a)]
pub struct PositionTrackingDBRequestPacket {
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