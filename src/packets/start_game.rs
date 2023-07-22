use protocol_derive::{packet, UseConstructorCloning};
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Vec3f, Vec2f};

use super::resource_pack_stack::Experiment;

#[packet(0x0b)]
#[napi(constructor)]
pub struct StartGamePacket {
  // zigzag64
  pub entity_id: i64,
  // varint64 (varlong)
  pub runtime_entity_id: i64,
  // zigzag32
  pub player_gamemode: GameMode,
  pub player_position: Vec3f,
  pub player_rotation: Vec2f,
  // little endian u64
  pub seed: BigInt,
  // little endian
  pub biome_type: i16,
  pub biome_name: String,
  // zigzag32
  pub dimension: Dimension,
  // zigzag32
  pub generator: i32,
  // zigzag32
  pub world_gamemode: GameMode,
  // zigzag32
  pub difficulty: i32,
  pub spawn_position: BlockCoordinates,
  pub achievements_disabled: bool,
  pub editor_world: bool,
  pub created_in_editor: bool,
  pub exported_from_editor: bool,
  // zigzag32
  pub day_cycle_stop_time: i32,
  // zigzag32
  pub edu_offer: i32,
  pub edu_features_enabled: bool,
  pub edu_product_uuid: String,
  // Little endian f32
  pub rain_level: f64,
  // Little endian f32
  pub lightning_level: f64,
  pub has_confirmed_platform_locked_content: bool,
  pub is_multiplayer: bool,
  pub broadcast_to_lan: bool,
  // varint
  pub xbox_live_broadcast_mode: i32,
  // varint
  pub platform_broadcast_mode: i32,
  pub enable_commands: bool,
  pub is_texturepacks_required: bool,
  // varint
  pub game_rules: Vec<GameRule>,
  // li32
  pub experiments: Vec<Experiment>,
  pub experiments_previously_used: bool,
  pub bonus_chest: bool,
  pub map_enabled: bool,
  pub permission_level: PermissionLevel,
  // little endian
  pub server_chunk_tick_range: i32,
  pub has_locked_behavior_pack: bool,
  pub is_from_locked_world_template: bool,
  pub msa_gamertags_only: bool,
  pub is_from_world_template: bool,
  pub only_spawn_v1_villagers: bool,
  pub persona_disabled: bool,
  pub custom_skins_disabled: bool,
  pub emote_chat_muted: bool,
  pub game_version: String,
  // little endian
  pub limited_world_width: i32,
  // little endian
  pub limited_world_length: i32,
  pub is_new_nether: bool,
  pub edu_resource_uri: EducationSharedResourceURI,
  pub experimental_gameplay_override: bool,
  pub chat_restriction_level: ChatRestrictionLevel,
  pub disabled_player_interactions: bool,
  pub level_id: String,
  pub world_name: String,
  pub premium_world_template_id: String,
  pub is_trial: bool,
  // zigzag 32
  pub movement_authority: MovementAuthority,
  // zigzag 32
  pub rewind_history_size: i32,
  pub server_authoritative_block_breaking: bool,
  // little endian
  pub current_tick: i64,
  // zigzag 32
  pub enchantment_seed: i32,
  // varint
  pub block_properties: Vec<BlockProperty>,
  // varint
  pub item_states: Vec<ItemState>,
  pub multiplayer_correlation_id: String,
  pub server_authoritative_inventory: bool,
  pub server_engine: String,
  // Needs to be nbt
  pub property_data: String,
  // little endian u64
  pub block_pallette_checksum: BigInt,
  // uuid
  pub world_template_id: String,
  pub client_side_generation: bool,
  pub block_network_ids_are_hashes: bool,
  pub server_controlled_sounds: bool
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

// Bruh mojang what is this shit...
#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct BlockCoordinates {
  // zigzag32
  pub x: i32,
  // varint
  pub y: u32,
  // zigzag32
  pub z: i32,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct GameRule {
  pub name: String,
  pub editable: bool,
  // Todo type and value
}

#[napi]
pub enum PermissionLevel {
  Visitor,
  Member,
  Operator,
  Custom,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct EducationSharedResourceURI {
  pub button_name: String,
  pub link_uri: String
}

#[napi]
pub enum ChatRestrictionLevel {
  None,
  Dropped,
  Disabled,
}

#[napi]
pub enum MovementAuthority {
  Client,
  Server,
  ServerWithRewind
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct BlockProperty {
  pub name: String,
  // this needs to actually be nbt
  pub state: String,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct ItemState {
  pub name: String,
  // little endian
  pub runtime_id: i16,
  pub component_based: bool,
}

// #[napi]
// impl StartGamePacket {
//   #[napi]
//   pub fn serialize(&self) -> Buffer {
//     let mut bin = BinaryStream::new();

//     bin.write_varint(StartGamePacket::id());
//     bin.write_varuint(self.token.len() as u32);
//     bin.write_string_without_length(self.token.clone());

//     bin.data.into()
//   }

//   #[napi]
//   pub fn deserialize(data: Buffer) -> Self {
//     let mut bin = BinaryStream::from(data.into());

//     let _id = bin.read_u8();
//     let len = bin.read_varuint();
//     let token = bin.read_string_with_length(len);

//     StartGamePacket {
//       token,
//     }
//   }

// }
