use macroquad::prelude::*;
use std::collections::HashMap;

mod architect;
use architect::*;

use crate::sprite_library::SpriteLibraryData;
use crate::hero::*;

pub struct Level {
  hero: Hero,
  background: Texture2D,
  camera: Vec2,
  collision_grid: [i32; 25 * 14]
}

impl Level {
  pub fn new(scale: f32, atlas: &HashMap<String, SpriteLibraryData>) -> Level {
    let bg_texture = Texture2D::from_file_with_format(include_bytes!("../../assets/LevelOne.png"), None);

    Level {
      hero: Hero::new(16.0, 16.0, atlas),
  
      background:bg_texture,
      camera: Vec2::new(
        -0.5 * (screen_width() - bg_texture.width() * scale) / scale,
        -0.5 * (screen_height() -  bg_texture.height() * scale) / scale
      ),
      collision_grid: grid_generation(),
    }
  }


  pub fn update(&mut self) {
    self.hero.update(&self.collision_grid);

    self.debug();
  }

  pub fn draw(&mut self, scale: f32) {
        clear_background(BLACK);

        draw_text(&get_fps().to_string(), 10.0, 30.0, 28.0, WHITE);

        let background_param = DrawTextureParams {
            dest_size: Some(Vec2::new(scale * self.background.width(), scale * self.background.height())),
            ..Default::default()
        };
        
        draw_texture_ex(self.background, -scale * self.camera.x, -scale * self.camera.y, WHITE, background_param);

        let draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new(scale * self.hero.sprite.rect.w, scale * self.hero.sprite.rect.h)),
            source: Some(self.hero.sprite.rect),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(self.hero.texture, scale * (self.hero.x - self.camera.x), scale * (self.hero.y - 7.0 - self.camera.y), WHITE, draw_param );



  }

  fn debug(&mut self) {
    if is_key_down(KeyCode::Q) {
      self.camera.x -= 1.0;
    }
    else if is_key_down(KeyCode::W) {
      self.camera.x += 1.0;
    }
  }
}

