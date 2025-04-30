use macroquad::color::{BLUE, WHITE};
use macroquad::prelude::{draw_rectangle, draw_text, screen_height, screen_width};
use crate::data::{stub_party, stub_party_2};
use crate::domain::{shiai, Party};
use crate::domain::shiai::ShiaiResult;
use crate::macroquad::{State, StateTransition};

pub struct BattleState {
    shiai_result: ShiaiResult
}

impl BattleState {
    pub fn new() -> BattleState {
        let attacker = stub_party();
        let defender = stub_party_2();

        let result = shiai::battle(attacker, defender);
        Self{
            shiai_result: result
        }
    }
}

impl State for BattleState {
    fn update(&mut self) -> StateTransition {
        StateTransition::None
    }

    fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        draw_text("Score", screen_w * 0.05, screen_h * 0.1, 30.0, WHITE);
        draw_rectangle(screen_w * 0.5, screen_h * 0.5, screen_w * 0.25, screen_h * 0.1, BLUE);
    }

    fn debug(&self) -> &str {
        "BattleState"
    }
}