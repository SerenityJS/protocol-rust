use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::BinaryStream;

#[packet(0x04)]
#[napi(constructor)]
pub struct ClientToServerHandshakePacket {}

#[napi]
impl ClientToServerHandshakePacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(ClientToServerHandshakePacket::id());

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_varint();

    ClientToServerHandshakePacket {}
  }
}
