use protocol_derive::packet;
use napi::bindgen_prelude::*;
use napi::Result;
use crate::binary::rework::{BinaryStream2, I32};

#[packet(0xc1)]
#[napi(constructor)]
pub struct RequestNetworkSettingsPacket {
  pub protocol_version: I32,
}

// #[napi]
// impl RequestNetworkSettingsPacket {
//   #[napi]
//   pub fn serialize(&self) -> Buffer {
//     let mut bin = BinaryStream2::new();

//     bin.write_varint(RequestNetworkSettingsPacket::id());
//     bin.write_i32(self.protocol_version);

//     bin.data.into()
//   }

//   #[napi]
//   pub fn deserialize(data: Buffer) -> Self {
//     let mut bin = BinaryStream2::from(data.into());

//     let _id = bin.read_varint();
//     let protocol_version = bin.read_i32();

//     RequestNetworkSettingsPacket {
//       protocol_version,
//     }
//   }
// }

// Same implementation as above but using napi result and errors
#[napi]
impl RequestNetworkSettingsPacket {
  #[napi]
  pub fn serialize(&self) -> Result<Buffer> {
    let mut bin = BinaryStream2::new();

    bin.write_varint(RequestNetworkSettingsPacket::id())?;
    bin.write_i32(self.protocol_version)?;

    Ok(bin.data.into())
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Result<RequestNetworkSettingsPacket> {
    let mut bin = BinaryStream2::from(data.into());

    let _id = bin.read_varint()?;
    let protocol_version = bin.read_i32()?;

    Ok(RequestNetworkSettingsPacket {
      protocol_version,
    })
  }
}
