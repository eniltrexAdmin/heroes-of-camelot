use crate::domain::*;
use super::*;

pub struct MacroquadTeam {
    game_team: BattleTeam,
    cards: CardTeam,
    team_layout: TeamLayout,
    rotation: f32,
}


impl MacroquadTeam {
    pub fn new(game_team: &BattleTeam,  cards: CardTextures, textures: TeamLayoutTextures,) -> Self {
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
            cards: CardTeam::new(cards, game_team.position()),
            team_layout,
            rotation,

        }
    }

    pub fn update(&mut self) {
        self.team_layout.update(self.game_team.current_hp().value());
    }

    pub fn draw(&self) {
        // TODO decide whether do realtime resize or not.
        self.team_layout.draw();
        self.cards.draw();
    }
}



