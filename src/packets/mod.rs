#![allow(dead_code, unused_variables)]

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;
mod network_settings;
mod request_network_settings;

use protocol_derive::enum_serializer;
use napi::bindgen_prelude::*;

use prelude::*;

// TODO: I heavily dislike how packet ids have to be defined
// here and in the packet attribute macro. I'd like to find a
// way to only define them once ideally in the macro.

#[napi]
#[enum_serializer]
pub enum Packet {
  Login = 0x01,
  PlayStatus = 0x02,
  ServerToClientHandshake = 0x03,
  ClientToServerHandshake = 0x04,
  Disconnect = 0x05,
  ResourcePacksInfo = 0x06,
  NetworkSettings = 0x8f,
  RequestNetworkSettings = 0xc1,
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
  
  pub trait PacketChildConversion: Sized {
    fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self>;
    fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object>;
  }
  
  pub trait PacketChildSerialization: Sized {
    fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream>;
    fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self>;
  }
}
