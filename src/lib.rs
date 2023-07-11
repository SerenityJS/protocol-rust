#![deny(clippy::all)]

// use protocol_derive::*;

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

// #[napi(constructor)]
// #[derive(Packet)]
// #[packet(id = 0x07)]
// struct TestPacket {
//   pub test: String,
//   pub bruh: u8,
//   pub foo: i32,
// }

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

fn write_bool(buffer: &mut Vec<u8>, value: bool) {
  buffer.push(if value { 1 } else { 0 });
}

fn write_string(buffer: &mut Vec<u8>, value: String) {
  buffer.push(value.len() as u8);
  buffer.extend(value.as_bytes());
}

fn write_bigint(buffer: &mut Vec<u8>, value: BigInt) {
  buffer.extend(value.get_u64().1.to_be_bytes().iter());
}

fn write_u8(buffer: &mut Vec<u8>, value: u8) {
  buffer.push(value);
}

fn read_bool(data: &mut impl Iterator<Item = u8>) -> bool {
  data.next().unwrap() == 1
}

fn read_string(data: &mut impl Iterator<Item = u8>) -> String {
  let len = data.next().unwrap() as usize;
  let mut string = String::new();
  for _ in 0..len {
    string.push(data.next().unwrap() as char);
  }
  string
}

fn read_bigint(data: &mut impl Iterator<Item = u8>) -> BigInt {
  let mut bytes = [0; 8];
  for i in 0..8 {
    bytes[i] = data.next().unwrap();
  }
  BigInt::from(u64::from_be_bytes(bytes))
}

fn read_u8(data: &mut impl Iterator<Item = u8>) -> u8 {
  data.next().unwrap()
}


// Generates raknet packet buffer for resource packs info minecraft bedrock edition packet
#[napi]
pub fn serialize_resource_packs_info(info: ResourcePacksInfoPacket) -> Vec<u8> {
  let mut buffer = Vec::new();
  write_u8(&mut buffer, ResourcePacksInfoPacket::packet_id());
  write_bool(&mut buffer, info.must_accept);
  write_bool(&mut buffer, info.has_scripts);
  write_u8(&mut buffer, info.behaviour_packs.len() as u8);
  for pack in info.behaviour_packs {
    write_string(&mut buffer, pack.uuid);
    write_string(&mut buffer, pack.version);
    write_bigint(&mut buffer, pack.size);
    write_string(&mut buffer, pack.content_key);
    write_string(&mut buffer, pack.sub_pack_name);
    write_string(&mut buffer, pack.content_identity);
    write_bool(&mut buffer, pack.has_scripts);
  }
  write_u8(&mut buffer, info.resource_packs.len() as u8);
  for pack in info.resource_packs {
    write_string(&mut buffer, pack.uuid);
    write_string(&mut buffer, pack.version);
    write_bigint(&mut buffer, pack.size);
    write_string(&mut buffer, pack.content_key);
    write_string(&mut buffer, pack.sub_pack_name);
    write_string(&mut buffer, pack.content_identity);
    write_bool(&mut buffer, pack.has_scripts);
    write_bool(&mut buffer, pack.rtx_enabled);
  }

  buffer
}

// Deserializes raknet packet buffer for resource packs info minecraft bedrock edition packet
#[napi]
pub fn deserialize_resource_packs_info(data: Vec<u8>) -> ResourcePacksInfoPacket {
  let mut data = data.into_iter();
  let _id = read_u8(&mut data);
  let must_accept = read_bool(&mut data);
  let has_scripts = read_bool(&mut data);
  let behaviour_packs_len = read_u8(&mut data);
  let mut behaviour_packs = Vec::new();
  for _ in 0..behaviour_packs_len {
    let uuid = read_string(&mut data);
    let version = read_string(&mut data);
    let size = read_bigint(&mut data);
    let content_key = read_string(&mut data);
    let sub_pack_name = read_string(&mut data);
    let content_identity = read_string(&mut data);
    let has_scripts = read_bool(&mut data);

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
  let resource_packs_len = read_u8(&mut data);
  let mut resource_packs = Vec::new();
  for _ in 0..resource_packs_len {
    let uuid = read_string(&mut data);
    let version = read_string(&mut data);
    let size = read_bigint(&mut data);
    let content_key = read_string(&mut data);
    let sub_pack_name = read_string(&mut data);
    let content_identity = read_string(&mut data);
    let has_scripts = read_bool(&mut data);
    let rtx_enabled = read_bool(&mut data);

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
