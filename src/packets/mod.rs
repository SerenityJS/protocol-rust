use napi::bindgen_prelude::*;

#[napi(object)]
pub struct BehaviourPackInfo {
  pub uuid: String,
  pub version: String,
  pub size: BigInt,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
}

#[napi(object)]
pub struct ResourcePackInfo {
  pub uuid: String,
  pub version: String,
  pub size: BigInt,
  pub content_key: String,
  pub sub_pack_name: String,
  pub content_identity: String,
  pub has_scripts: bool,
  pub rtx_enabled: bool,
}


