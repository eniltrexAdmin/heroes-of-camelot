use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BattleTeamAttack(u128);

impl BattleTeamAttack{
    pub fn new(value: u128) -> Self{
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }

    pub fn increase(self, value: AttackIncreaseValue) -> Self {
        Self(self.0 + value.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let attack = BattleTeamAttack::new(value);
        assert_eq!(value, attack.value());
    }
}