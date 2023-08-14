// TODO: Complete packet... Bedrock Protocol and Gophertunnel have conflicting types

use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::*;

#[packet(0x91)]
pub struct CreativeContentPacket {
  #[VarInt] pub items: Vec<CreativeItem>
}

#[packet]
pub struct CreativeItem {
  pub entry_id: VarInt,
  pub item: ItemLegacy
}

// TODO: Handle network id
#[packet]
pub struct ItemLegacy {
  pub network_id: ZigZag,
  pub count: LU16,
  pub metadata: VarInt,
  pub runtime_id: ZigZag,
  pub extra: ItemLegacyExtras
}
  
// TODO: Handle extra data
#[packet]
pub struct ItemLegacyExtras {
  // pub length_type: VarInt,
  pub has_nbt: LU16,
  // TODO: NBT if true
  #[LI32] pub can_place_on: Vec<LittleString>,
  #[LI32] pub can_destroy: Vec<LittleString>,
}