use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

use super::add_player::*;

#[packet(0x0d)]
pub struct AddEntityPacket {
  pub unique_id: ZigZong,
  pub runtime_id: VarLong,
  pub entity_type: String,
  pub position: Vec3f,
  pub velocity: Vec3f,
  pub pitch: LF32,
  pub yaw: LF32,
  pub head_yaw: LF32,
  pub body_yaw: LF32,
  #[VarInt] pub attributes: Vec<EntityAttribute>,
  #[VarInt] pub metadata: Vec<MetadataDictionary>,
  pub properties: EntityProperties,
  #[VarInt] pub links: Vec<Link>,
}

#[packet]
pub struct EntityAttribute {
  pub name: String,
  pub min: LF32,
  pub max: LF32,
  pub value: LF32,
}
