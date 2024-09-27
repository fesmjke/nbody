pub fn cross(w: f32, h: f32) {
    draw_line(w / 2.0, 0.0, w / 2.0, h, 1.0, RED);
    draw_line(0.0, h / 2.0, w, h / 2.0, 1.0, RED);
}
