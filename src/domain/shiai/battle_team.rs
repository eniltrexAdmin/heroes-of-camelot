use crate::domain::*;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BattleTeam {
    original_team: Team,
    position: ShiaiPosition,
    current_hp: TeamHealthPoints,
    current_attack: BattleTeamAttack
}

impl BattleTeam {
    pub fn new(original_team: Team, position: ShiaiPosition) -> Self {
        Self{
            current_hp: original_team.health_points().clone(),
            current_attack: BattleTeamAttack::new(original_team.attack().value()),
            original_team,
            position
        }
    }

    pub fn position(&self) -> &ShiaiPosition {
        &self.position
    }

    pub fn current_hp(&self) -> &TeamHealthPoints {
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
    pub fn receive_damage(&mut self, damage: Damage) -> DamageReceived {
        DamageReceived{
            team: self.position.clone(),
            damage: damage.clone()
        }
    }

    pub fn reflect_damage(&self,  damage: &BattleTeamAttack) -> Option<PhysicalDamage> {
        None
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
        assert_eq!(team.health_points(), battle_team.current_hp());
        assert_eq!(team.attack().value(), battle_team.current_attack().value());
    }

    #[test]
    fn test_damage() {
        let mut battle_team = BattleTeam::new(stub_team(), AttackParty(CaptainTeam));

        let events = battle_team.receive_damage(Damage::new_attack_damage(BattleTeamAttack::new(200)));
        assert_eq!(
            battle_team.current_hp().value(),
            battle_team.original_team.health_points().value() -200
        );

        assert_eq!(events.damage.value(), 200);
        assert_eq!(&events.team, battle_team.position());

        battle_team.receive_damage(Damage::new_attack_damage(BattleTeamAttack::new(200000)));
        assert_eq!(0, battle_team.current_hp().value());
    }

    #[test]
    fn test_is_alive() {
        let team = stub_team();
        let mut battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));
        assert!(battle_team.is_alive());

        battle_team.receive_damage(Damage::new_attack_damage(BattleTeamAttack::new(200000)));

        assert!(!battle_team.is_alive());
    }

    #[test]
    fn test_receive_reflect_damage() {
        let reflect_damage = Damage::new_reflected_damage(BattleTeamAttack::new(200));
        let team = stub_team();
        let mut battle_team = BattleTeam::new(team.clone(), AttackParty(CaptainTeam));
        battle_team.receive_damage(reflect_damage);

        assert_eq!(
            battle_team.current_hp().value(),
            battle_team.original_team.health_points().value() -200
        );
    }
}