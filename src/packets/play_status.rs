use protocol_derive::packet;
use napi::bindgen_prelude::*;

#[packet(0x02)]
pub struct PlayStatusPacket {
  #[napi(ts_type = "PlayStatus")]
  pub status: i32,
}

#[napi]
pub enum PlayStatus {
  LoginSuccess,
  FailedClient,
  FailedSpawn,
  PlayerSpawn,
  FailedInvalidTenant,
  FailedVanillaEdu,
  FailedEduVanilla,
  FailedServerFull,
  FailedEditorVanillaMismatch,
  FailedVanillaEditorMismatch,
}
