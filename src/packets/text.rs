use protocol_derive::packet;
use napi::bindgen_prelude::*;

// TODO: Add logic to handle the different text types

#[packet(0xa9)]
pub struct TextPacket {
  #[napi(ts_type = "TextType")]
  pub text_type: u8,
  pub needs_translation: bool,
  pub xuid: String,
  pub platform_chat_id: String,
}

#[napi]
pub enum TextType {
  Raw,
  Chat,
  Translation,
  Popup,
  JukeboxPopup,
  Tip,
  System,
  Whisper,
  Announcement,
  JsonWhisper,
  Json,
  JsonAnnouncement,
}
