use crate::domain::*;
use crate::domain::shiai::damage::Damage::Physical;
use crate::domain::ShiaiEventType::DamageReceived;
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

    pub fn receive_attack(&self, subject: &BattleTeam) -> Vec<ShiaiEvent> {
        // shields etc.
        let damage = Damage::new_attack_damage(subject.current_attack().clone());
        vec![ShiaiEvent::new_damage_received(self.position.clone(), damage)]
    }

    pub fn apply_domain_event(self, shiai_event_type: ShiaiEventType) -> Self {
        match shiai_event_type {
            DamageReceived(damage) => {
                self.apply_receive_damage(damage)
            }
        }
    }

    // this might return other domain events...
    fn apply_receive_damage(self, damage: Damage) -> Self {
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

        battle_team.apply_receive_damage(Damage::new_attack_damage(BattleTeamAttack::new(200000)));

        assert!(!battle_team.is_alive());
    }

    #[test]
    fn test_receive_attack() {
        let team = stub_team();
        let battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));

        let result = battle_team.receive_attack(&battle_team);
        assert_eq!(result.len(), 1);
        let event = result.get(0).unwrap();
        assert_eq!(event.target, battle_team.position);

        if let DamageReceived(damage) = &event.event {
            assert_eq!(battle_team.current_attack().value(), damage.value())
        }
    }
}