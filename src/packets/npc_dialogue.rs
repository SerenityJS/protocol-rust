use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0xa9)]
pub struct NpcDialoguePacket {
  pub entity_id: LU64,
  #[napi(ts_type = "DialogueAction")]
  pub action: u8,
  pub dialogue: String,
  pub screen_name: String,
  pub npc_name: String,
  pub action_json: String,
}

#[napi]
pub enum DialogueAction {
  Open,
  Close,
}
