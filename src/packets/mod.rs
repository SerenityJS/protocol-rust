#![allow(dead_code, unused_variables)]

// Shared Packet Types
mod types;

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;
mod resource_packs_stack;
mod start_game;
mod network_settings;
mod request_network_settings;

mod update_player_game_type;

use protocol_derive::packet_enum;
use napi::bindgen_prelude::*;
use crate::binary::BinaryStream;

use prelude::*;

// TODO: I heavily dislike how packet ids have to be defined
// here and in the packet attribute macro. I'd like to find a
// way to only define them once ideally in the macro.

#[packet_enum]
pub enum Packet {
  Login = 0x01,
  PlayStatus = 0x02,
  ServerToClientHandshake = 0x03,
  ClientToServerHandshake = 0x04,
  Disconnect = 0x05,
  ResourcePacksInfo = 0x06,
  ResourcePacksStack = 0x07,
  StartGame = 0x0b,
  NetworkSettings = 0x8f,
  RequestNetworkSettings = 0xc1,
  UpdatePlayerGameType = 0x97,
}

// These must be implemented on packets so the enum serializer can work
pub mod prelude {
  pub trait PacketConversion: Sized {
    fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self>;
    fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object>;
  }
  
  pub trait PacketSerialization: Sized {
    fn serialize(&self) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Buffer>;
    fn deserialize(data: napi::bindgen_prelude::Buffer) -> napi::bindgen_prelude::Result<Self>;
  }
  
  // pub trait PacketChildConversion: Sized {
  //   fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self>;
  //   fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object>;
  // }
  
  pub trait PacketChildSerialization: Sized {
    fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream>;
    fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self>;
  }
}

// These are temporary methods;

#[napi]
pub fn get_packet_id(data: Buffer) -> i32 {
  // read varint directly from front of buffer for speed without binary stream
  let mut id = 0;
  let mut shift = 0;
  let mut i = 0;

  for byte in data.to_vec() {
    id |= ((byte & 0x7f) as i32) << shift;
    shift += 7;
    i += 1;

    if shift > 35 {
      panic!("VarInt is too big");
    }

    if (byte & 0x80) == 0 {
      break;
    }
  }

  id
}

// Frames an array of buffers into a single buffer
#[napi]
pub fn frame_packets(packets: Vec<Buffer>) -> napi::Result<Buffer> {
  let mut bin = BinaryStream::new();

  // bin.write_bytes(vec![0xfe]);

  for packet in packets {
    bin.write_varint(packet.len() as u32)?;
    bin.write(packet.into())?;
  }

  Ok(bin.data.into())
}

// Unframes a buffer of packets and returns an array of buffers
#[napi]
pub fn unframe_packets(data: Buffer) -> napi::Result<Vec<Buffer>> {
  let mut bin = BinaryStream::from(data.into());
  let mut packets = Vec::new();

  // let _id = bin.read_u8();

  while !bin.empty() {
    let len = bin.read_varint()? as usize;
    let packet = bin.read(len)?;
    packets.push(packet.into());
  }

  Ok(packets)
}

// Temporary generates motd_string for raknet
#[napi]
pub fn make_motd(
  motd: String,
  protocol_version: u32,
  version: String,
  current_players: u32,
  max_players: u32,
  server_id: String,
  world_name: String,
  gamemode: String,
  gamemode_id: u8,
  portv4: u16,
  portv6: u16,
) -> napi::Result<Buffer> {
  let str = format!(
    "MCPE;{};{};{};{};{};{};{};{};{};{};{};{};",
    motd,
    protocol_version,
    version,
    current_players,
    max_players,
    server_id,
    world_name,
    gamemode,
    gamemode_id,
    portv4,
    portv6,
    0
  );

  let mut bin = BinaryStream::new();
  bin.write_u16(str.as_bytes().len() as u16)?;
  bin.write(str.as_bytes().to_vec())?;

  Ok(bin.data.into())
}
