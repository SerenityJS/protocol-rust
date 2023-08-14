use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet]
pub struct MetadataDictionary {
  pub key: VarInt,
  pub key_type: VarInt,
  // TODO: pub value
}