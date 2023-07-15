use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::BinaryStream;

#[packet(0x05)]
#[napi(constructor)]
pub struct DisconnectPacket {
  pub hide_disconnect_screen: bool,
  pub message: String,
}

#[napi]
impl DisconnectPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_u8(DisconnectPacket::id());
    bin.write_bool(self.hide_disconnect_screen);
    bin.write_varuint(self.message.len() as u32);
    bin.write_string_without_length(self.message.clone());

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_u8();
    let hide_disconnect_screen = bin.read_bool();
    let len = bin.read_varuint();
    let message = bin.read_string_with_length(len);

    DisconnectPacket {
      hide_disconnect_screen,
      message,
    }
  }

}