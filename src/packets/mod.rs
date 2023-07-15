#![allow(dead_code, unused_variables)]

use napi::bindgen_prelude::Buffer;

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;

#[napi]
pub fn get_packet_id(data: Buffer) -> u8 {
  data[0]
}
