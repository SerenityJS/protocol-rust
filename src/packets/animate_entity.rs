use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x9e)]
pub struct AnimateEntityPacket {
  pub animation: String,
  pub next_state: String,
  pub stop_condition: String,
  pub stop_condition_verion: LI32,
  pub controller: String,
  pub blend_out_time: LF32,
  #[VarInt] pub entity_runtime_ids: Vec<VarLong>
}
