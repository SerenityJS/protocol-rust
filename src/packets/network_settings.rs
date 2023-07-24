use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::{LF32, LU16};

#[packet(0x8f)]
pub struct NetworkSettingsPacket {
  pub compression_threshold: LU16,
  #[napi(ts_type = "CompressionAlgorithm")]
  pub compression_algorithm: LU16,
  pub client_throttle: bool, 
  pub client_throttle_threshold: u8, 
  pub client_throttle_scalar: LF32, 
}

#[napi]
pub enum CompressionAlgorithm {
  Deflate,
  Snappy,
}
