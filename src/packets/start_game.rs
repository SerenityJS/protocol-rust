use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};
use serde_json::Value;

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
  #[VarInt] pub gamerules: Vec<GameRule>,
  #[LI32] pub experiments: Vec<Experiment>,
  pub experiments_previously_used: bool,
  pub bonus_chest: bool,
  pub map_enabled: bool,
  #[napi(ts_type = "PermissionLevel")]
  pub permission_level: u8,
  pub server_chunk_tick_range: LI32,
  pub has_locked_behaviour_packs: bool,
  pub has_locked_resource_packs: bool,
  pub from_locked_world_template: bool,
  pub msa_gamertags_only: bool,
  pub from_world_template: bool,
  pub world_template_option_locked: bool,
  pub only_spawn_v1_villagers: bool,
  pub persona_disabled: bool,
  pub custom_skins_disabled: bool,
  pub emote_chat_muted: bool,
  pub game_version: String,
  pub limited_world_width: LI32,
  pub limited_world_length: LI32,
  pub new_nether: bool,
  pub edu_resource_uri: EducationSharedResourceURI,
  pub experimental_gameplay_override: bool,
  #[napi(ts_type = "ChatRestrictionLevel")]
  pub chat_restriction_level: u8,
  pub disable_player_interactions: bool,
  pub level_id: String,
  pub world_name: String,
  pub premium_world_template_id: String,
  pub is_trial: bool,
  #[napi(ts_type = "MovementAuthority")]
  pub movement_authority: ZigZag,
  pub rewind_history_size: ZigZag,
  pub server_authoritive_block_breaking: bool,
  pub current_tick: LI64,
  pub enchantment_seed: ZigZag,
  // TODO - nbt
  // pub block_properties
  #[VarInt] pub item_states: Vec<ItemState>,
  pub multiplayer_correlation_id: String,
  pub server_authoritative_inventory: bool,
  pub engine: String,
  // TODO - nbt
  // pub property_data
  pub block_pallette_checksum: LU64,
  // TODO - uuid
  // pub world_template_id: uuid
  pub client_side_generation: bool,
  pub block_network_ids_are_hashes: bool,
  pub server_controlled_sound: bool
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

#[packet(manual_serialize)]
pub struct GameRule {
  pub name: String,
  pub editable: bool,
  #[napi(ts_type = "GameRuleType")]
  pub field_type: VarInt,
  #[napi(ts_type = "boolean | number")]
  pub value: Value,
}

// Value depends off another field.. serialization gen has nothing in place
// to handle this and there is not much reason to as it doesn't occur often
impl PacketChildSerialization for GameRule {
  fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
    let name = data.read_string()?;
    let editable = data.read_bool()?;
    let field_type = data.read_varint()?;

    let value = match field_type {
      1 => Value::from(data.read_bool()?),
      2 => Value::from(data.read_zigzag()?),
      3 => Value::from(data.read_lf32()?),
      _ => return Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Expected 'field_type' to be '1', '2' or '3'! Recieved '{}'", field_type)
      ))
    };

    Ok(Self {
      name,
      editable,
      field_type,
      value,
    })
  }
  fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream> {
    let mut stream = crate::binary::BinaryStream::new();

    stream.write_string(self.name.to_owned())?;
    stream.write_bool(self.editable)?;
    stream.write_varint(self.field_type)?;
    match self.field_type {
      1 => match self.value.as_bool() {
        Some(value) => stream.write_bool(value)?,
        None => return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("'field_type' is '1' but 'value' is not boolean!")
        ))
      }
      2 => match self.value.as_i64() {
        Some(value) => stream.write_zigzag(value as i32)?,
        None => return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("'field_type' is '2' but 'value' is not a number!")
        ))
      },
      3 => match self.value.as_f64() {
        Some(value) => stream.write_lf32(value)?,
        None => return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("'field_type' is '3' but 'value' is not a float!")
        ))
      },
      _ => return Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("Expected 'field_type' to be '1', '2' or '3'! Recieved '{}'", self.field_type)
      ))
    }


    Ok(stream)
  }
}

#[napi]
pub enum GameRuleType {
  Bool = 1,
  Int = 2,
  Float = 3,
}

#[napi]
pub enum PermissionLevel {
  Visitor,
  Member,
  Operator,
  Custom,
}

#[packet]
pub struct EducationSharedResourceURI {
  pub button_name: String,
  pub link_uri: String,
}

#[napi]
pub enum ChatRestrictionLevel {
  None,
  Dropped,
  Disabled
}

#[napi]
pub enum MovementAuthority {
  Client,
  Server,
  ServerWithRewind
}

#[packet]
pub struct ItemState {
  pub name: String,
  pub runtime_id: LI16,
  pub component_based: bool,
}
