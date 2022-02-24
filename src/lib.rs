extern "C" {
    /// Clears the screen to the given color.
    pub fn js_clear_screen_to_color(r: f32, g: f32, b: f32, a: f32);
}

/// Clears the screen to a color
///
/// ### Arguments
/// * `r` - Red component of the color
/// * `g` - Green component of the color
/// * `b` - Blue component of the color
/// * `a` - Alpha component of the color
///
/// ### Example
/// ```
/// use oxide_game_engine::clear_screen_to_color;
///
/// clear_screen_to_color(0.175, 0.2, 0.275, 1.0);
/// ```
///
/// ### Safety
/// This function is unsafe because it calls a JS function that is not
/// guaranteed to be safe.
///
/// ### Panics
/// This function will panic if the JS function is not found.
///
/// ### Notes
/// This function is not thread safe.
pub fn clear_screen_to_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        js_clear_screen_to_color(r, g, b, a);
    }
}
