#![allow(dead_code, unused_variables)]

use napi::bindgen_prelude::Buffer;
use crate::binary::BinaryStream;

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;
mod request_network_settings;
mod network_settings;

#[napi]
pub fn get_packet_id(data: Buffer) -> u8 {
  data[0]
}

// Takes an array of serialized packet buffers and frames them with their length
#[napi]
pub fn frame_packets(packets: Vec<Buffer>) -> Buffer {
  let mut bin = BinaryStream::new();

  bin.write_bytes(vec![0xfe]);

  for packet in packets {
    bin.write_varint(packet.len() as i32);
    bin.write_bytes(packet.into());
  }

  bin.data.into()
}
