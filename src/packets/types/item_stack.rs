use protocol_derive::packet;
use crate::binary::prelude::*;

// TODO: handle network_id fields
#[packet]
pub struct ItemStack {
  pub network_id: ZigZag,
}