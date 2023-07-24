use protocol_derive::packet;

#[packet(0x03)]
pub struct ServerToClientHandshakePacket {
  pub token: String,
}
