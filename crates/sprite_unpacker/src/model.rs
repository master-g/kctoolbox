use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SpriteSheet {
  pub frames: HashMap<String, FrameValue>,
  pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameValue {
  pub frame: SpriteSourceSize,
  pub rotated: bool,
  pub trimmed: bool,
  pub sprite_source_size: SpriteSourceSize,
  pub source_size: Size,
}

#[derive(Serialize, Deserialize)]
pub struct SpriteSourceSize {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
  pub w: u32,
  pub h: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
  pub app: String,
  pub image: String,
  pub format: String,
  pub size: Size,
  pub scale: u32,
}
