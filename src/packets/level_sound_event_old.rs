use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x18)]
pub struct LevelSoundEventOldPacket {
  pub sound_id: u8,
  pub position: Vec3f,
  pub block_id: ZigZag,
  pub entity_type: ZigZag,
  pub is_baby_mob: bool,
  pub is_global: bool,
}
