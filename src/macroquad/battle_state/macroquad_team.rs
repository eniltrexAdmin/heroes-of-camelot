use crate::domain::*;
use crate::macroquad::battle_state::battle_state::BattlePhaseTurn;
use super::*;

pub struct MacroquadTeam {
    game_team: BattleTeam,
    cards: CardTeam,
    team_layout: TeamLayout,
    rotation: f32,
    active: bool,
    animation: Option<BattlePhaseTurn>
}


impl MacroquadTeam {
    pub fn new(game_team: &BattleTeam,  cards_textures: CardTextures, textures: TeamLayoutTextures,) -> Self {
        let rotation = match game_team.position() {
            AttackParty(_) => 0.0,
            DefenseParty(_) => std::f32::consts::PI
        };
        let team_layout =  TeamLayout::new(
            game_team.position(),
            textures,
            game_team.original_team().health_points().value(),
            game_team.current_hp().value(),
            game_team.current_attack().value(),
        );
        Self{
            game_team: game_team.clone(),
            cards: CardTeam::new(cards_textures, game_team.position()),
            team_layout,
            rotation,
            active: false,
            animation: None
        }
    }

    pub fn game_team(&self) -> &BattleTeam {
        &self.game_team
    }

    pub fn update_team(&mut self, game_team: BattleTeam) {
        self.game_team = game_team;
    }

    pub fn set_animation(&mut self, animation: BattlePhaseTurn) {
        match animation {
            BattlePhaseTurn::StartTurn{active_team} => {
                if self.game_team.position() == &active_team {
                    self.cards.set_animation(CardAnimationKind::StartTurn);
                    self.team_layout.set_animation(Some(TeamLayoutAnimation::Active));
                } else {
                    self.cards.set_animation(CardAnimationKind::Idle);
                    self.team_layout.set_animation(None);
                }
            }
            BattlePhaseTurn::Attack{attacker, target} => {
                if self.game_team.position() == &attacker {
                    self.cards.set_animation(CardAnimationKind::Attack);
                    self.team_layout.set_animation(Some(TeamLayoutAnimation::Active));
                } else if self.game_team.position() == &target {
                    self.cards.set_animation(CardAnimationKind::Idle);
                    self.team_layout.set_animation(Some(TeamLayoutAnimation::Damage));
                } else {
                    self.cards.set_animation(CardAnimationKind::Idle);
                    self.team_layout.set_animation(None);
                }
            },
            BattlePhaseTurn::AttackReturn {attacker} => {
                if self.game_team.position() == &attacker {
                    self.cards.set_animation(CardAnimationKind::AttackReturn);
                }
            },
            BattlePhaseTurn::EndTurn => {
                self.cards.set_animation(CardAnimationKind::EndTurn);
                self.team_layout.set_animation(None);
            }
        }
    }

    pub fn is_animation_finished(&self) -> bool {
        self.team_layout.animation_finished() && self.cards.is_animation_finished()
    }

    pub fn update(&mut self) {
        self.team_layout.update(self.game_team.current_hp().value());
        self.cards.update();
    }

    pub fn draw(&self) {
        self.cards.draw();
        self.team_layout.draw();
    }
}



