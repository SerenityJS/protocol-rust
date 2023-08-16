use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x27)]
pub struct SetEntityDataPacket {
  pub runtime_id: VarLong,
  pub metadata: MetadataDictionary,
  pub properties: EntityProperties,
  pub tick: VarInt,
}
