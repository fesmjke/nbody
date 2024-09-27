use macroquad::{color::WHITE, math::Vec2, shapes::draw_circle};

pub trait Draw {
    fn draw(&self);
}

pub struct Entity {
    position: Vec2,
}

impl Entity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }

    pub fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
        }
    }
}

impl Draw for Entity {
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 1.0, WHITE);
    }
}
