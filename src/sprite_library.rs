use std::fs::File;
use std::path::Path;

use serde_json::{Result, Value};
use std::collections::HashMap;
//use serde_json::Result;


pub struct SpriteLibraryData {
  x: i32,
  y: i32,
  w: i32,
  h: i32,
  frame: i32,
  speed: i32,
  path: String,
}

pub fn read_atlas() -> Result<Value> {
  let json_file_path = Path::new("./assets/atlas.json");
  let file = File::open(json_file_path).expect("erreur lecture: ");
    // Parse the string of data into serde_json::Value.
  let v: Value = serde_json::from_reader(file).unwrap();

    // Access parts of the data by indexing with square brackets.
  //println!("Please call {} at the number {}", v["hero_walk_left"], v["speed"]);
  

  Ok(v)
}



fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
  let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
  let mut map = HashMap::new();
  for key in keys {
      let (k, v) = lookup.remove_entry (key).unwrap();
      map.insert(k, v);
  }
  Ok(map)
}