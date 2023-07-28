use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::prelude::*;
use crate::packets::{prelude::*, types::vec::*};

#[packet(0x0b)]
pub struct StartGamePacket {
  pub entity_id: ZigZong,
  pub runtime_entity_id: VarLong,
  #[napi(ts_type = "GameMode")]
  pub player_gamemode: ZigZag,
  pub player_position: Vec3f,
  pub player_rotation: Vec2f,
  pub seed: LU64,
  pub biome_type: LI16,
  pub biome_name: String,
  #[napi(ts_type = "Dimension")]
  pub dimension: ZigZag,
  pub generator: ZigZag,
  #[napi(ts_type = "GameMode")]
  pub world_gamemode: ZigZag,
  pub difficulty: ZigZag,
  pub spawn_position: BlockCoordinates,
  pub achievements_disabled: bool,
  pub editor_world: bool,
  pub created_in_editor: bool,
  pub day_cycle_stop_time: ZigZag,
  pub edu_offer: ZigZag,
  pub edu_features_enabled: bool,
  pub edu_product_id: String,
  pub rain_level: LF32,
  pub lightning_level: LF32,
  pub confirmed_platform_locked_content: bool,
  pub multiplayer_game: bool,
  pub broadcast_to_lan: bool,
  pub xbl_broadcast_mode: VarInt,
  pub platform_broadcast_mode: VarInt,
  pub enable_commands: bool,
  pub texture_packs_required: bool,
}

#[napi]
pub enum GameMode {
  Survival,
  Creative,
  Adventure,
  SurvivalSpectator,
  CreativeSpectator,
  Fallback,
  Spectator,
}

#[napi]
pub enum Dimension {
  Overworld,
  Nether,
  End,
}

// Certified Mojang Moment
// This is only used here 🤦
#[packet]
pub struct BlockCoordinates {
  pub x: ZigZag,
  pub y: VarInt,
  pub z: ZigZag,
}

#[packet]
pub struct GameRule {
  pub name: String,
  pub editable: bool,
  #[napi(ts_type = "GameRuleType")]
  pub field_type: VarInt,
  // pub value: Value,
}

#[napi]
pub enum GameRuleType {
  Bool = 1,
  Int = 2,
  Float = 3,
}
