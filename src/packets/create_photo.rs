use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0xab)]
pub struct CreatePhotoPacket {
  pub entity_unique_id: LI64,
  pub photo_name: String,
  pub item_name: String,
}
