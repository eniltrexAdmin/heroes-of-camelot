#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BattleTeamAttack(u128);

impl BattleTeamAttack{
    pub fn new(value: u128) -> Self{
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
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