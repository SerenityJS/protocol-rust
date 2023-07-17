use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0x8f)]
#[napi(constructor)]
pub struct NetworkSettingsPacket {
  pub compression_threshold: u16,
  pub compression_algorithm: CompressionAlgorithm,
  pub client_throttle: bool,
  pub client_throttle_threshold: u8,
  // Napi only supports f64 this is a f32 in the protocol
  pub client_throttle_scalar: f64,
}

#[napi]
pub enum CompressionAlgorithm {
  Deflate,
  Snappy,
  None,
}

#[napi]
impl NetworkSettingsPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(NetworkSettingsPacket::id() as i32);
    bin.write_u16(self.compression_threshold, Endianess::Little);
    bin.write_u16(self.compression_algorithm as u16, Endianess::Little);
    bin.write_bool(self.client_throttle);
    bin.write_u8(self.client_throttle_threshold);
    bin.write_f32(self.client_throttle_scalar as f32, Endianess::Little);
    

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_varint();
    let compression_threshold = bin.read_u16(Endianess::Little);
    let compression_algorithm = bin.read_u16(Endianess::Little);
    let client_throttle = bin.read_bool();
    let client_throttle_threshold = bin.read_u8();
    let client_throttle_scalar = bin.read_f32(Endianess::Little) as f64;

    NetworkSettingsPacket {
      compression_threshold,
      compression_algorithm: match compression_algorithm {
        0 => CompressionAlgorithm::Deflate,
        1 => CompressionAlgorithm::Snappy,
        _ => CompressionAlgorithm::None,
      },
      client_throttle,
      client_throttle_threshold,
      client_throttle_scalar,
    }
  }
}
