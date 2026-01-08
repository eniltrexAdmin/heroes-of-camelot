use std::collections::{HashMap, VecDeque};
use async_trait::async_trait;
use macroquad::prelude::*;
use crate::data::{stub_party, stub_party_2};
use crate::domain::*;
use crate::macroquad::*;
use super::*;

#[async_trait]
pub trait CardTexturesRepository {
    async fn load_for_team(&self, team: &Team) -> CardTextures;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BattlePhaseTurn {
    StartTurn{active_team: BattlePosition },
    Attack{attacker: BattlePosition, target: BattlePosition },
    AttackReturn{attacker: BattlePosition },
    EndTurn,
}

pub struct BattleState {
    shiai: BattleResult,
    background_image: Texture2D,
    teams: HashMap<BattlePosition,MacroquadTeam>,
    current_turn: Option<TurnLog>,
    current_turn_number: usize,
    current_event: Option<BattleEvent>,
    current_event_number: usize,
    battle_phase_queue: VecDeque<BattlePhaseTurn>
}

impl BattleState {
    pub async fn new(card_textures_repository: &dyn CardTexturesRepository) -> Self {
        let attacker = stub_party();
        let defender = stub_party_2();

        let result = BattleResult::new(attacker, defender);

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

        print_battle_turn(&first_turn, 1);
        Self{
            background_image: texture,
            teams,
            current_turn: None,
            current_turn_number: 0,
            current_event: None,
            current_event_number: 0,
            shiai: result,
            battle_phase_queue: VecDeque::new(),
        }
    }

    fn advance_turn(&mut self) -> StateTransition {
        if let Some(next_turn) = self.shiai.turn_logs.get(self.current_turn_number) {
            let current_phase = BattlePhaseTurn::StartTurn {active_team: next_turn.subject.clone()};

            for team in next_turn.state_result.state.iter() {
                self.teams.get_mut(team.0).unwrap().update_team(team.1.clone());
                self.teams.get_mut(team.0).unwrap().set_animation(current_phase.clone());
            }
            self.current_turn_number = self.current_turn_number + 1;
            self.current_turn = Some(next_turn.clone());
            self.current_event_number = 0;

            print_battle_turn(&next_turn, self.current_turn_number);
            StateTransition::None
        } else {
            StateTransition::Push(HocStates::EndBattle)
        }
    }

    fn advance_turn_event(&mut self)  {
        if let Some(current_turn) = &self.current_turn {
            if let Some(next_event) = current_turn.events.get(self.current_event_number) {
                self.current_event_number = self.current_event_number + 1;
                self.current_event = Some(next_event.clone());

                match next_event {
                    TeamAttacked(team_attacked) => {
                        let attack_phase = BattlePhaseTurn::Attack {
                            attacker: team_attacked.attacker.clone(),
                            target: team_attacked.target.clone(),
                        };
                        let attack_return = BattlePhaseTurn::AttackReturn {
                            attacker: team_attacked.attacker.clone(),
                        };
                        self.battle_phase_queue = VecDeque::from(vec![
                            attack_phase, attack_return
                        ]);
                    }
                }
            }
        }
    }

    fn advance_phase_turn(&mut self) {
        let all_done = self.teams.values().all(|t| t.is_animation_finished());
        if !all_done { return; }
        if let Some(phase) = self.battle_phase_queue.pop_front() {
            self.teams.values_mut().for_each(|team| {
                team.set_animation(phase.clone());
            });
        } else {
            self.advance_turn_event(); // queue empty, move to next event
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
            team.update()
        });

        self.advance_phase_turn();

        StateTransition::None
    }

    fn draw(&self) {
        macroquad_draw_background(&self.background_image);
        self.teams.values().for_each(|team: &MacroquadTeam| {
            team.draw()
        });
    }
}