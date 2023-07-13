use protocol_derive::{UseConstructorCloning, packet};
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0x06)]
#[napi(constructor)]
pub struct ResourcePacksInfoPacket {
  pub must_accept: bool,
  pub has_scripts: bool,
  pub force_server_packs: bool,

  pub behaviour_packs: Vec<BehaviourPackInfo>,
  pub resource_packs: Vec<ResourcePackInfo>,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct BehaviourPackInfo {
  pub uuid: String,
  pub version: String,
  pub size: BigInt,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct ResourcePackInfo {
  pub uuid: String,
  pub version: String,
  pub size: BigInt,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
  pub rtx_enabled: bool,
}

#[napi]
impl ResourcePacksInfoPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_u8(ResourcePacksInfoPacket::pid());
    bin.write_bool(self.must_accept);
    bin.write_bool(self.has_scripts);
    bin.write_bool(self.force_server_packs);
    bin.write_i16(self.behaviour_packs.len() as i16, Endianess::Little);

    for pack in &self.behaviour_packs {
      bin.write_string(pack.uuid.clone());
      bin.write_string(pack.version.clone());
      bin.write_u64(pack.size.get_u64().1, Endianess::Big);
      bin.write_string(pack.content_key.clone());
      bin.write_string(pack.sub_pack_name.clone());
      bin.write_string(pack.content_identity.clone());
      bin.write_bool(pack.has_scripts);
    }

    bin.write_i16(self.resource_packs.len() as i16, Endianess::Little);
    for pack in &self.resource_packs {
      bin.write_string(pack.uuid.clone());
      bin.write_string(pack.version.clone());
      bin.write_u64(pack.size.get_u64().1, Endianess::Big);
      bin.write_string(pack.content_key.clone());
      bin.write_string(pack.sub_pack_name.clone());
      bin.write_string(pack.content_identity.clone());
      bin.write_bool(pack.has_scripts);
      bin.write_bool(pack.rtx_enabled);
    }


    bin.data.into()
  }

  #[napi]
  pub fn deserialize(buf: Buffer) -> Self {
    let mut bin = BinaryStream::from(buf.into());

    let must_accept = bin.read_bool();
    let has_scripts = bin.read_bool();
    let force_server_packs = bin.read_bool();

    let behaviour_packs_len = bin.read_i16(Endianess::Little);
    let mut behaviour_packs = Vec::new();
    for _ in 0..behaviour_packs_len {
      let uuid = bin.read_string();
      let version = bin.read_string();
      let size = bin.read_u64(Endianess::Big);
      let content_key = bin.read_string();
      let sub_pack_name = bin.read_string();
      let content_identity = bin.read_string();
      let has_scripts = bin.read_bool();

      behaviour_packs.push(BehaviourPackInfo {
        uuid,
        version,
        size: BigInt::from(size),
        content_key,
        sub_pack_name,
        content_identity,
        has_scripts,
      });
    }

    let resource_packs_len = bin.read_i16(Endianess::Little);
    let mut resource_packs = Vec::new();
    for _ in 0..resource_packs_len {
      let uuid = bin.read_string();
      let version = bin.read_string();
      let size = bin.read_u64(Endianess::Big);
      let content_key = bin.read_string();
      let sub_pack_name = bin.read_string();
      let content_identity = bin.read_string();
      let has_scripts = bin.read_bool();
      let rtx_enabled = bin.read_bool();

      resource_packs.push(ResourcePackInfo {
        uuid,
        version,
        size: BigInt::from(size),
        content_key,
        sub_pack_name,
        content_identity,
        has_scripts,
        rtx_enabled,
      });
    }

    Self {
      must_accept,
      has_scripts,
      force_server_packs,
      behaviour_packs,
      resource_packs,
    }
  }
}
