use protocol_derive::packet;

#[packet(0xa3)]
pub struct FilterTextPacket {
  pub text: String,
  pub from_server: bool,
}
