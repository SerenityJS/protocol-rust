use protocol_derive::packet2;

#[packet2(0xc1)]
#[napi(object)]
pub struct RequestNetworkSettingsPacket {
  pub protocol_version: i32,
}
