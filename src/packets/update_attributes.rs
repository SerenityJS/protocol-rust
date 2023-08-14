use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::{
  prelude::*,
  types::*
};

#[packet(0x1d)]
pub struct UpdateAttributesPacket {
  pub runtime_id: VarLong,
  #[VarInt] pub attributes: Vec<PlayerAttribute>,
  pub tick: VarLong
}
