use std::path::Path;
use crate::model;

use anyhow::{Context, Result};
use image::GenericImage;
use image::io::Reader as ImageReader;

pub fn execute(src: &str, dst: &str) -> Result<()> {
  // patch path if it is not ended with .json
  let path_to_json = if src.ends_with(".json") {
    src.to_string()
  } else {
    format!("{}.json", src)
  };
  // read json file
  let raw = std::fs::read_to_string(&path_to_json).with_context(|| {
    format!("failed to read json file from {}", path_to_json)
  })?;
  // parse json file
  let sprite_sheet: model::SpriteSheet =
    serde_json::from_str(&raw).context("failed to parse json file")?;

  // find image path
  let image_path = Path::new(&path_to_json)
    .parent()
    .ok_or(anyhow::anyhow!(
      "failed to find parent dir of {}",
      path_to_json
    ))?
    .join(&sprite_sheet.meta.image)
    .canonicalize()?;

  // open png file
  let mut img = ImageReader::open(image_path)
    .context("failed to open image file")?
    .decode()
    .context("failed to decode image file")?;

  // create output dir
  let name = Path::new(&sprite_sheet.meta.image)
    .file_stem()
    .ok_or(anyhow::anyhow!("failed to get file stem from meta.image"))?
    .to_str()
    .ok_or(anyhow::anyhow!("failed to convert image file name to str"))?;
  let output_dir = Path::new(dst).join(name);
  std::fs::create_dir_all(&output_dir)
    .with_context(|| format!("failed to create dir {}", dst))?;

  // for each sprite frame
  for (name, frame) in sprite_sheet.frames {
    // extract sprite frame
    let sprite = img
      .sub_image(
        frame.frame.x,
        frame.frame.y,
        frame.frame.w,
        frame.frame.h,
      )
      .to_image();
    // save sprite frame
    let png_name = format!("{}.png", &name);
    let path = output_dir.join(png_name);
    sprite
      .save_with_format(&path, image::ImageFormat::Png)
      .with_context(|| format!("failed to save sprite frame {}", &name))?;
  }

  Ok(())
}
