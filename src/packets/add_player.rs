use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x0c)]
pub struct AddPlayerPacket {
  pub uuid: UUID,
  pub username: String,
  pub runtime_id: VarLong,
  pub platform_chat_id: String,
  pub position: Vec3f,
  pub velocity: Vec3f,
  pub pitch: LF32,
  pub yaw: LF32,
  pub head_yaw: LF32,
  pub item: Item,
  #[napi(ts_type = "GameMode")]
  pub gamemode: ZigZag,
  #[VarInt] pub metadata: Vec<MetadataDictionary>,
  pub properties: EntityProperties,
  pub unique_id: LI16,
  #[napi(ts_type = "PermissionLevel")]
  pub permission_level: u8,
  #[napi(ts_type = "CommandPermissionLevel")]
  pub command_permission_level: u8,
  pub abilities: AbilityLayers,
  #[VarInt] pub links: Vec<Link>,
  pub device_id: String,
  // TODO: Added device list...
  pub device_os: LI32,
}

// TODO: handle network_id fields
#[packet]
pub struct Item {
  pub network_id: ZigZag,
}

#[packet]
pub struct MetadataDictionary {
  pub key: VarInt,
  pub key_type: VarInt,
  // TODO: pub value
}

#[packet]
pub struct EntityProperties {
  #[VarInt] pub ints: Vec<EntityInts>,
  #[VarInt] pub floats: Vec<EntityFloats>,
}

#[packet]
pub struct EntityInts {
  pub index: VarInt,
  pub value: ZigZag,
}

#[packet]
pub struct EntityFloats {
  pub index: VarInt,
  pub value: LF32,
}

#[napi]
pub enum CommandPermissionLevel {
  Normal,
  Operator,
  Automation,
  Host,
  Owner,
  Internal,
}

#[packet]
pub struct AbilityLayers {
  pub ability_type: LU16,
  // TODO: pub allowed: AbilitySet,
  // TODO: pub enabled: AbilitySet,
  pub fly_speed: LF32,
  pub walk_speed: LF32,
}

#[packet]
pub struct Link {
  pub ridden_entity_id: ZigZong,
  pub rider_entity_id: ZigZong,
  pub link_type: u8,
  pub immediate: bool,
  pub rider_initiated: bool,
}