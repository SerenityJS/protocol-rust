#![deny(clippy::all)]

use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct BehaviourPackInfo {
  pub uuid: String,
  pub version: String,
  pub size: BigInt,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
}

#[napi(object)]
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

#[napi(object)]
pub struct ResourcePacksInfoPacket {
  pub must_accept: bool,
  pub has_scripts: bool,
  pub behaviour_packs: Vec<BehaviourPackInfo>,
  pub resource_packs: Vec<ResourcePackInfo>,
}

#[napi]
impl ResourcePacksInfoPacket {
  fn packet_id() -> u8 {
    0x07
  }
}


// Generates raknet packet buffer for resource packs info minecraft bedrock edition packet
#[napi]
pub fn serialize_resource_packs_info(info: ResourcePacksInfoPacket) -> Vec<u8> {
  let mut buffer = Vec::new();
  buffer.push(ResourcePacksInfoPacket::packet_id());
  buffer.push(if info.must_accept { 1 } else { 0 });
  buffer.push(if info.has_scripts { 1 } else { 0 });
  buffer.push(info.behaviour_packs.len() as u8);
  for pack in info.behaviour_packs {
    buffer.push(pack.uuid.len() as u8);
    buffer.extend(pack.uuid.as_bytes());
    buffer.push(pack.version.len() as u8);
    buffer.extend(pack.version.as_bytes());
    buffer.extend(pack.size.get_u64().1.to_be_bytes().iter());
    buffer.push(pack.content_key.len() as u8);
    buffer.extend(pack.content_key.as_bytes());
    buffer.push(pack.sub_pack_name.len() as u8);
    buffer.extend(pack.sub_pack_name.as_bytes());
    buffer.push(pack.content_identity.len() as u8);
    buffer.extend(pack.content_identity.as_bytes());
    buffer.push(if pack.has_scripts { 1 } else { 0 });
  }
  buffer.push(info.resource_packs.len() as u8);
  for pack in info.resource_packs {
    buffer.push(pack.uuid.len() as u8);
    buffer.extend(pack.uuid.as_bytes());
    buffer.push(pack.version.len() as u8);
    buffer.extend(pack.version.as_bytes());
    buffer.extend(pack.size.get_u64().1.to_be_bytes().iter());
    buffer.push(pack.content_key.len() as u8);
    buffer.extend(pack.content_key.as_bytes());
    buffer.push(pack.sub_pack_name.len() as u8);
    buffer.extend(pack.sub_pack_name.as_bytes());
    buffer.push(pack.content_identity.len() as u8);
    buffer.extend(pack.content_identity.as_bytes());
    buffer.push(if pack.has_scripts { 1 } else { 0 });
    buffer.push(if pack.rtx_enabled { 1 } else { 0 });
  }
  buffer
}

// Deserializes raknet packet buffer for resource packs info minecraft bedrock edition packet
#[napi]
pub fn deserialize_resource_packs_info(data: Vec<u8>) -> ResourcePacksInfoPacket {
  let mut data = data.into_iter();
  let _id = data.next().unwrap();
  let must_accept = data.next().unwrap() == 1;
  let has_scripts = data.next().unwrap() == 1;
  let behaviour_packs_len = data.next().unwrap();
  let mut behaviour_packs = Vec::new();
  for _ in 0..behaviour_packs_len {
    let uuid_len = data.next().unwrap();
    let mut uuid = Vec::new();
    for _ in 0..uuid_len {
      uuid.push(data.next().unwrap());
    }
    let uuid = String::from_utf8(uuid).unwrap();
    let version_len = data.next().unwrap();
    let mut version = Vec::new();
    for _ in 0..version_len {
      version.push(data.next().unwrap());
    }
    let version = String::from_utf8(version).unwrap();
    let mut size = [0; 8];
    for i in 0..8 {
      size[i] = data.next().unwrap();
    }
    let size = BigInt::from(u64::from_be_bytes(size));
    let content_key_len = data.next().unwrap();
    let mut content_key = Vec::new();
    for _ in 0..content_key_len {
      content_key.push(data.next().unwrap());
    }
    let content_key = String::from_utf8(content_key).unwrap();
    let sub_pack_name_len = data.next().unwrap();
    let mut sub_pack_name = Vec::new();
    for _ in 0..sub_pack_name_len {
      sub_pack_name.push(data.next().unwrap());
    }
    let sub_pack_name = String::from_utf8(sub_pack_name).unwrap();
    let content_identity_len = data.next().unwrap();
    let mut content_identity = Vec::new();
    for _ in 0..content_identity_len {
      content_identity.push(data.next().unwrap());
    }
    let content_identity = String::from_utf8(content_identity).unwrap();
    let has_scripts = data.next().unwrap() == 1;
    behaviour_packs.push(BehaviourPackInfo {
      uuid,
      version,
      size,
      content_key,
      sub_pack_name,
      content_identity,
      has_scripts,
    });
  }
  let resource_packs_len = data.next().unwrap();
  let mut resource_packs = Vec::new();
  for _ in 0..resource_packs_len {
    let uuid_len = data.next().unwrap();
    let mut uuid = Vec::new();
    for _ in 0..uuid_len {
      uuid.push(data.next().unwrap());
    }
    let uuid = String::from_utf8(uuid).unwrap();
    let version_len = data.next().unwrap();
    let mut version = Vec::new();
    for _ in 0..version_len {
      version.push(data.next().unwrap());
    }
    let version = String::from_utf8(version).unwrap();
    let mut size = [0; 8];
    for i in 0..8 {
      size[i] = data.next().unwrap();
    }
    let size = BigInt::from(u64::from_be_bytes(size));
    let content_key_len = data.next().unwrap();
    let mut content_key = Vec::new();
    for _ in 0..content_key_len {
      content_key.push(data.next().unwrap());
    }
    let content_key = String::from_utf8(content_key).unwrap();
    let sub_pack_name_len = data.next().unwrap();
    let mut sub_pack_name = Vec::new();
    for _ in 0..sub_pack_name_len {
      sub_pack_name.push(data.next().unwrap());
    }
    let sub_pack_name = String::from_utf8(sub_pack_name).unwrap();
    let content_identity_len = data.next().unwrap();
    let mut content_identity = Vec::new();
    for _ in 0..content_identity_len {
      content_identity.push(data.next().unwrap());
    }
    let content_identity = String::from_utf8(content_identity).unwrap();
    let has_scripts = data.next().unwrap() == 1;
    let rtx_enabled = data.next().unwrap() == 1;
    resource_packs.push(ResourcePackInfo {
      uuid,
      version,
      size,
      content_key,
      sub_pack_name,
      content_identity,
      has_scripts,
      rtx_enabled,
    });
  }

  ResourcePacksInfoPacket {
    must_accept,
    has_scripts,
    behaviour_packs,
    resource_packs,
  }
}
