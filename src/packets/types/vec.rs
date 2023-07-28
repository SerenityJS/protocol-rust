use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct Vec3f {
  pub x: LF32,
  pub y: LF32,
  pub z: LF32,
}

#[packet] 
pub struct Vec2f {
  pub x: LF32,
  pub z: LF32,
}
