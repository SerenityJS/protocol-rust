use protocol_derive::{UseConstructorCloning, packet};
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0x01)]
#[napi(constructor)]
pub struct LoginPacket {
  pub protocol_version: i32,
  pub tokens: LoginToken,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct LoginToken {
  pub identity: String,
  pub client: String,
}

#[napi]
impl LoginPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(LoginPacket::id());
    bin.write_i32(self.protocol_version, Endianess::Big);
    bin.write_varint(self.tokens.identity.len() as i32 + self.tokens.client.len() as i32 + 2);
    bin.write_little_string(&self.tokens.identity);
    bin.write_little_string(&self.tokens.client);

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_varint();
    let protocol_version = bin.read_i32(Endianess::Big);

    // There is a varint here which tells us the length of the next
    // 2 strings. I am not sure what it is needed for tbh.
    bin.read_varuint();

    let identity = bin.read_little_string();
    let client = bin.read_little_string();

    LoginPacket {
      protocol_version,
      tokens: LoginToken {
        identity,
        client,
      },
    }
  }
}
