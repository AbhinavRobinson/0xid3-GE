extern "C" {
    pub fn js_clear_screen_to_color(r: f32, g: f32, b: f32, a: f32);
}

pub fn clear_screen_to_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        js_clear_screen_to_color(r, g, b, a);
    }
}
