use macroquad::prelude::*;

pub struct AnimatedSprite{
    pub rect: Rect,
    max_frame: i32,
    current_frame: i32,
    time: i32,
    ellapsed_tick: i32,
    is_playing: bool,
}

impl AnimatedSprite{
    pub fn new() -> AnimatedSprite{
        AnimatedSprite{
            rect: Rect {
                x: 0.0,
                y: 23.0,
                w: 19.0,
                h: 23.0,
            },
            max_frame: 8,
            current_frame: 0,
            time: 5,
            ellapsed_tick: 0,
            is_playing: true
        }
    }
    pub fn animate(&mut self) {
        if self.is_playing {
            self.ellapsed_tick += 1;
            self.ellapsed_tick = self.ellapsed_tick % self.time;
            if self.ellapsed_tick == 0 {
                self.current_frame += 1;
                if self.current_frame > self.max_frame - 1 {
                    self.current_frame = 0;
                }
                self.rect.x = self.current_frame as f32 * self.rect.w;
            }

        }
    }

    pub fn set_animation(&mut self, rect: Rect, max_frame: i32, time: i32) {
        self.rect = rect;
        self.max_frame = max_frame;
        self.time = time;
        self.current_frame = 0;
        self.ellapsed_tick = 0;
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;

    }
}
