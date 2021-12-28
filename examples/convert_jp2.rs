use std::fs::File;
use std::io::prelude::*;
use std::env;

use anyhow::Result;

use image::DynamicImage;

use jpeg2k::*;

fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let jp2_filename = env::args().nth(1)
    .unwrap_or_else(|| "test.j2k".to_string());
  let savename = env::args().nth(2)
    .unwrap_or_else(|| "test.jpg".to_string());

  let mut file = File::open(jp2_filename)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf)?;

  let jp2_image = Image::from_bytes(&mut buf)?;

  println!("jp2_image: width={:?}, height={:?}", jp2_image.width(), jp2_image.height());

  let img: DynamicImage = jp2_image.try_into()?;
  img.save(&savename)?;

  println!("Saved to: {}", savename);
  Ok(())
}
