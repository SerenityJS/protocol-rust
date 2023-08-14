use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::binary::prelude::*;
use crate::packets::types::*;
use napi::bindgen_prelude::*;

#[packet(0x1f)]
pub struct MobEquipmentPacket {
  pub runtime_id: VarLong,
  pub item: ItemStack,
  pub slot: u8,
  pub selected_slot: u8,
  // TODO: Bedrock Protocol has window id as i8?
  #[napi(ts_type = "WindowId")]
  pub window_id: u8,
}

#[napi]
pub enum WindowId {
  DropContents = -100,
  Beacon = -24,
  TradingOutput = -23,
  TradingUseInputs = -22,
  TradingInput1 = -21,
  TradingInput2 = -20,
  EnchantOutput = -17,
  EnchantMaterial = -16,
  EnchantInput = -15,
  AnvilOutput = -13,
  AnvilInput = -12,
  AnvilMaterial = -11,
  ContainerInput = -10,
  CraftingUseIngredient = -5,
  CraftingResult = -4,
  CraftingRemoveIngredient = -3,
  CraftingAddIngredient = -2,
  None = -1,
  Inventory = 0,
  First = 1,
  Last = 100,
  Offhand = 119,
  Armor = 120,
  Creative = 121,
  Hotbar = 122,
  FixedInventory = 123,
  Ui = 124,
}