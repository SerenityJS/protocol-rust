use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x9f)]
pub struct CameraShakePacket {
  pub intensity: LF32,
  pub duration: LF32,
  pub shake_type: u8,
  #[napi(ts_type = "ShakeAction")]
  pub action: u8,
}

pub enum ShakeAction {
  Add,
  Stop,
}
