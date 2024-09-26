use macroquad::prelude::*;

#[macroquad::main("N-Body")]
async fn main() {
    let night_blue= Color::from_rgba(0, 0, 25, 255);
    loop {
        clear_background(night_blue);                 
        next_frame().await
    }
}
