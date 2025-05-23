use crate::domain::*;
use super::*;


pub struct MacroquadTeam {
    game_team: BattleTeam,
    cards: CardTeam,
    team_layout: TeamLayout,
    rotation: f32,
    active: bool,
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
            cards: CardTeam::new(cards_textures, game_team.position(), team_layout.background_rectangle()),
            team_layout,
            rotation,
            active: false
        }
    }

    pub fn game_team(&self) -> &BattleTeam {
        &self.game_team
    }

    pub fn update_team(&mut self, game_team: BattleTeam) {
        self.game_team = game_team;
    }

    pub fn update(&mut self, active:bool, current_event: Option<ShiaiEvent>) {
        self.team_layout.update(self.game_team.current_hp().value());
        self.cards.update(self.team_layout.background_rectangle(), active);
    }

    pub fn draw(&self) {
        self.cards.draw();
        self.team_layout.draw();
    }
}



