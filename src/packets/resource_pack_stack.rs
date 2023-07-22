use protocol_derive::{UseConstructorCloning, packet};
use napi::bindgen_prelude::*;
use crate::binary::{BinaryStream, Endianess};

#[packet(0x07)]
#[napi(constructor)]
pub struct ResourcePackStackPacket {
  pub must_accept: bool,
  pub behaviour_packs: Vec<BehaviourPackIdVersion>,
  pub resource_packs: Vec<ResourcePackIdVersion>,
  pub game_version: String,
  pub experiments: Vec<Experiment>,
  pub experiments_previously_used: bool,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct BehaviourPackIdVersion {
  pub uuid: String,
  pub version: String,
  pub name: String,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct ResourcePackIdVersion {
  pub uuid: String,
  pub version: String,
  pub name: String,
}

#[napi(constructor)]
#[derive(Clone, UseConstructorCloning)]
pub struct Experiment {
  pub name: String,
  pub enabled: bool,
}

// TODO: let resourceinfopack and behaviourinfopack have their own serialize and deserialize
// TODO: resource packs & behaviour packs info is invalid uuids are handled differently and other fields
#[napi]
impl ResourcePackStackPacket {
  #[napi]
  pub fn serialize(&self) -> Buffer {
    let mut bin = BinaryStream::new();

    bin.write_varint(ResourcePackStackPacket::id());
    bin.write_bool(self.must_accept);

    bin.write_varuint(self.behaviour_packs.len() as u32);
    for pack in &self.behaviour_packs {
      bin.write_string(pack.uuid.clone());
      bin.write_string(pack.version.clone());
      bin.write_string(pack.name.clone());
    }

    bin.write_varuint(self.resource_packs.len() as u32);
    for pack in &self.resource_packs {
      bin.write_string(pack.uuid.clone());
      bin.write_string(pack.version.clone());
      bin.write_string(pack.name.clone());
    }

    bin.write_string(self.game_version.clone());

    bin.write_i32(self.experiments.len() as i32, Endianess::Little);
    for experiment in &self.experiments {
      bin.write_string(experiment.name.clone());
      bin.write_bool(experiment.enabled);
    }

    bin.write_bool(self.experiments_previously_used);

    bin.data.into()
  }

  #[napi]
  pub fn deserialize(buf: Buffer) -> Self {
    let mut bin = BinaryStream::from(buf.into());

    let _id = bin.read_varint();

    let must_accept = bin.read_bool();
    let behaviour_packs_len = bin.read_varuint() as usize;
    let mut behaviour_packs = Vec::with_capacity(behaviour_packs_len);
    for _ in 0..behaviour_packs_len {
      let uuid = bin.read_string();
      let version = bin.read_string();
      let name = bin.read_string();
      behaviour_packs.push(BehaviourPackIdVersion { uuid, version, name });
    }

    let resource_packs_len = bin.read_varuint() as usize;
    let mut resource_packs = Vec::with_capacity(resource_packs_len);
    for _ in 0..resource_packs_len {
      let uuid = bin.read_string();
      let version = bin.read_string();
      let name = bin.read_string();
      resource_packs.push(ResourcePackIdVersion { uuid, version, name });
    }

    let game_version = bin.read_string();

    let experiments_len = bin.read_i32(Endianess::Little) as usize;
    let mut experiments = Vec::with_capacity(experiments_len);
    for _ in 0..experiments_len {
      let name = bin.read_string();
      let enabled = bin.read_bool();
      experiments.push(Experiment { name, enabled });
    }

    let experiments_previously_used = bin.read_bool();

    Self {
      must_accept,
      behaviour_packs,
      resource_packs,
      game_version,
      experiments,
      experiments_previously_used,
    }
  }
}
