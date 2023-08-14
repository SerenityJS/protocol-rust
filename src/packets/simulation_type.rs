use protocol_derive::packet;
use napi::bindgen_prelude::*;

#[packet(0xa8)]
pub struct SimulationTypePacket {
  #[napi(ts_type = "SimulationType")]
  pub status: u8,
}

#[napi]
pub enum SimulationType {
  Game,
  Editor,
  Test,
  Invalid,
}
