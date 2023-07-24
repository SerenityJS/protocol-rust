use protocol_derive::packet;

#[packet(0xc1)]
pub struct RequestNetworkSettingsPacket {
  pub protocol_version: i32,
}
