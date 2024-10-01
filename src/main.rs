use entity::{Draw, Entity};
use helper::{cross, rand_mass, rand_pos, rand_vec2};
use macroquad::{prelude::*, telemetry::enable};

mod entity;
mod helper;

#[macroquad::main("N-Body")]
async fn main() {

    let quantity = 100;
    let night_blue= Color::from_rgba(0, 0, 25, 255);
    let mut entities : Vec<Entity> = Vec::with_capacity(quantity);
    let (width, height) = (screen_width(), screen_height());

    for _ in 0..quantity {
        let position = rand_pos(width , height );
        let mass = rand_mass(1.0, 5.0);
        let mut entity = Entity::default();
        entity.set_position(position);
        entity.set_mass(mass);
        entities.push(entity);
    }

    entities[0].toggle_debug();

    loop {
        clear_background(night_blue);        
        cross(width, height);

        // let delta_time = get_frame_time();
        println!("{:?}", entities[0]);

        for i in 0..entities.len() {
            let mut force = Vec2::ZERO;
            
            for j in 0..entities.len() {
                if i == j {
                    continue;
                }
                
                force = entities[j].force_attraction(&entities[i]);

                if i == 0 {
                    println!("attraction force to {:?} -> {:?}",j, force);
                }
                entities[i].apply_force(force);
            }

            entities[i].moving();
            entities[i].draw();
        }
        
        next_frame().await
    }
}
