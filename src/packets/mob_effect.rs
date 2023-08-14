use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x1c)]
pub struct MobEffectPacket {
  pub runtime_id: VarLong,
  pub action: u8,
  pub effect_id: ZigZag,
  pub amplifier: ZigZag,
  pub particles: bool,
  pub duration: ZigZag,
}
