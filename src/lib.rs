#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct ResourcePacksInfo {
  pub must_accept: bool,
  pub has_scripts: bool,
  pub behaviour_packs: Vec<String>,
  pub resource_packs: Vec<String>,
}

// 0x07

#[napi]
// Generates raknet packet buffer for resource packs info minecraft bedrock edition packet
pub fn serialize_resource_packs_info(info: ResourcePacksInfo) -> Vec<u8> {
  let mut buffer = Vec::new();
  buffer.push(0x07);
  buffer.push(if info.must_accept { 1 } else { 0 });
  buffer.push(if info.has_scripts { 1 } else { 0 });
  buffer.push(info.behaviour_packs.len() as u8);
  for pack in info.behaviour_packs {
    buffer.push(pack.len() as u8);
    buffer.extend(pack.as_bytes());
  }
  buffer.push(info.resource_packs.len() as u8);
  for pack in info.resource_packs {
    buffer.push(pack.len() as u8);
    buffer.extend(pack.as_bytes());
  }
  buffer
}


#[napi]
pub fn deserialize_resource_packs_info(data: Vec<u8>) -> ResourcePacksInfo {
  let mut data = data.into_iter();
  let _id = data.next().unwrap();
  let must_accept = data.next().unwrap() == 1;
  let has_scripts = data.next().unwrap() == 1;
  let behaviour_packs_len = data.next().unwrap();
  let mut behaviour_packs = Vec::new();
  for _ in 0..behaviour_packs_len {
    let pack_len = data.next().unwrap();
    let mut pack = Vec::new();
    for _ in 0..pack_len {
      pack.push(data.next().unwrap());
    }
    behaviour_packs.push(String::from_utf8(pack).unwrap());
  }
  let resource_packs_len = data.next().unwrap();
  let mut resource_packs = Vec::new();
  for _ in 0..resource_packs_len {
    let pack_len = data.next().unwrap();
    let mut pack = Vec::new();
    for _ in 0..pack_len {
      pack.push(data.next().unwrap());
    }
    resource_packs.push(String::from_utf8(pack).unwrap());
  }
  ResourcePacksInfo {
    must_accept,
    has_scripts,
    behaviour_packs,
    resource_packs,
  }
}