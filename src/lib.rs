extern "C" {
    /// Clears the screen to the given color.
    pub fn js_clear_screen_to_color(r: f32, g: f32, b: f32, a: f32);
    pub fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32);
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
pub fn clear_screen_to_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        js_clear_screen_to_color(r, g, b, a);
    }
}

pub fn draw_rectangle(x: f32, y: f32, width: f32, height: f32) {
    unsafe {
        js_draw_rectangle(x, y, width, height);
    }
}

thread_local! {
    /// Thread's "Local" context, for a single threaded app it's global!
    pub static EVENT_HANDLER: std::cell::RefCell<Box<dyn FnMut(Key)>> = std::cell::RefCell::new(Box::new(|_|{}));
}

/// Add Event Handler to global
pub fn set_event_handler(function: impl FnMut(Key) + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    })
}

/// Changes color of background on key press.
///
/// ### Panics
/// This function will panic if the JS function is not found.
///
/// ### Notes
/// This function responds to action handled in javascript.
#[no_mangle]
extern "C" fn key_pressed(value: usize) {
    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    EVENT_HANDLER.with(|event_handler| (event_handler.borrow_mut())(key))
}

/// Key press enums
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
}
