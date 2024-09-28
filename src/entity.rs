use std::fmt;

use macroquad::{
    color::{RED, WHITE},
    math::Vec2,
    shapes::{draw_circle, draw_rectangle_lines},
};

pub trait Draw {
    fn draw(&self);
}

pub struct Entity {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    debug: bool,
}

impl Entity {
    pub fn new(x: f32, y: f32, mass: f32, velocity: Vec2) -> Self {
        Self {
            position: Vec2::new(x, y),
            mass,
            velocity,
            debug: false,
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn moving(&mut self) {
        self.position += self.velocity;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            velocity: Vec2::default(),
            mass: 1.0,
            debug: false,
        }
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Entity]:\nVelocity: {}\nPosition: {}\nSpeed: {}",
            self.velocity,
            self.position,
            self.velocity.length()
        )
    }
}

impl Draw for Entity {
    fn draw(&self) {
        if self.debug {
            draw_rectangle_lines(
                self.position.x - 5.0,
                self.position.y - 5.0,
                10.0,
                10.0,
                2.0,
                RED,
            );
        }
        draw_circle(self.position.x, self.position.y, 1.0, WHITE);
    }
}
