use std::collections::HashMap;
use async_trait::async_trait;
use macroquad::prelude::*;
use crate::data::{stub_party, stub_party_2};
use crate::domain::*;
use crate::macroquad::*;
use crate::macroquad::battle_state::battle_state::BattlePhase::StartTurn;
use super::*;

#[async_trait]
pub trait CardTexturesRepository {
    async fn load_for_team(&self, team: &Team) -> CardTextures;
}

enum BattlePhase {
    StartTurn,
    ActionAnimation,
    ResolveEffects,
    EndTurn,
}

pub struct BattleState {
    shiai: Shiai,
    background_image: Texture2D,
    teams: HashMap<ShiaiPosition,MacroquadTeam>,
    current_turn: ShiaiTurn,
    current_action: ShiaiAction,
    current_events: Vec<ShiaiEvent>,
    battle_phase: BattlePhase
}

impl BattleState {
    pub async fn new(card_textures_repository: &dyn CardTexturesRepository) -> Self {
        let attacker = stub_party();
        let defender = stub_party_2();

        let shiai = Shiai::new(attacker, defender);
        let result = shiai.battle();

        // Team Layout

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

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/battle_state/team/empty_life_bar.png")
        );
        let life_bar_container = Texture2D::from_file_with_format(bytes, None);
        life_bar_container.set_filter(FilterMode::Linear);

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/battle_state/team/full_life_bar.png")
        );
        let life_bar = Texture2D::from_file_with_format(bytes, None);
        life_bar.set_filter(FilterMode::Linear);

        let states_textures = TeamLayoutTextures {
            life_bar_container,
            life_bar,
            stats_background_texture,
            stats_border,
            stats_label_background
        };

        let mut teams = HashMap::new();

        for team in result.init_state.state.iter() {
            teams.insert(team.0.clone(), MacroquadTeam::new(
                team.1,
                card_textures_repository.load_for_team(team.1.original_team()).await,
                states_textures.clone())
            );
        }

        let bytes = include_bytes!(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/background.png")
        );
        let texture = Texture2D::from_file_with_format(bytes, None);

        let first_turn =  result.result.get(0).unwrap().clone();

        print_shiai_turn(&first_turn);
        Self{
            background_image: texture,
            teams,
            current_action: first_turn.actions.get(0).unwrap().clone(),
            current_turn: first_turn,
            current_events: Vec::new(),
            shiai: result,
            battle_phase: StartTurn
        }
    }

    fn advance_turn(&mut self) {

        let next_turn_number = self.current_turn.number;
        println!("number is{}", next_turn_number);
        let next_turn =  self.shiai.result.get(next_turn_number).unwrap().clone();

        for team in next_turn.state_result.state.iter() {
            self.teams.get_mut(team.0).unwrap().update_team(team.1.clone())
        }
        print_shiai_turn(&next_turn);

        self.current_action = next_turn.actions.get(0).unwrap().clone();
        self.current_turn = next_turn;

    }
}


impl State for BattleState {
    fn debug(&self) -> &str {
        "BattleState"
    }
    fn update(&mut self) -> StateTransition {
        self.teams.values_mut().for_each(|team: &mut MacroquadTeam| {
            let current_action_team = &self.current_turn.subject;
            let current_action =
                match current_action_team == team.game_team().position() {
                    true => Some( self.current_action.command.clone()),
                    false =>None
                };
            team.update(
                current_action.is_some(),
                current_action,
            )
        });

        if is_mouse_button_pressed(MouseButton::Left) {
            self.advance_turn();
        }

        StateTransition::None
    }

    fn draw(&self) {
        macroquad_draw_background(&self.background_image);
        self.teams.values().for_each(|team: &MacroquadTeam| {
            team.draw()
        });
    }
}