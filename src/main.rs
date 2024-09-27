use entity::{Draw, Entity};
use helper::{cross};
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
        let x = rand::gen_range(0.0, width);
        let y = rand::gen_range(0.0, height);
        println!("x , y - {:?} {:?}", x, y);
        let position = Vec2::new(x ,y);
        let mut entity = Entity::default();
        entity.set_position(position);

        entities.push(entity);
    }
    
    loop {
        clear_background(night_blue);        
        cross(width, height);
        for entity in entities.iter() {
            entity.draw();
        }
        
        next_frame().await
    }
}
