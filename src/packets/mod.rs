#![allow(dead_code, unused_variables)]

use napi::bindgen_prelude::Buffer;
use crate::binary::{BinaryStream, Endianess};

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;
mod request_network_settings;
mod network_settings;

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

// These are temporary methods below

// Frames an array of buffers into a single buffer
#[napi]
pub fn frame_packets(packets: Vec<Buffer>) -> Buffer {
  let mut bin = BinaryStream::new();

  // bin.write_bytes(vec![0xfe]);

  for packet in packets {
    bin.write_varint(packet.len() as i32);
    bin.write_bytes(packet.into());
  }

  bin.data.into()
}

// Unframes a buffer of packets and returns an array of buffers
#[napi]
pub fn unframe_packets(data: Buffer) -> Vec<Buffer> {
  let mut bin = BinaryStream::from(data.into());
  let mut packets = Vec::new();

  // let _id = bin.read_u8();

  while !bin.is_empty() {
    let len = bin.read_varint() as usize;
    let packet = bin.read_bytes(len);
    packets.push(packet.into());
  }

  packets
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
) -> Buffer {
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
  bin.write_u16(str.as_bytes().len() as u16, Endianess::Big);
  bin.write_bytes(str.as_bytes().to_vec());

  bin.data.into()
}
