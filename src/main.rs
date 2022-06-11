use macroquad::prelude::*;
use serde_json::Value;

mod level;
use level::*;
mod animated_sprite;

mod sprite_library;
use sprite_library::*;

mod hero;

struct Game {
    scale: f32,
    level: Level,
    atlas: Value,
}

impl Game {
    fn new() -> Game {
        let scale = 3.0;
        let atlas = read_atlas().unwrap();
        Game {
            scale: scale,
            level: Level::new(scale, &atlas),
            atlas,

        }
    }

    fn update(&mut self) {
        self.level.update();
        self.level.draw(self.scale);
        
    }
}

// Entry point for macroquad is the window_conf function
#[macroquad::main(window_conf())]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RuneLighter".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        //high_dpi: true,
        ..Default::default()
    }
}
