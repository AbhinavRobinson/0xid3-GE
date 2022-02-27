use oxide_game_engine::*;

fn main() {
    clear_screen_to_color(0.0, 0.0, 0.3, 1.0);

    let mut x_position = 200.0;
    let mut y_position = 30.0;

    draw_rectangle(x_position, y_position, 100., 100.);

    set_event_handler(move |key| {
        let move_amount = 20.0;
        match key {
            Key::Left => x_position -= move_amount,
            Key::Right => x_position += move_amount,
            Key::Up => y_position += move_amount,
            Key::Down => y_position -= move_amount,
            Key::Space => {}
        }

        clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        draw_rectangle(x_position, y_position, 100., 100.);
    })
}
