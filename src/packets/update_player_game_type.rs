// PMK744

use protocol_derive::packet;
use crate::binary::prelude::*;

#[packet(0x97)]
pub struct UpdatePlayerGameTypePacket {
  #[napi(ts_type = "GameMode")]
  pub gamemode: ZigZag,
  pub player_unique_id: ZigZong,
}
