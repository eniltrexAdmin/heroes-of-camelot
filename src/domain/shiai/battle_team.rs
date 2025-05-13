use crate::domain::*;
use crate::domain::shiai::damage::Damage::Physical;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BattleTeam {
    original_team: Team,
    position: ShiaiPosition,
    current_hp: BattleTeamHealthPoints,
    current_attack: BattleTeamAttack
}

impl BattleTeam {
    pub fn new(original_team: Team, position: ShiaiPosition) -> Self {
        Self{
            current_hp: BattleTeamHealthPoints::new(original_team.health_points().value()),
            current_attack: BattleTeamAttack::new(original_team.attack().value()),
            original_team,
            position
        }
    }

    pub fn position(&self) -> &ShiaiPosition {
        &self.position
    }

    pub fn current_hp(&self) -> &BattleTeamHealthPoints {
        &self.current_hp
    }

    pub fn current_attack(&self) -> &BattleTeamAttack {
        &self.current_attack
    }

    pub fn original_team(&self) -> &Team {
        &self.original_team
    }
    pub fn is_alive(&self) -> bool {
        self.current_hp.value() > 0
    }

    pub fn calculate_attack_damage(&self, attack: BattleTeamAttack) -> Damage {
        // shields etc.
        Damage::new_attack_damage(attack.value())
    }

    pub fn receive_damage(self, damage: Damage) -> Self {
        let mut new_self = self.clone();
        new_self.current_hp = match damage {
            Physical(physical_damage) => new_self.current_hp.apply_damage(physical_damage),
            Damage::Magical => new_self.current_hp.clone(),
        };
        new_self
    }
}

#[cfg(test)]
mod tests {
    use crate::data::stub_team;
    use super::*;

    #[test]
    fn test_construct() {
        let team = stub_team();
        let battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));

        assert_eq!(battle_team.original_team, team);
        assert_eq!(team.health_points().value(), battle_team.current_hp().value());
        assert_eq!(team.attack().value(), battle_team.current_attack().value());
    }

    #[test]
    fn test_is_alive() {
        let team = stub_team();
        let mut battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));
        assert!(battle_team.is_alive());

        battle_team = battle_team.receive_damage(Damage::new_attack_damage(200000));

        assert!(!battle_team.is_alive());
    }

    #[test]
    fn test_calculate_damage() {
        let team = stub_team();
        let battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));

        let result = battle_team.calculate_attack_damage(battle_team.current_attack);
        assert_eq!(battle_team.current_attack().value(), result.value())
    }
}