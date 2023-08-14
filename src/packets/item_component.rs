use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::binary::prelude::*;
use crate::nbt::NBT;

#[packet(0xa2)]
pub struct ItemComponentPacket {
  #[VarInt] pub entries: Vec<ItemComponentList>
}

#[packet]
pub struct ItemComponentList {
  pub name: String,
  pub nbt: NBT,
}
