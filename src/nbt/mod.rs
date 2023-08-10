// use napi::{Result, Error, Status::GenericFailure, bindgen_prelude::*};
use serde_json::{Map, Value};
use crate::packets::prelude::PacketChildSerialization;

pub type NBT = Map<String, Value>;

impl PacketChildSerialization for NBT {
  fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream> {
    let mut stream = crate::binary::BinaryStream::new();
    
    stream.write(vec![0x0A, 0x00, 0x00])?;

    Ok(stream)
  }
  fn deserialize(stream: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self> {
    // Temporary handle we are using empty compounds
    // Compound open tag.     0A
    stream.read_u8()?;
    // Compound name length.  00
    stream.read_u8()?;
    // Compound close tag     00
    stream.read_u8()?;

    Ok(Map::new())
  }
}
