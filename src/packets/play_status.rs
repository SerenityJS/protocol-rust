use protocol_derive::packet;
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0x02)]
#[napi(constructor)]
pub struct PlayStatusPacket {
  pub status: PlayStatus,
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

#[napi]
impl PlayStatusPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(PlayStatusPacket::id());
    bin.write_i32(self.status as i32, Endianess::Big);

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(data: Buffer) -> Self {
    let mut bin = BinaryStream::from(data.into());

    let _id = bin.read_varint();
    let status = bin.read_i32(Endianess::Big);

    PlayStatusPacket {
      status: match status {
        0 => PlayStatus::LoginSuccess,
        1 => PlayStatus::FailedClient,
        2 => PlayStatus::FailedSpawn,
        3 => PlayStatus::PlayerSpawn,
        4 => PlayStatus::FailedInvalidTenant,
        5 => PlayStatus::FailedVanillaEdu,
        6 => PlayStatus::FailedEduVanilla,
        7 => PlayStatus::FailedServerFull,
        8 => PlayStatus::FailedEditorVanillaMismatch,
        9 => PlayStatus::FailedVanillaEditorMismatch,
        _ => panic!("Invalid play status"),
      },
    }
  }
}
