mod request_network_settings;

use protocol_derive::enum_serializer;
use napi::bindgen_prelude::*;

#[napi]
#[enum_serializer]
pub enum Packet {
  RequestNetworkSettings = 0xc1,
}
