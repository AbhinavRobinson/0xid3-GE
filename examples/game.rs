fn main() {
    oxide_game_engine::clear_screen_to_color(0.0, 0.0, 0.0, 0.0);

    let mut blue_amount = 0.0;

    oxide_game_engine::set_event_handler(move || {
        blue_amount += 0.1;
        oxide_game_engine::clear_screen_to_color(0.0, 0.0, blue_amount, 1.0);
    });
}
