use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0xad)]
pub struct PhotoInfoRequestPacket {
  pub photo_id: ZigZong,
}
