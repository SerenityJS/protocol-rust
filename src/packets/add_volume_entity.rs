use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::prelude::*;
use crate::nbt::NBT;
use crate::packets::types::*;

#[packet(0xa6)]
pub struct AddVolumeEntityPacket {
  pub runtime_id: VarLong,
  pub nbt: NBT,
  pub encoding_identifier: String,
  pub instance_name: String,
  pub bounds: EntityBounds,
  pub dimension: ZigZag,
  pub engine_version: String,
}

#[packet]
pub struct EntityBounds {
  pub min: BlockCoordinates,
  pub max: BlockCoordinates,
}
