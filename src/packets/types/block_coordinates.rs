use protocol_derive::packet;
use crate::binary::prelude::*;

// Certified Mojang Moment
// This is only used here 🤦
#[packet]
pub struct BlockCoordinates {
  pub x: ZigZag,
  pub y: VarInt,
  pub z: ZigZag,
}