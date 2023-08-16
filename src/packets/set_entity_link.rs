use protocol_derive::packet;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x29)]
pub struct SetEntityLinkPacket {
  pub link: Link,
}
