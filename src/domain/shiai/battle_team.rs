use crate::domain::*;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BattleTeam {
    original_team: Team,
    current_hp: TeamHealthPoints,
    current_attack: BattleTeamAttack
}

impl BattleTeam {
    pub fn new(original_team: Team) -> Self {
        Self{
            current_hp: original_team.health_points().clone(),
            current_attack: BattleTeamAttack::new(original_team.attack().value()),
            original_team,
        }
    }

    pub fn current_hp(&self) -> &TeamHealthPoints {
        &self.current_hp
    }

    pub fn current_attack(&self) -> &BattleTeamAttack {
        &self.current_attack
    }

    pub fn receive_damage(&mut self, damage: &BattleTeamAttack) {

    }

    pub fn attack(&mut self, target: &mut BattleTeam) {
        target.receive_damage(&self.current_attack)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::stub_team;
    use super::*;

    #[test]
    fn test_construct() {
        let team = stub_team();
        let battle_team = BattleTeam::new(team.clone());

        assert_eq!(battle_team.original_team, team);
        assert_eq!(team.health_points(), battle_team.current_hp());
        assert_eq!(team.attack().value(), battle_team.current_attack().value());
    }

    #[test]
    fn test_damage() {
        let mut team = stub_team();
        let mut battle_team = BattleTeam::new(team.clone());

        battle_team.receive_damage(&BattleTeamAttack::new(200));
        assert_eq!(
            battle_team.current_hp().value(),
            battle_team.original_team.health_points().value() -200
        );
    }
}