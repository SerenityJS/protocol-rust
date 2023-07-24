use protocol_derive::packet;

#[packet(0x04)]
pub struct ClientToServerHandshakePacket {}
