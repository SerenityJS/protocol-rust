use protocol_derive::packet;
use crate::packets::prelude::*;
use crate::binary::U64;

#[packet(0x06)]
pub struct ResourcePacksInfoPacket {
  pub must_accept: bool,
  pub has_scripts: bool,
  pub force_server_packs: bool,
  pub behaviour_packs: Vec<BehaviourPackInfo>,
  pub resource_packs: Vec<ResourcePackInfo>,
}

#[packet]
pub struct BehaviourPackInfo {
  pub uuid: String,
  pub version: String,
  pub size: U64,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
}

#[packet]
pub struct ResourcePackInfo {
  pub uuid: String,
  pub version: String,
  pub size: U64,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
  pub rtx_enabled: bool,
}
