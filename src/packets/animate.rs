use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::prelude::*;

// TODO: Handle action id for boat_rowing_time
#[packet(0x2c)]
pub struct AnimatePacket {
  #[napi(ts_type = "AnimateActions")]
  pub action: ZigZag,
  pub runtime_id: VarLong,
}

#[napi]
pub enum AnimateActions {
  None = 0,
  SwingArm = 1,
  Unknown = 2,
  WakeUp = 3,
  CriticalHit = 4,
  MagicCriticalHit = 5,
  RowRight = 128,
  RowLeft = 129,
}
