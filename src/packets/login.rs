use protocol_derive::packet;
use crate::packets::prelude::*;

#[packet(0x01)]
pub struct LoginPacket {
  pub protocol_version: i32,
  pub tokens: LoginTokens,
}

#[packet(manual_serialize)]
pub struct LoginTokens {
  pub identity: String,
  pub client: String,
}

// if this gets repitive add a derive attribute for prepending a
// sub struct with a length variable.
impl PacketChildSerialization for LoginTokens {
  fn serialize(&self) -> napi::Result<crate::binary::BinaryStream> {
    let mut bin = crate::binary::BinaryStream::new();

    bin.write_varint(self.identity.len() as i32 + self.client.len() as i32 + 2)?;
    bin.write_string(self.identity.to_owned())?;
    bin.write_string(self.client.to_owned())?;

    Ok(bin)
  }
  fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
    let _len = data.read_varint()?;
    let identity = data.read_string()?;
    let client = data.read_string()?;

    Ok(Self {
      identity,
      client,
    })
  }
}
