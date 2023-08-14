use protocol_derive::packet;
use crate::binary::prelude::*;
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
  // TODO: pub item: VarInt,
  #[napi(ts_type = "GameMode")]
  pub gamemode: ZigZag,
  // TODO: pub metadata: MetadataDictionary,
  // TODO: pub properties: EntityProperties,
  pub unique_id: LI16,
  // TODO: pub permission_level: PermissionLevel
  // TODO: pub command_permission_level: CommandPermissionLevel,
  // TODO: pub abilities: Abilities,
  // TODO: pub links: Links
  pub device_id: String,
  // TODO: pub device_os: DeviceOS,
}
