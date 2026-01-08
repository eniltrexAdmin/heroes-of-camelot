use std::cmp::min;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BattleTeamHealthPoints(u128);

impl BattleTeamHealthPoints{
    pub fn new(value: u128) -> Self{
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }

    pub fn apply_damage(self, amount: u128) -> Self {
        let amount_to_subtract = min(self.0, amount);
        Self(self.0 - amount_to_subtract)
    }
}


#[cfg(test)]
mod tests {
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
        let hp = hp.apply_damage(500);
        assert_eq!(hp.value(), 2500);
    }

    #[test]
    fn test_overflow_damage() {
        let hp = BattleTeamHealthPoints::new(3000);
        let hp = hp.apply_damage(50000);
        assert_eq!(hp.value(), 0);
    }

    #[test]
    fn test_overflow_reflect_damage() {
        let hp = BattleTeamHealthPoints::new(3000);
        let hp = hp.apply_damage(50000);
        assert_eq!(hp.value(), 1);
    }
}