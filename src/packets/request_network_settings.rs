use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::*;

#[packet(0xc1)]
#[napi(object)]
pub struct RequestNetworkSettingsPacket {
  pub protocol_version: I32,
}

impl RequestNetworkSettingsPacket {
  pub fn from_object(data: Object) -> Result<Self> {
    let protocol_version = data.get_named_property("protocol_version")?;

    Ok(Self {
      protocol_version,
    })
  }
  pub fn to_object(&self, env: Env) -> Result<Object> {
    let mut object = env.create_object()?;

    object.set_named_property("protocol_version", self.protocol_version)?;

    Ok(object)
  }
}

impl RequestNetworkSettingsPacket {
  pub fn serialize(&self) -> Result<Buffer> {
    let mut stream = BinaryStream::new();

    stream.write_varint(RequestNetworkSettingsPacket::ID)?;
    stream.write_i32(self.protocol_version)?;

    Ok(stream.data.into())
  }

  pub fn deserialize(data: Buffer) -> Result<Self> {
    let mut stream = BinaryStream::from(data.into());

    let _id = stream.read_varint()?;
    let protocol_version = stream.read_i32()?;

    Ok(Self {
      protocol_version,
    })
  }
}
