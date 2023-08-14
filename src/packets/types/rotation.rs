use protocol_derive::packet;
use crate::binary::prelude::*;

// TODO: Implement ByteFloat???
#[packet]
pub struct Rotation {
  pub yaw: LF32,
  pub pitch: LF32,
  pub head_yaw: LF32,
}