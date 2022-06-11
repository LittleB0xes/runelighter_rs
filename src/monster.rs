trait Monster {
    fn new(x: i32, y: i32) -> Self;
    fn update(&mut self);
    fn get_sprite(&self) -> AnimatedSprite
}