fn main() {
    oxide_game_engine::clear_screen_to_color(0.0, 0.0, 0.0, 0.0);

    oxide_game_engine::set_event_handler(move |key| match key {
        oxide_game_engine::Key::Left => {
            oxide_game_engine::clear_screen_to_color(1.0, 0.0, 0.0, 1.0)
        }
        oxide_game_engine::Key::Right => {
            oxide_game_engine::clear_screen_to_color(0.0, 1.0, 0.0, 1.0)
        }
        oxide_game_engine::Key::Up => oxide_game_engine::clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
        oxide_game_engine::Key::Down => {
            oxide_game_engine::clear_screen_to_color(1.0, 1.0, 1.0, 1.0)
        }
        oxide_game_engine::Key::Space => {
            oxide_game_engine::clear_screen_to_color(0.0, 0.0, 0.0, 1.0)
        }
    })
}
