use protocol_derive::packet;

#[packet(0xba)]
pub struct ToastRequestPacket {
  pub title: String,
  pub message: String,
}
