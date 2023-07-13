use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::BinaryStream;

#[packet(0x03)]
#[napi(constructor)]
pub struct ServerToClientHandshakePacket {
  pub token: String,
}

#[napi]
impl ServerToClientHandshakePacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_u8(ServerToClientHandshakePacket::pid());
    bin.write_varuint(self.token.len() as u32);
    bin.write_string_without_length(self.token.clone());

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_u8();
    let len = bin.read_varuint();
    let token = bin.read_string_with_length(len);

    ServerToClientHandshakePacket {
      token,
    }
  }

}
