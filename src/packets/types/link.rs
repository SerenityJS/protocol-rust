use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct Link {
  pub ridden_entity_id: ZigZong,
  pub rider_entity_id: ZigZong,
  pub link_type: u8,
  pub immediate: bool,
  pub rider_initiated: bool,
}
