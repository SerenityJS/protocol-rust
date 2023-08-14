use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::nbt::NBT;

#[packet(0xa5)]
pub struct SyncEntityPropertyPacket {
  pub nbt: NBT,
}
