use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x1a)]
pub struct BlockEventPacket {
  pub position: BlockCoordinates,
  #[napi(ts_type = "BlockEventType")]
  pub event_type: ZigZag,
  pub data: ZigZag,
}

#[napi]
pub enum BlockEventType {
  Sound,
  ChangeState,
}
