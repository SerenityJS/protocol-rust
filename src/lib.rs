#![deny(clippy::all)]

mod binary;

use binary::{BinaryStream,Endianess};
// use protocol_derive::*;

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

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
  let mut bin = BinaryStream::new();

  bin.write_u8(ResourcePacksInfoPacket::packet_id());
  bin.write_bool(info.must_accept);
  bin.write_bool(info.has_scripts);
  bin.write_u8(info.behaviour_packs.len() as u8);
  for pack in info.behaviour_packs {
    bin.write_string(&pack.uuid);
    bin.write_string(&pack.version);
    bin.write_u64(pack.size.get_u64().1, Endianess::Big);
    bin.write_string(&pack.content_key);
    bin.write_string(&pack.sub_pack_name);
    bin.write_string(&pack.content_identity);
    bin.write_bool(pack.has_scripts);
  }

  bin.data
}

// Deserializes raknet packet buffer for resource packs info minecraft bedrock edition packet
#[napi]
pub fn deserialize_resource_packs_info(data: Vec<u8>) -> ResourcePacksInfoPacket {
  let mut bin = BinaryStream::from(data);

  let _id = bin.read_u8();
  let must_accept = bin.read_bool();
  let has_scripts = bin.read_bool();
  let behaviour_packs_len = bin.read_u8();
  let mut behaviour_packs = Vec::new();
  for _ in 0..behaviour_packs_len {
    let uuid = bin.read_string();
    let version = bin.read_string();
    let size = BigInt::from(bin.read_u64(Endianess::Big));
    let content_key = bin.read_string();
    let sub_pack_name = bin.read_string();
    let content_identity = bin.read_string();
    let has_scripts = bin.read_bool();

    behaviour_packs.push(
        BehaviourPackInfo {
          uuid,
          version,
          size,
          content_key,
          sub_pack_name,
          content_identity,
          has_scripts,
        }
    );
  }

  let resource_packs_len = bin.read_u8();
  let mut resource_packs = Vec::new();
  for _ in 0..resource_packs_len {
    let uuid = bin.read_string();
    let version = bin.read_string();
    let size = BigInt::from(bin.read_u64(Endianess::Big));
    let content_key = bin.read_string();
    let sub_pack_name = bin.read_string();
    let content_identity = bin.read_string();
    let has_scripts = bin.read_bool();
    let rtx_enabled = bin.read_bool();

    resource_packs.push(
        ResourcePackInfo {
          uuid,
          version,
          size,
          content_key,
          sub_pack_name,
          content_identity,
          has_scripts,
          rtx_enabled,
        }
    );
  }

  ResourcePacksInfoPacket {
    must_accept,
    has_scripts,
    behaviour_packs,
    resource_packs,
  }
}

#[napi(object)]
pub struct LoginToken {
  pub identity: String,
  pub client: String,
}

#[napi(object)]
pub struct LoginPacket {
  pub protocol_version: i32,
  pub tokens: LoginToken,
}

#[napi]
impl LoginPacket {
  pub fn packet_id() -> u8 {
    0x01
  }
}

#[napi]
pub fn serialize_login(login: LoginPacket) -> Vec<u8> {
  let mut bin = BinaryStream::new();

  bin.write_u8(LoginPacket::packet_id());
  bin.write_i32(login.protocol_version, Endianess::Big);
  bin.write_varint(login.tokens.identity.len() as i32 + login.tokens.client.len() as i32 + 2);
  bin.write_little_string(&login.tokens.identity);
  bin.write_little_string(&login.tokens.client);

  bin.data
}

// Deserializes raknet packet buffer for login minecraft bedrock edition packet
#[napi]
pub fn deserialize_login(data: Vec<u8>) -> LoginPacket {
  let mut bin = BinaryStream::from(data);

  let _id = bin.read_u8();
  let protocol_version = bin.read_i32(Endianess::Big);

  // There is a varint here which tells us the length of the next
  // 2 strings. I am not sure what it is needed for tbh.
  bin.read_varint();

  let identity = bin.read_little_string();
  let client = bin.read_little_string();

  LoginPacket {
    protocol_version,
    tokens: LoginToken {
      identity,
      client,
    },
  }
}
