use entity::{Draw, Entity};
use helper::{cross, rand_pos, rand_vec2};
use macroquad::prelude::*;

mod entity;
mod helper;

#[macroquad::main("N-Body")]
async fn main() {

    let quantity = 100;
    let night_blue= Color::from_rgba(0, 0, 25, 255);
    let mut entities : Vec<Entity> = Vec::with_capacity(quantity);
    let (width, height) = (screen_width(), screen_height());

    for _ in 0..quantity {
        let velocity = rand_vec2(-0.01, 0.01);
        let position = rand_pos(width, height );
        let mut entity = Entity::default();
        entity.set_position(position);
        entity.set_velocity(velocity);

        entities.push(entity);
    }
    
    loop {
        clear_background(night_blue);        
        cross(width, height);
        
        for entity in entities.iter_mut() {
            entity.draw();
            entity.moving();
        }
        
        next_frame().await
    }
}
