use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0xc1)]
#[napi(constructor)]
pub struct RequestNetworkSettingsPacket {
  // Not to sure what this is but the packet length is 6 bytes usually
  // but the protocol is an i32 which is only 4 bytes. The packet id is 1 byte
  // so that leaves 1 byte left. It appears to be a bool that is always true.
  pub unknown: bool,
  pub protocol_version: i32,
}

#[napi]
impl RequestNetworkSettingsPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_u8(RequestNetworkSettingsPacket::id());
    bin.write_bool(self.unknown);
    bin.write_i32(self.protocol_version, Endianess::Big);

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_u8();
    let unknown = bin.read_bool();
    let protocol_version = bin.read_i32(Endianess::Big);

    RequestNetworkSettingsPacket {
      protocol_version,
      unknown,
    }
  }
}
