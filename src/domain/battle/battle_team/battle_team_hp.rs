use std::cmp::min;
use crate::domain::battle::battle_team::attack::damage::PhysicalDamage;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BattleTeamHealthPoints(u128);

impl BattleTeamHealthPoints{
    pub fn new(value: u128) -> Self{
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }

    // TODO not sure this goes here, probably not, on battle_team. Here's just an anemic constructor.
    pub fn apply_damage(self, damage: PhysicalDamage) -> Self {
        match damage {
            PhysicalDamage::AttackDamage(amount) => {
                let amount_to_subtract = min(self.0, amount);
                Self(self.0 - amount_to_subtract)
            },
            PhysicalDamage::ReflectedDamage(amount) => {
                let amount_to_subtract = min(self.0-1, amount);
                Self(self.0 - amount_to_subtract)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::battle::battle_team_attack::BattleTeamAttack;
    use super::*;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let hp = BattleTeamHealthPoints::new(value);
        assert_eq!(value, hp.value());
    }

    #[test]
    fn test_apply_damage() {
        let hp = BattleTeamHealthPoints::new(3000);

        let damage = PhysicalDamage::new_attack_damage(BattleTeamAttack::new(500));
        let hp = hp.apply_damage(damage);
        assert_eq!(hp.value(), 2500);
    }

    #[test]
    fn test_overflow_damage() {
        let hp = BattleTeamHealthPoints::new(3000);
        let hp = hp.apply_damage(
            PhysicalDamage::new_attack_damage(BattleTeamAttack::new(50000))
        );
        assert_eq!(hp.value(), 0);
    }

    #[test]
    fn test_overflow_reflect_damage() {
        let hp = BattleTeamHealthPoints::new(3000);
        let hp = hp.apply_damage(
            PhysicalDamage::new_reflected_damage(BattleTeamAttack::new(50000))
        );
        assert_eq!(hp.value(), 1);
    }
}