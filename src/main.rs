use heroes_of_camelot::data::*;
use heroes_of_camelot::domain::shiai::shiai;
use macroquad::prelude::*;
use heroes_of_camelot::macroquad::presentation_state::PresentationState;
use heroes_of_camelot::macroquad::StackState;

#[macroquad::main(window_conf)]
async fn main() {
    let mut stack_state = StackState::new();
    let presentation_state = PresentationState::new().await;
    stack_state.push(Box::new(presentation_state));
    loop {
        stack_state.update();
        stack_state.draw();

        next_frame().await
    }
}

fn window_conf() -> Conf{
    Conf {
        window_title: "Heroes of Camelot".to_owned(),
        fullscreen: false,
        window_width: 1280,  // Wider than tall
        window_height: 720,  // Matches a 16:9 landscape layout
        ..Default::default()
    }
}