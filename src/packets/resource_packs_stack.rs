use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::binary::prelude::*;

#[packet(0x07)]
pub struct ResourcePacksStackPacket {
  pub must_accept: bool,
  #[VarInt] pub behaviour_packs: Vec<PackIdVersion>,
  #[VarInt] pub resource_packs: Vec<PackIdVersion>,
  pub game_version: String,
  #[LI32] pub experiments: Vec<Experiment>,
  pub experiments_previously_used: bool,
}

#[packet]
pub struct PackIdVersion {
  pub uuid: String,
  pub version: String,
  pub name: String,
}

#[packet]
pub struct Experiment {
  pub name: String,
  pub enabled: bool,
}
