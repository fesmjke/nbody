use macroquad::{color::RED, math::Vec2, rand::gen_range, shapes::draw_line};

pub fn cross(w: f32, h: f32) {
    draw_line(w / 2.0, 0.0, w / 2.0, h, 1.0, RED);
    draw_line(0.0, h / 2.0, w, h / 2.0, 1.0, RED);
}

pub fn rand_vec2(min: f32, max: f32) -> Vec2 {
    let x = gen_range(min, max);
    let y = gen_range(min, max);

    Vec2::new(x, y)
}

pub fn rand_pos(w: f32, h: f32) -> Vec2 {
    let x = gen_range(0.0, w);
    let y = gen_range(0.0, h);

    Vec2::new(x, y)
}

pub fn rand_mass(min: f32, max: f32) -> f32 {
    gen_range(min, max)
}
