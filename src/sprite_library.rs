use std::fs::File;
use std::path::Path;
use serde::Deserialize;

use serde_json::Result;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct SpriteLibraryData {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32,
  pub frame: i32,
  pub speed: i32,
  pub path: String,
}

struct SpriteLibrary {
  data: HashMap<String, SpriteLibraryData>
}

impl SpriteLibrary {
}

pub fn read_atlas() -> Result<HashMap<String, SpriteLibraryData>> {
  let json_file_path = Path::new("./assets/atlas.json");
  let atlas_file = File::open(json_file_path).expect("erreur lecture: ");


  let atlas: HashMap<String, SpriteLibraryData> = serde_json::from_reader(atlas_file)?;

  Ok(atlas)
}

pub fn extract_data(atlas: &HashMap<String,SpriteLibraryData>, name: String) -> SpriteLibraryData {
  let mut data =  SpriteLibraryData{
    x: 0,
    y: 0,
    w: 0,
    h: 0,
    speed: 0,
    frame: 0,
    path: "".to_string()
  };
  data.x = atlas.get(&name).unwrap().x;
  data.y = atlas.get(&name).unwrap().y;
  data.w = atlas.get(&name).unwrap().w;
  data.h = atlas.get(&name).unwrap().h;
  data.frame = atlas.get(&name).unwrap().frame;
  data.speed = atlas.get(&name).unwrap().speed;
  data.path = String::from(&atlas.get(&name).unwrap().path);
  data
}

pub fn get_path(atlas: &HashMap<String,SpriteLibraryData>, name: String) -> &str {
  &atlas.get(&name).unwrap().path
}