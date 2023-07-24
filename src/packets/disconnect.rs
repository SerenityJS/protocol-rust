use protocol_derive::packet;

#[packet(0x05)]
pub struct DisconnectPacket {
  pub hide_disconnect_screen: bool,
  pub message: String,
}
