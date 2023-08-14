use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x21)]
pub struct InteractPacket {
  #[napi(ts_type = "InteractAction")]
  pub action: u8,
  pub entity_id: VarLong,
  // TODO: This is only valid when action is 4 or 3
  pub position: Vec3f,
}

#[napi]
pub enum InteractAction {
  LeaveVehicle = 3,
  MouseOverEntity = 4,
  NpcOpen = 5,
  OpenInventory = 6,
}