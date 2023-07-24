#![allow(dead_code, unused_variables)]

mod login;
mod disconnect;
mod request_network_settings;

use protocol_derive::enum_serializer;
use napi::bindgen_prelude::*;

use crate::binary::BinaryStream;

// TODO: I heavily dislike how packet ids have to be defined
// here and in the packet attribute macro. I'd like to find a
// way to only define them once ideally in the macro.

#[napi]
#[enum_serializer]
pub enum Packet {
  Login = 0x01,
  Disconnect = 0x05,
  RequestNetworkSettings = 0xc1,
}

// These must be implemented on packets so the enum serializer can work
pub trait PacketConversion: Sized {
  fn from_object(data: Object) -> Result<Self>;
  fn to_object(&self, env: Env) -> Result<Object>;
}

pub trait PacketSerialization: Sized {
  fn serialize(&self) -> Result<Buffer>;
  fn deserialize(data: Buffer) -> Result<Self>;
}

pub trait PacketChildConversion: Sized {
  fn from_object(data: Object) -> Result<Self>;
  fn to_object(&self, env: Env) -> Result<Object>;
}

pub trait PacketChildSerialization: Sized {
  fn serialize(&self) -> Result<BinaryStream>;
  fn deserialize(data: &mut BinaryStream) -> Result<Self>;
}
