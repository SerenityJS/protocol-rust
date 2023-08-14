use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::binary::prelude::*;
use crate::packets::types::*;

#[packet(0x20)]
pub struct MobArmorEquipmentPacket {
  pub runtime_id: VarLong,
  pub helmet: ItemStack,
  pub chestplate: ItemStack,
  pub leggings: ItemStack,
  pub boots: ItemStack,
}
