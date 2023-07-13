#![allow(dead_code, unused_variables)]

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;

#[napi]
pub fn get_packet_id(data: Vec<u8>) -> u8 {
  data[0]
}
