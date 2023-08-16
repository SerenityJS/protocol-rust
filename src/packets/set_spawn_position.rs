use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x2b)]
pub struct SetSpawnPositionPacket {
  pub runtime_id: VarLong,
  pub metadata: MetadataDictionary,
  pub properties: EntityProperties,
  pub tick: VarInt,
}
