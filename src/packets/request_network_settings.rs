use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0xc1)]
#[napi(constructor)]
pub struct RequestNetworkSettingsPacket {
  pub protocol_version: i32,
}

#[napi]
impl RequestNetworkSettingsPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(RequestNetworkSettingsPacket::id());
    bin.write_i32(self.protocol_version, Endianess::Big);

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_varint();
    let protocol_version = bin.read_i32(Endianess::Big);

    RequestNetworkSettingsPacket {
      protocol_version,
    }
  }
}
