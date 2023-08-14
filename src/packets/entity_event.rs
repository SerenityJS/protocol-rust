use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0x1b)]
pub struct EntityEventPacket {
  pub runtime_id: VarLong,
  #[napi(ts_type = "EntityEventType")]
  pub event_type: u8,
  pub data: ZigZag,
}

// TODO: Add the rest of the entity events
#[napi]
pub enum EntityEventType {
  Jump,
  HurtAnimation,
  DeathAnimation,
  ArmSwing,
  StopAttack,
  TameFail,
  TameSuccess,
  ShakeWet,
  UseItem,
  EatGrassAnimation,
  FishHookBubble,
  FishHookPosition,
  FishHookHook,
  FishHookTease,
  SquidInkCloud,
  ZombieVillagerCure,
}
