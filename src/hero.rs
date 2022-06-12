use macroquad::prelude::*;

use std::collections::HashMap;

use crate::animated_sprite::AnimatedSprite;
use crate::sprite_library::{SpriteLibraryData, extract_data, get_path};

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum AnimationState {
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
    Idle,
}

struct LibData {
    rect: Rect,
    speed: i32,
    frames: i32,
}

pub struct Hero {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
    dx: f32,
    dy: f32,
    speed: f32,
    direction: Vec2,
    animation_state: AnimationState,
    animation_list: HashMap<AnimationState, SpriteLibraryData>,
    collision_box: Rect,
}

impl Hero {
    pub fn new(xo: f32, yo: f32, atlas: &HashMap<String, SpriteLibraryData>) -> Hero {
        


        //--- test ici

        let mut animation_list = HashMap::new();
        animation_list.insert(
            AnimationState::WalkLeft,
            extract_data(atlas, "hero_walk_left".to_string())
        );
        animation_list.insert(
            AnimationState::WalkRight,
            extract_data(atlas, "hero_walk_right".to_string())
            
        );
        

        let sprite_texture = Texture2D::from_file_with_format(include_bytes!("../assets/hero.png"), None);
        sprite_texture.set_filter(FilterMode::Nearest);
        
        Hero {
            x: xo,
            y: yo,
            texture: sprite_texture,
            sprite: AnimatedSprite::new(),
            speed: 1.5,
            direction: Vec2::new(0.0, 0.0),
            animation_state: AnimationState::WalkRight,
            animation_list: animation_list,
            dx: 0.0,
            dy: 0.0,
            collision_box: Rect::new(0.0, 0.0, 19.0, 16.0),
        }
    }

    pub fn update(&mut self, collision_grid: &[i32; 350]) {
        self.sprite.animate();
        if is_key_down(KeyCode::Left) {
            self.direction.x = -1.0;
        } else if is_key_down(KeyCode::Right) {
            self.direction.x = 1.0;
        } else {
            self.direction.x = 0.0;
        }
        if is_key_down(KeyCode::Down) {
            self.direction.y = 1.0;
        } else if is_key_down(KeyCode::Up) {
            self.direction.y = -1.0;
        } else {
            self.direction.y = 0.0;
        }

        self.dx = self.direction.x * self.speed;
        self.dy = self.direction.y * self.speed;

        // Collision managment
        let dest_xa = ((self.x + self.dx) / 16.0) as i32;
        let dest_ya = (self.y / 16.0) as i32;

        let dest_xb = (self.x / 16.0) as i32;
        let dest_yb = ((self.y + self.dy) / 16.0) as i32;

        let check_list = [[0,0], [0,1], [1,0], [1,1]];
        // Collision on x
        for tile in check_list {
            let i = tile[0];
            let j = tile[1];
            if self.has_collision(collision_grid, dest_xa + i, dest_ya + j, self.dx, 0.0) == 1 {
                self.dx = 0.0;
                break;
            }
        }

        // Collsion on y
        for tile in check_list {
            let i = tile[0];
            let j = tile[1];
            if self.has_collision(collision_grid, dest_xb + i, dest_yb + j, 0.0, self.dy) == 1 {
                self.dy = 0.0;
                break;
            }
        }

        



        self.animation_manager();


        self.x += self.dx;
        self.y += self.dy;
    }

    fn intersect_rect(&self, box1: Rect, box2: Rect) -> bool {
		let r1_right = box1.x + box1.w;
		let r1_left = box1.x;
		let r1_top = box1.y;
		let r1_bottom = box1.y + box1.h;
		
		let r2_right = box2.x + box2.w;
		let r2_left = box2.x;
		let r2_top = box2.y;
		let r2_bottom = box2.h + box2.h;

        r1_left < r2_right && r1_right > r2_left && r1_top  < r2_bottom && r1_bottom > r2_top
    }

    fn has_collision(&self, grid: &[i32; 350], x: i32, y: i32, dx: f32, dy: f32) -> i32 {
        let rect_tile = Rect::new((x * 16) as f32, (y * 16) as f32, 16.0, 16.0);
        let rect_hero = Rect::new(
            self.collision_box.x +  dx + self.x,
            self.collision_box.y +  dy + self.y,
            self.collision_box.w,
            self.collision_box.h
        );
        if grid[(x + y * 25) as usize] != 0 // && self.intersect_rect(rect_tile, rect_hero)
        {
            grid[(x + y * 25) as usize]
        } else {
            0
        }
    }

    fn animation_manager(&mut self) {
        let current_animation_state = self.animation_state;
        if self.direction.x == -1.0 {
            self.animation_state = AnimationState::WalkLeft;
        } else if self.direction.x == 1.0 {
            self.animation_state = AnimationState::WalkRight;
        } else if self.direction.x == 0.0 && self.direction.y == 0.0 {
            self.animation_state = AnimationState::Idle;
        }

        if current_animation_state != self.animation_state {
            if self.animation_state == AnimationState::Idle {
                self.sprite.stop();

            }
            else {
                let data = self.animation_list.get(&self.animation_state).unwrap();
                self.sprite
                    .set_animation(data);
            }
        }
    }
}
