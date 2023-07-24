use protocol_derive::packet;

use super::{PacketChildConversion, PacketChildSerialization};

#[packet(0x01)]
#[napi(object)]
pub struct LoginPacket {
  pub protocol_version: i32,
  pub tokens: LoginTokens,
}

#[napi(object)]
pub struct LoginTokens {
  pub identity: String,
  pub client: String,
}

impl PacketChildConversion for LoginTokens {
  fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self> {
    let identity: String = data.get_named_property("identity")?;
    let client: String = data.get_named_property("client")?;

    Ok(Self {
      identity,
      client,
    })
  }

  fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object> {
    let mut object = env.create_object()?;

    object.set_named_property("identity", self.identity.to_owned())?;
    object.set_named_property("client", self.client.to_owned())?;

    Ok(object)
  }
}

impl PacketChildSerialization for LoginTokens {
  fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream> {
    let mut stream = crate::binary::BinaryStream::new();

    stream.write_string(self.identity.to_owned())?;
    stream.write_string(self.client.to_owned())?;

    Ok(stream)
  }

  fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
    let identity = data.read_string()?;
    let client = data.read_string()?;

    Ok(Self {
      identity,
      client,
    })
  }
}
