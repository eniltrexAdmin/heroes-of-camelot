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