use macroquad::prelude::*;
use crate::data::{stub_party, stub_party_2};
use crate::domain::*;
use crate::macroquad::*;
use crate::macroquad::battle_state::macroquad_team::{MacroquadTeam, StatsTextures};

pub struct BattleState {
    shiai_result: ShiaiResult,
    background_image: Texture2D,
    teams: Vec<MacroquadTeam>,
}

impl BattleState {
    pub fn new() -> Self {
        let attacker = stub_party();
        let defender = stub_party_2();

        let mut teams = Vec::new();
        let result = shiai::battle(attacker, defender);


        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/battle_state/team/stats_background.png")
        );
        let stats_background_texture = Texture2D::from_file_with_format(bytes, None);
        stats_background_texture.set_filter(FilterMode::Linear);

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/battle_state/team/stats_border.png")
        );
        let stats_border = Texture2D::from_file_with_format(bytes, None);
        stats_border.set_filter(FilterMode::Linear);

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/battle_state/team/stats_label_background.png")
        );
        let stats_label_background = Texture2D::from_file_with_format(bytes, None);
        stats_label_background.set_filter(FilterMode::Linear);

        let states_textures = StatsTextures{
            stats_background_texture,
            stats_border,
            stats_label_background
        };

        result.teams().iter().for_each(|team| {
            teams.push(MacroquadTeam::new(team.1, states_textures.clone()));
        });

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/background.png")
        );
        let texture = Texture2D::from_file_with_format(bytes, None);
        Self{
            shiai_result: result,
            background_image: texture,
            teams
        }
    }
}

impl State for BattleState {
    fn debug(&self) -> &str {
        "BattleState"
    }
    fn update(&mut self) -> StateTransition {
        StateTransition::None
    }

    fn draw(&self) {
        macroquad_draw_background(&self.background_image);
        self.teams.iter().for_each(|team: &MacroquadTeam| {
            team.draw()
        });

    }
}