use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x98)]
pub struct EmoteListPacket {
  pub player_unique_id: ZigZong,
  #[VarInt] pub emote_ids: Vec<UUID>,
}
