use protocol_derive::packet;
use crate::binary::prelude::*;
use crate::packets::prelude::*;

#[packet(0x01)]
pub struct LoginPacket {
  pub protocol_version: i32,
  #[VarInt] pub tokens: LoginTokens,
}

#[packet]
pub struct LoginTokens {
  pub identity: LittleString,
  pub client: LittleString,
}
