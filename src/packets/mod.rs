#![allow(dead_code, unused_variables)]

// Shared Packet Types
mod types;

mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod disconnect;
mod resource_packs_info;
mod resource_packs_stack;
mod resource_pack_client_response;
mod text;
mod set_time;
mod start_game;
mod add_player;
mod add_entity;
mod remove_entity;
mod add_item_entity;
mod take_item_entity;
mod network_settings;
mod request_network_settings;

mod request_chunk_radius;
mod chunk_radius_update;

mod update_player_game_type;
mod emote_list;
mod position_tracking_db_request;
mod position_tracking_db_broadcast;
mod packet_violation_warning;
mod motion_prediction_hints;
mod animate_entity;
mod camera_shake;
mod player_fog;
mod correct_player_move_prediction;
mod item_component;
mod filter_text;
// mod debug_renderer;
mod sync_entity_property;
mod add_volume_entity;
mod remove_volume_entity;
mod simulation_type;
mod npc_dialogue;
mod edu_shared_resource_uri;
mod create_photo;
mod update_subchunk_blocks;
mod photo_info_request;

use protocol_derive::packet_enum;
use napi::bindgen_prelude::*;
use crate::binary::BinaryStream;

use prelude::*;

// TODO: I heavily dislike how packet ids have to be defined
// here and in the packet attribute macro. I'd like to find a
// way to only define them once ideally in the macro.

#[packet_enum]
pub enum Packet {
  Login = 0x01,
  PlayStatus = 0x02,
  ServerToClientHandshake = 0x03,
  ClientToServerHandshake = 0x04,
  Disconnect = 0x05,
  ResourcePacksInfo = 0x06,
  ResourcePacksStack = 0x07,
  ResourcePackClientResponse = 0x08,
  Text = 0x09,
  SetTime = 0x0a,
  StartGame = 0x0b,
  AddPlayer = 0x0c,
  AddEntity = 0x0d,
  RemoveEntity = 0x0e,
  AddItemEntity = 0x0f,
  TakeItemEntity = 0x11,
  NetworkSettings = 0x8f,
  RequestNetworkSettings = 0xc1,
  
  RequestChunkRadius = 0x45,
  ChunkRadiusUpdate = 0x46,

  UpdatePlayerGameType = 0x97,
  EmoteList = 0x98,
  PositionTrackingDbRequest = 0x9a,
  PositionTrackingDbBroadcast = 0x99,
  PacketViolationWarning = 0x9c,
  MotionPredictionHints = 0x9d,
  AnimateEntity = 0x9e,
  CameraShake = 0x9f,
  PlayerFog = 0xa0,
  CorrectPlayerMovePrediction = 0xa1,
  ItemComponent = 0xa2,
  FilterText = 0xa3,
  // TODO: DebugRenderer = 0xa4,
  SyncEntityProperty = 0xa5,
  AddVolumeEntity = 0xa6,
  RemoveVolumeEntity = 0xa7,
  SimulationType = 0xa8,
  NpcDialogue = 0xa9,
  EduSharedResourceUri = 0xaa,
  CreatePhoto = 0xab,
  UpdateSubchunkBlocks = 0xac,
  PhotoInfoRequest = 0xad,
}

// These must be implemented on packets so the enum serializer can work
pub mod prelude {
  pub trait PacketConversion: Sized {
    fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self>;
    fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object>;
  }
  
  pub trait PacketSerialization: Sized {
    fn serialize(&self) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Buffer>;
    fn deserialize(data: napi::bindgen_prelude::Buffer) -> napi::bindgen_prelude::Result<Self>;
  }
  
  // pub trait PacketChildConversion: Sized {
  //   fn from_object(data: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<Self>;
  //   fn to_object(&self, env: napi::bindgen_prelude::Env) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::Object>;
  // }
  
  pub trait PacketChildSerialization: Sized {
    fn serialize(&self) -> napi::bindgen_prelude::Result<crate::binary::BinaryStream>;
    fn deserialize(data: &mut crate::binary::BinaryStream) -> napi::bindgen_prelude::Result<Self>;
  }
}

// These are temporary methods;

#[napi]
pub fn get_packet_id(data: Buffer) -> i32 {
  // read varint directly from front of buffer for speed without binary stream
  let mut id = 0;
  let mut shift = 0;
  let mut i = 0;

  for byte in data.to_vec() {
    id |= ((byte & 0x7f) as i32) << shift;
    shift += 7;
    i += 1;

    if shift > 35 {
      panic!("VarInt is too big");
    }

    if (byte & 0x80) == 0 {
      break;
    }
  }

  id
}

// Frames an array of buffers into a single buffer
#[napi]
pub fn frame_packets(packets: Vec<Buffer>) -> napi::Result<Buffer> {
  let mut bin = BinaryStream::new();

  // bin.write_bytes(vec![0xfe]);

  for packet in packets {
    bin.write_varint(packet.len() as u32)?;
    bin.write(packet.into())?;
  }

  Ok(bin.data.into())
}

// Unframes a buffer of packets and returns an array of buffers
#[napi]
pub fn unframe_packets(data: Buffer) -> napi::Result<Vec<Buffer>> {
  let mut bin = BinaryStream::from(data.into());
  let mut packets = Vec::new();

  // let _id = bin.read_u8();

  while !bin.empty() {
    let len = bin.read_varint()? as usize;
    let packet = bin.read(len)?;
    packets.push(packet.into());
  }

  Ok(packets)
}

// Temporary generates motd_string for raknet
#[napi]
pub fn make_motd(
  motd: String,
  protocol_version: u32,
  version: String,
  current_players: u32,
  max_players: u32,
  server_id: String,
  world_name: String,
  gamemode: String,
  gamemode_id: u8,
  portv4: u16,
  portv6: u16,
) -> napi::Result<Buffer> {
  let str = format!(
    "MCPE;{};{};{};{};{};{};{};{};{};{};{};{};",
    motd,
    protocol_version,
    version,
    current_players,
    max_players,
    server_id,
    world_name,
    gamemode,
    gamemode_id,
    portv4,
    portv6,
    0
  );

  let mut bin = BinaryStream::new();
  bin.write_u16(str.as_bytes().len() as u16)?;
  bin.write(str.as_bytes().to_vec())?;

  Ok(bin.data.into())
}
