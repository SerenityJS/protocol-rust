use protocol_derive::packet;
use crate::binary::prelude::*;
use napi::bindgen_prelude::*;

#[packet(0x9c)]
pub struct PacketViolationWarningPacket {
  #[napi(ts_type = "ViolationType")]
  pub violation_type: ZigZag,
  #[napi(ts_type = "ViolationSeverity")]
  pub severity: ZigZag,
  pub packety_id: ZigZag,
  pub reason: String,
}

#[napi]
pub enum ViolationType {
  Malformed,
}

#[napi]
pub enum ViolationSeverity {
  Warning,
  Severe,
  Fatal,
}
