use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::nbt::NBT;

#[packet(0x7a)]
pub struct BiomeDefinitionListPacket {
  pub nbt: NBT,
}
