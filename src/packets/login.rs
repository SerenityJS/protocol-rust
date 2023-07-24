use protocol_derive::packet;
use crate::packets::prelude::*;

#[packet(0x01)]
#[napi(object)]
pub struct LoginPacket {
  pub protocol_version: i32,
  pub tokens: LoginTokens,
}

#[packet]
#[napi(object)]
pub struct LoginTokens {
  pub identity: String,
  pub client: String,
}
