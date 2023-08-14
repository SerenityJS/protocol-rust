use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0x08)]
pub struct ResourcePackClientResponsePacket {
  #[napi(ts_type = "ResourceResponseStatus")]
  pub status: u8,
  #[LI16] pub packs: Vec<String>
}

#[napi]
pub enum ResourceResponseStatus {
  None,
  Refused,
  SendPacks,
  HaveAllPacks,
  Completed,
}
