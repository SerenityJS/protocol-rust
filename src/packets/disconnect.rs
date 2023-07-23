use protocol_derive::packet2;

#[packet2(0x05)]
#[napi(object)]
pub struct DisconnectPacket {
  pub hide_disconnect_screen: bool,
  pub message: String,
}
