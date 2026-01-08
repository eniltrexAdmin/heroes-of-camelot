use super::*;
use crate::domain::management::*;
mod battle_team_attack;
use battle_team_attack::BattleTeamAttack;
mod battle_team_hp;
use battle_team_hp::BattleTeamHealthPoints;

mod increase_attack_skill;
mod attack;
pub use attack::*;


#[derive(Debug, Clone, PartialEq)]
pub struct BattleTeam {
    original_team: Team,
    position: BattlePosition,
    current_hp: BattleTeamHealthPoints,
    current_attack: BattleTeamAttack
}

impl BattleTeam {
    pub fn new(original_team: Team, position: BattlePosition) -> Self {
        Self{
            current_hp: BattleTeamHealthPoints::new(original_team.health_points().value()),
            current_attack: BattleTeamAttack::new(original_team.attack().value()),
            original_team,
            position
        }
    }

    pub fn position(&self) -> &BattlePosition {
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

        let event = TeamAttackedDomainEvent {
            attacker: battle_team.position().clone(),
            target: battle_team.position().clone(),
            damage_received:   AttackDamage::new_attack_damage(battle_team.current_hp.value())
        };

        battle_team.apply_team_attacked_domain_event(event);

        assert!(!battle_team.is_alive());
    }


}