use protocol_derive::packet;
use crate::packets::prelude::*;

#[packet(0xaa)]
pub struct EduSharedResourceUriPacket {
  pub resource: SharedUri
}

#[packet]
pub struct SharedUri {
  pub button_text: String,
  pub link_uri: String,
}
