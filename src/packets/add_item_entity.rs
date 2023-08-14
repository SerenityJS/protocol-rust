use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

use super::add_player::*;

#[packet(0x0f)]
pub struct AddItemEntityPacket {
  pub unique_id: ZigZong,
  pub runtime_id: VarLong,
  pub item: Item,
  pub position: Vec3f,
  pub velocity: Vec3f,
  #[VarInt] pub metadata: Vec<MetadataDictionary>,
  pub is_from_fishing: bool,
}
