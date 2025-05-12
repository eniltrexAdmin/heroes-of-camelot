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
    shiai: ShiaiResult,
    background_image: Texture2D,
    teams: HashMap<ShiaiPosition,MacroquadTeam>,
    current_turn: Option<TurnLog>,
    current_turn_number: usize,
    current_event: Option<ShiaiEvent>,
    battle_phase: BattlePhase
}

impl BattleState {
    pub async fn new(card_textures_repository: &dyn CardTexturesRepository) -> Self {
        let attacker = stub_party();
        let defender = stub_party_2();

        let result = ShiaiResult::new(attacker, defender);

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

        let first_turn =  result.turn_logs.first().unwrap().clone();

        print_shiai_turn(&first_turn, 1);
        Self{
            background_image: texture,
            teams,
            current_turn: result.turn_logs.first().cloned(),
            current_turn_number: 1,
            current_event: None,
            shiai: result,
            battle_phase: StartTurn
        }
    }

    fn advance_turn(&mut self) -> StateTransition {
        if let Some(next_turn) = self.shiai.turn_logs.get(self.current_turn_number - 1) {
            for team in next_turn.state_result.state.iter() {
                self.teams.get_mut(team.0).unwrap().update_team(team.1.clone())
            }
            print_shiai_turn(&next_turn, self.current_turn_number);

            self.current_turn_number = self.current_turn_number + 1;
            self.current_turn = Some(next_turn.clone());
            StateTransition::None
        } else {
            StateTransition::Push(HocStates::EndBattle)
        }
    }
}


impl State for BattleState {
    fn debug(&self) -> &str {
        "BattleState"
    }
    fn update(&mut self) -> StateTransition {
        if is_mouse_button_pressed(MouseButton::Left) {
            return self.advance_turn()
        }

        self.teams.values_mut().for_each(|team: &mut MacroquadTeam| {
            let is_active = self
                .current_turn
                .as_ref()
                .map_or(false, |turn| &turn.subject == team.game_team().position());

            team.update(
                is_active,
                None,
            )
        });

        StateTransition::None
    }

    fn draw(&self) {
        macroquad_draw_background(&self.background_image);
        self.teams.values().for_each(|team: &MacroquadTeam| {
            team.draw()
        });
    }
}