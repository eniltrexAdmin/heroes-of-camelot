use macroquad::prelude::{draw_text, is_mouse_button_pressed, screen_height, screen_width, MouseButton, WHITE};
use crate::macroquad::{HocStates, State, StateTransition};

pub struct EndBattleState {}

impl State for EndBattleState {
    fn debug(&self) -> &str {
        "EndBattleState"
    }

    fn update(&mut self) -> StateTransition {

        if is_mouse_button_pressed(MouseButton::Left) {
           StateTransition::Switch(HocStates::Battle)
        } else {
            StateTransition::None
        }

    }

    fn draw(&self) {
        draw_text(
            "END!!!".to_string().as_str(),
            screen_width() / 2.0,
            screen_height() / 2.0,
            50.0,
            WHITE
        );
    }
}